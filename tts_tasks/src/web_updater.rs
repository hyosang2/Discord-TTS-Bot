use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use serenity::all as serenity;

use tts_core::structs::{Result, TTSMode, WebsiteInfo};

#[allow(dead_code, clippy::match_same_arms)]
fn remember_to_update_analytics_query() {
    match TTSMode::gTTS {
        TTSMode::gTTS => (),
        TTSMode::Polly => (),
        TTSMode::eSpeak => (),
        TTSMode::gCloud => (),
        TTSMode::OpenAI => (),
        TTSMode::XTTS => (),
    }
}

fn count_members<'a>(guilds: impl Iterator<Item = serenity::cache::GuildRef<'a>>) -> u64 {
    guilds.map(|g| g.member_count).sum()
}

#[derive(serde::Serialize)]
struct Statistics {
    premium_guild: u32,
    premium_user: u64,
    message: u64,
    guild: u32,
    user: u64,
}

pub struct Updater {
    pub patreon_service: Option<reqwest::Url>,
    pub cache: Arc<serenity::Cache>,
    pub reqwest: reqwest::Client,
    pub config: WebsiteInfo,
    pub pool: sqlx::PgPool,
}

impl crate::Looper for Updater {
    const NAME: &'static str = "WebUpdater";
    const MILLIS: u64 = 1000 * 60 * 60;

    type Error = anyhow::Error;
    async fn loop_func(&self) -> Result<()> {
        #[derive(sqlx::FromRow)]
        struct AnalyticsQueryResult {
            count: i32,
        }

        #[derive(sqlx::FromRow)]
        struct PremiumGuildsQueryResult {
            guild_id: i64,
        }

        let patreon_members = if let Some(mut patreon_service) = self.patreon_service.clone() {
            patreon_service.set_path("members");
            let raw_members: HashMap<i64, serde::de::IgnoredAny> = self
                .reqwest
                .get(patreon_service)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            raw_members.into_keys().collect()
        } else {
            Vec::new()
        };

        let (message_count, premium_guild_ids) = {
            let mut db_conn = self.pool.acquire().await?;
            let message_count = sqlx::query_as::<_, AnalyticsQueryResult>(
                "
                SELECT count FROM analytics
                WHERE date_collected = (CURRENT_DATE - 1) AND (
                    event = 'gTTS_tts'   OR
                    event = 'eSpeak_tts' OR
                    event = 'gCloud_tts' OR
                    event = 'Polly_tts'  OR
                    event = 'OpenAI_tts'
                )
            ",
            )
            .fetch_all(&mut *db_conn)
            .await?
            .into_iter()
            .map(|r| r.count as i64)
            .sum::<i64>();

            let premium_guild_ids = sqlx::query_as::<_, PremiumGuildsQueryResult>(
                "SELECT guild_id FROM guilds WHERE premium_user = ANY($1)",
            )
            .bind(&patreon_members)
            .fetch_all(&mut *db_conn)
            .await?
            .into_iter()
            .map(|g| g.guild_id)
            .collect::<HashSet<_>>();

            (message_count, premium_guild_ids)
        };

        let guild_ids = self.cache.guilds();

        let guild_ref_iter = guild_ids.iter().filter_map(|g| self.cache.guild(*g));
        let user = count_members(guild_ref_iter.clone());

        let premium_guild_ref_iter =
            guild_ref_iter.filter(|g| premium_guild_ids.contains(&(g.id.get() as i64)));
        let premium_user = count_members(premium_guild_ref_iter.clone());
        let premium_guild_count = premium_guild_ref_iter.count();

        let stats = Statistics {
            user,
            premium_user,
            message: message_count as u64,
            guild: guild_ids.len() as u32,
            premium_guild: premium_guild_count as u32,
        };

        let url = {
            let mut url = self.config.url.clone();
            url.set_path("/update_stats");
            url
        };

        self.reqwest
            .post(url)
            .header(AUTHORIZATION, self.config.stats_key.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&stats)?)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
