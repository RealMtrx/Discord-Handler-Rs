use crate::config::Config;
use crate::core::webhooks;
use chrono::Utc;

pub async fn send_join_guild_webhook(
    config: &Config,
    guild_name: &str,
    guild_id: &str,
    owner_id: &str,
    member_count: u64,
    icon_url: Option<&str>,
) {
    if config.join_guild_webhook.is_empty() || config.join_guild_webhook == "#" {
        return;
    }

    let mut embed = webhooks::WebhookEmbed {
        color: 0x57F287,
        title: "🎉 Bot Joined New Server!".to_string(),
        description: format!("**Server:** {}\n**ID:** {}", guild_name, guild_id),
        fields: vec![
            webhooks::WebhookField {
                name: "👑 Owner".to_string(),
                value: format!("<@{}>", owner_id),
                inline: true,
            },
            webhooks::WebhookField {
                name: "👥 Members".to_string(),
                value: format!("{} members", member_count),
                inline: true,
            },
            webhooks::WebhookField {
                name: "📅 Joined At".to_string(),
                value: format!("<t:{}:F>", Utc::now().timestamp()),
                inline: true,
            },
        ],
        footer: webhooks::WebhookFooter {
            text: webhooks::footer_text(&config.bot_name, "Guild Join Logger"),
        },
        timestamp: webhooks::make_timestamp(),
        thumbnail: icon_url.map(|url| webhooks::WebhookThumbnail {
            url: url.to_string(),
        }),
    };

    if embed.thumbnail.is_none() {
        embed.thumbnail = Some(webhooks::WebhookThumbnail {
            url: "https://cdn.discordapp.com/embed/avatars/0.png".to_string(),
        });
    }

    let _ = webhooks::send_webhook(&config.join_guild_webhook, embed).await;
}
