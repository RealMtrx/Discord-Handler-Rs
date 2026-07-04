use crate::config::Config;
use crate::core::webhooks;
use chrono::Utc;

pub async fn send_leave_guild_webhook(
    config: &Config,
    guild_id: &str,
    guild_name: &str,
    member_count: u64,
    remaining_servers: usize,
) {
    if config.leave_guild_webhook.is_empty() || config.leave_guild_webhook == "#" {
        return;
    }

    let embed = webhooks::WebhookEmbed {
        color: 0xFF0000,
        title: "👋 Bot Left Server".to_string(),
        description: format!("**Server:** {}\n**ID:** {}", guild_name, guild_id),
        fields: vec![
            webhooks::WebhookField {
                name: "👥 Members".to_string(),
                value: format!("{} members", member_count),
                inline: true,
            },
            webhooks::WebhookField {
                name: "📅 Left At".to_string(),
                value: format!("<t:{}:F>", Utc::now().timestamp()),
                inline: true,
            },
            webhooks::WebhookField {
                name: "📊 Remaining Servers".to_string(),
                value: format!("{} servers", remaining_servers),
                inline: true,
            },
        ],
        footer: webhooks::WebhookFooter {
            text: webhooks::footer_text(&config.bot_name, "Guild Leave Logger"),
        },
        timestamp: webhooks::make_timestamp(),
        thumbnail: None,
    };

    let _ = webhooks::send_webhook(&config.leave_guild_webhook, embed).await;
}
