use crate::config::Config;
use crate::core::webhooks;

pub async fn send_ready_webhook(
    config: &Config,
    bot_username: &str,
    bot_id: &str,
    server_count: usize,
) {
    if config.ready_webhook.is_empty() || config.ready_webhook == "#" {
        return;
    }

    let embed = webhooks::WebhookEmbed {
        color: 0x00FF00,
        title: "🟢 Bot is Online!".to_string(),
        description: format!("**Bot:** {}\n**Status:** Online and Ready", bot_username),
        fields: vec![
            webhooks::WebhookField {
                name: "🤖 Bot Info".to_string(),
                value: format!("**ID:** {}", bot_id),
                inline: true,
            },
            webhooks::WebhookField {
                name: "🏠 Servers".to_string(),
                value: format!("{} servers", server_count),
                inline: true,
            },
        ],
        footer: webhooks::WebhookFooter {
            text: webhooks::footer_text(&config.bot_name, "System Logger"),
        },
        timestamp: webhooks::make_timestamp(),
        thumbnail: None,
    };

    let _ = webhooks::send_webhook(&config.ready_webhook, embed).await;
}
