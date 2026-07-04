use crate::config::Config;
use crate::core::leave_guild_webhook;

pub async fn handle_guild_delete(
    config: &Config,
    guild_id: &str,
    guild_name: &str,
    member_count: u64,
    remaining_servers: usize,
) {
    leave_guild_webhook::send_leave_guild_webhook(
        config,
        guild_id,
        guild_name,
        member_count,
        remaining_servers,
    )
    .await;
}
