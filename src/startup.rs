use std::collections::BTreeMap;

use small_fixed_array::{FixedArray, FixedString, TruncatingInto as _};

use poise::serenity_prelude as serenity;

use tts_core::{
    opt_ext::OptionTryUnwrap as _,
    structs::{GoogleGender, GoogleVoice, PollyVoice, Result, TTSMode, WebhookConfig, WebhookConfigRaw},
    xtts,
};

pub async fn get_webhooks(
    http: &serenity::Http,
    webhooks_raw: WebhookConfigRaw,
) -> Result<WebhookConfig> {
    let get_webhook = |url: Option<reqwest::Url>| async move {
        if let Some(url) = url {
            let (webhook_id, _) = serenity::parse_webhook(&url).try_unwrap()?;
            anyhow::Ok(Some(webhook_id.to_webhook(http).await?))
        } else {
            anyhow::Ok(None)
        }
    };

    let (logs, errors, dm_logs) = tokio::try_join!(
        get_webhook(webhooks_raw.logs),
        get_webhook(webhooks_raw.errors),
        get_webhook(webhooks_raw.dm_logs),
    )?;

    println!("Fetched webhooks");
    Ok(WebhookConfig {
        logs,
        errors,
        dm_logs,
    })
}

async fn fetch_json<T>(reqwest: &reqwest::Client, url: reqwest::Url, auth_header: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let resp = reqwest
        .get(url)
        .header("Authorization", auth_header)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp)
}

pub async fn fetch_voices<T: serde::de::DeserializeOwned>(
    reqwest: &reqwest::Client,
    mut tts_service: reqwest::Url,
    auth_key: Option<&str>,
    mode: TTSMode,
) -> Result<T> {
    tts_service.set_path("voices");
    tts_service
        .query_pairs_mut()
        .append_pair("mode", mode.into())
        .append_pair("raw", "true")
        .finish();

    let res = fetch_json(reqwest, tts_service, auth_key.unwrap_or("")).await?;

    println!("Loaded voices for TTS Mode: {mode}");
    Ok(res)
}

pub async fn fetch_translation_languages(
    reqwest: &reqwest::Client,
    mut tts_service: reqwest::Url,
    auth_key: Option<&str>,
) -> Result<BTreeMap<FixedString<u8>, FixedString<u8>>> {
    tts_service.set_path("translation_languages");

    match fetch_json::<Vec<(String, FixedString<u8>)>>(reqwest, tts_service, auth_key.unwrap_or("")).await {
        Ok(raw_langs) => {
            let lang_map = raw_langs.into_iter().map(|(mut lang, name)| {
                lang.make_ascii_lowercase();
                (lang.trunc_into(), name)
            });

            println!("Loaded DeepL translation languages");
            Ok(lang_map.collect())
        }
        Err(e) => {
            eprintln!("Failed to fetch translation languages: {e}");
            println!("Using empty translation languages list");
            Ok(BTreeMap::new())
        }
    }
}

// Wrapper functions that provide fallback values when TTS service is unavailable
pub async fn fetch_voices_safe_gtts(
    reqwest: &reqwest::Client,
    tts_service: reqwest::Url,
    auth_key: Option<&str>,
) -> Result<BTreeMap<FixedString<u8>, FixedString<u8>>> {
    match fetch_voices(reqwest, tts_service, auth_key, TTSMode::gTTS).await {
        Ok(voices) => Ok(voices),
        Err(e) => {
            eprintln!("Failed to fetch gTTS voices: {e}");
            println!("Using empty gTTS voice list");
            Ok(BTreeMap::new())
        }
    }
}

pub async fn fetch_voices_safe_espeak(
    reqwest: &reqwest::Client,
    tts_service: reqwest::Url,
    auth_key: Option<&str>,
) -> Result<FixedArray<FixedString<u8>>> {
    match fetch_voices(reqwest, tts_service, auth_key, TTSMode::eSpeak).await {
        Ok(voices) => Ok(voices),
        Err(e) => {
            eprintln!("Failed to fetch eSpeak voices: {e}");
            println!("Using empty eSpeak voice list");
            Ok(FixedArray::new())
        }
    }
}

pub async fn fetch_voices_safe_gcloud(
    reqwest: &reqwest::Client,
    tts_service: reqwest::Url,
    auth_key: Option<&str>,
) -> Result<Vec<GoogleVoice>> {
    match fetch_voices(reqwest, tts_service, auth_key, TTSMode::gCloud).await {
        Ok(voices) => Ok(voices),
        Err(e) => {
            eprintln!("Failed to fetch gCloud voices: {e}");
            println!("Using empty gCloud voice list");
            Ok(Vec::new())
        }
    }
}

pub async fn fetch_voices_safe_polly(
    reqwest: &reqwest::Client,
    tts_service: reqwest::Url,
    auth_key: Option<&str>,
) -> Result<Vec<PollyVoice>> {
    match fetch_voices(reqwest, tts_service, auth_key, TTSMode::Polly).await {
        Ok(voices) => Ok(voices),
        Err(e) => {
            eprintln!("Failed to fetch Polly voices: {e}");
            println!("Using empty Polly voice list");
            Ok(Vec::new())
        }
    }
}

pub fn prepare_gcloud_voices(
    raw_map: Vec<GoogleVoice>,
) -> BTreeMap<FixedString<u8>, BTreeMap<FixedString<u8>, GoogleGender>> {
    // {lang_accent: {variant: gender}}
    let mut cleaned_map = BTreeMap::new();
    for gvoice in raw_map {
        let variant = gvoice
            .name
            .splitn(3, '-')
            .nth(2)
            .and_then(|mode_variant| mode_variant.split_once('-'))
            .filter(|(mode, _)| *mode == "Standard")
            .map(|(_, variant)| variant);

        if let Some(variant) = variant {
            let [language] = gvoice.language_codes;
            cleaned_map
                .entry(language)
                .or_insert_with(BTreeMap::new)
                .insert(FixedString::from_str_trunc(variant), gvoice.ssml_gender);
        }
    }

    cleaned_map
}

pub async fn init_xtts_voice_cache_safe() -> Result<xtts::VoiceCache> {
    let voice_clips_path = std::path::Path::new("./xtts_voice_clips");
    match xtts::init_voice_cache(voice_clips_path).await {
        Ok(cache) => {
            println!("Initialized XTTS voice cache");
            Ok(cache)
        }
        Err(e) => {
            eprintln!("Failed to initialize XTTS voice cache: {e}");
            println!("Using empty XTTS voice cache");
            // Create empty cache as fallback
            Ok(std::sync::Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())))
        }
    }
}

pub async fn send_startup_message(
    http: &serenity::Http,
    log_webhook: &Option<serenity::Webhook>,
) -> Result<Option<serenity::MessageId>> {
    if let Some(webhook) = log_webhook {
        let startup_builder = serenity::ExecuteWebhook::default().content("**TTS Bot is starting up**");
        let startup_message = webhook.execute(http, true, startup_builder).await?;
        Ok(Some(startup_message.unwrap().id))
    } else {
        println!("No log webhook configured, skipping startup message");
        Ok(None)
    }
}
