use crate::config::Config;
use crate::core::ready_webhook;

pub async fn handle_ready(
    config: &Config,
    bot_username: &str,
    bot_id: &str,
    server_count: usize,
) {
    ready_webhook::send_ready_webhook(config, bot_username, bot_id, server_count).await;
}
