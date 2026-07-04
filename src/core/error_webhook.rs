use crate::config::Config;
use crate::core::webhooks;

pub async fn send_error_webhook(config: &Config, error_msg: &str) {
    if config.error_webhook.is_empty() || config.error_webhook == "#" {
        return;
    }

    let embed = webhooks::WebhookEmbed {
        color: 0xFF0000,
        title: "❌ Bot Error Report".to_string(),
        description: format!("**Error:** {}", error_msg),
        fields: vec![webhooks::WebhookField {
            name: "📅 Timestamp".to_string(),
            value: webhooks::make_timestamp(),
            inline: true,
        }],
        footer: webhooks::WebhookFooter {
            text: webhooks::footer_text(&config.bot_name, "Error Logger"),
        },
        timestamp: webhooks::make_timestamp(),
        thumbnail: None,
    };

    let _ = webhooks::send_webhook(&config.error_webhook, embed).await;
}
