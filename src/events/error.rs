use crate::config::Config;
use crate::core::error_webhook;

pub async fn handle_error(config: &Config, error_msg: &str) {
    error_webhook::send_error_webhook(config, error_msg).await;
}
