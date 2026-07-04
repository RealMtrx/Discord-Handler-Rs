use crate::config::Config;
use crate::core::join_guild_webhook;

pub async fn handle_guild_create(
    config: &Config,
    guild_name: &str,
    guild_id: &str,
    owner_id: &str,
    member_count: u64,
    icon_url: Option<&str>,
) {
    join_guild_webhook::send_join_guild_webhook(
        config,
        guild_name,
        guild_id,
        owner_id,
        member_count,
        icon_url,
    )
    .await;
}
