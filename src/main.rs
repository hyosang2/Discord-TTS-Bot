
use std::{
    collections::BTreeMap,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use anyhow::Ok;
use parking_lot::Mutex;

use poise::serenity_prelude as serenity;
use serenity::small_fixed_array::FixedString;

use tts_core::{
    analytics, create_db_handler, database,
    structs::{Data, RegexCache, Result},
};
use tts_events::EventHandler;
use tts_tasks::{logging::Layer, Looper as _};

mod startup;

use startup::*;

#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<()> {
    let start_time = std::time::SystemTime::now();
    let console_layer = console_subscriber::spawn();

    println!("Starting tokio runtime");
    unsafe {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(main_(console_layer, start_time))
}

async fn main_(console_layer: impl Layer, start_time: std::time::SystemTime) -> Result<()> {
    println!("Loading and performing migrations");
    let (pool, config) = tts_migrations::load_db_and_conf().await?;

    println!("Initialising Http client");
    let reqwest = reqwest::Client::new();
    let auth_key = config.main.tts_service_auth_key.as_deref();

    let token = config.main.token.clone();
    let mut http_builder = serenity::HttpBuilder::new(token.clone());
    if let Some(proxy) = &config.main.proxy_url {
        println!("Connecting via proxy");
        http_builder = http_builder
            .proxy(proxy.as_str())
            .ratelimiter_disabled(true);
    }

    let http = Arc::new(http_builder.build());

    println!("Performing big startup join");
    let tts_service = || config.main.tts_service.clone();
    let (
        webhooks,
        guilds_db,
        userinfo_db,
        user_voice_db,
        guild_voice_db,
        nickname_db,
        user_opt_out_db,
        gtts_voices,
        espeak_voices,
        gcloud_voices,
        polly_voices,
        translation_languages,
        shard_count,
    ) = tokio::try_join!(
        get_webhooks(&http, config.webhooks),
        create_db_handler!(pool.clone(), "guilds", "guild_id"),
        create_db_handler!(pool.clone(), "userinfo", "user_id"),
        create_db_handler!(pool.clone(), "user_voice", "user_id", "mode"),
        create_db_handler!(pool.clone(), "guild_voice", "guild_id", "mode"),
        create_db_handler!(pool.clone(), "nicknames", "guild_id", "user_id"),
        create_db_handler!(pool.clone(), "user_opt_out", "user_id", "guild_id"),
        fetch_voices_safe_gtts(&reqwest, tts_service(), auth_key),
        fetch_voices_safe_espeak(&reqwest, tts_service(), auth_key),
        fetch_voices_safe_gcloud(&reqwest, tts_service(), auth_key),
        fetch_voices_safe_polly(&reqwest, tts_service(), auth_key),
        fetch_translation_languages(&reqwest, tts_service(), auth_key),
        async { Ok(http.get_bot_gateway().await?.shards) },
    )?;

    // Load premium user separately (failure is okay for self-hosted)
    let premium_user = serenity::UserId::new(802632257658683442)
        .to_user(&http)
        .await;
    
    match &premium_user {
        std::result::Result::Ok(_) => println!("Loaded premium user"),
        std::result::Result::Err(e) => println!("Failed to load premium user (this is okay for self-hosted): {}", e),
    }

    println!("Setting up webhook logging");
    tts_tasks::logging::WebhookLogger::init(
        console_layer,
        http.clone(),
        webhooks.logs.clone(),
        webhooks.errors.clone(),
    );

    println!("Sending startup message");
    let startup_message = send_startup_message(&http, &webhooks.logs).await?;

    println!("Spawning analytics handler");
    let analytics = Arc::new(analytics::Handler::new(pool.clone()));
    tokio::spawn(analytics.clone().start());

    let data = Arc::new(Data {
        pool,
        system_info: Mutex::new(sysinfo::System::new()),
        bot_list_tokens: Mutex::new(config.bot_list_tokens),

        fully_started: AtomicBool::new(false),
        join_vc_tokens: dashmap::DashMap::new(),
        songbird: songbird::Songbird::serenity(),
        last_to_xsaid_tracker: dashmap::DashMap::new(),
        update_startup_lock: tokio::sync::Mutex::new(()),
        entitlement_cache: mini_moka::sync::Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60))
            .build(),

        gtts_voices,
        espeak_voices,
        translation_languages,
        gcloud_voices: prepare_gcloud_voices(gcloud_voices),
        polly_voices: polly_voices
            .into_iter()
            .map(|v| (v.id.clone(), v))
            .collect::<BTreeMap<_, _>>(),

        config: config.main,
        premium_config: config.premium,
        website_info: Mutex::new(config.website_info),
        reqwest,
        premium_avatar_url: FixedString::from_string_trunc(
            premium_user.as_ref().map(|u| u.face()).unwrap_or_else(|_| "".to_string())
        ),
        analytics,
        webhooks,
        start_time,
        startup_message,
        regex_cache: RegexCache::new()?,
        guilds_db,
        userinfo_db,
        nickname_db,
        user_voice_db,
        guild_voice_db,
        user_opt_out_db,
    });

    let framework_options = poise::FrameworkOptions {
        commands: tts_commands::commands(),
        on_error: |error| {
            Box::pin(async move {
                let res = tts_core::errors::handle(error).await;
                res.unwrap_or_else(|err| tracing::error!("on_error: {:?}", err));
            })
        },
        allowed_mentions: Some(
            serenity::CreateAllowedMentions::default()
                .replied_user(true)
                .all_users(true),
        ),
        pre_command: analytics::pre_command,
        prefix_options: poise::PrefixFrameworkOptions {
            dynamic_prefix: Some(|ctx| Box::pin(tts_commands::get_prefix(ctx))),
            ..poise::PrefixFrameworkOptions::default()
        },
        command_check: Some(|ctx| Box::pin(tts_commands::command_check(ctx))),
        ..poise::FrameworkOptions::default()
    };

    let mut client = serenity::ClientBuilder::new_with_http(token, http, tts_events::get_intents())
        .voice_manager::<songbird::Songbird>(data.songbird.clone())
        .framework(poise::Framework::new(framework_options))
        .event_handler::<EventHandler>(EventHandler)
        .data(data as _)
        .await?;

    let shutdown_trigger = client.shard_manager.get_shutdown_trigger();
    tokio::spawn(async move {
        wait_until_shutdown().await;

        tracing::warn!("Recieved control C and shutting down.");
        shutdown_trigger();
    });

    client
        .start_shards(shard_count.get())
        .await
        .map_err(Into::into)
}

#[cfg(unix)]
async fn wait_until_shutdown() {
    use tokio::signal::unix as signal;

    let [mut s1, mut s2, mut s3] = [
        signal::signal(signal::SignalKind::hangup()).unwrap(),
        signal::signal(signal::SignalKind::interrupt()).unwrap(),
        signal::signal(signal::SignalKind::terminate()).unwrap(),
    ];

    tokio::select!(
        v = s1.recv() => v.unwrap(),
        v = s2.recv() => v.unwrap(),
        v = s3.recv() => v.unwrap(),
    );
}

#[cfg(windows)]
async fn wait_until_shutdown() {
    let (mut s1, mut s2) = (
        tokio::signal::windows::ctrl_c().unwrap(),
        tokio::signal::windows::ctrl_break().unwrap(),
    );

    tokio::select!(
        v = s1.recv() => v.unwrap(),
        v = s2.recv() => v.unwrap(),
    );
}
