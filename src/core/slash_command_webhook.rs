use crate::config::Config;
use crate::core::emojis::Emojis;
use crate::core::webhooks;
use chrono::Utc;

pub async fn send_slash_command_usage(
    config: &Config,
    user_id: &str,
    user_name: &str,
    command_name: &str,
    guild_name: &str,
    avatar_url: Option<&str>,
) {
    if config.slash_command_webhook.is_empty() || config.slash_command_webhook == "#" {
        return;
    }

    let mut embed = webhooks::WebhookEmbed {
        color: 0x5865F2,
        title: format!("{} Slash Command Used", Emojis::SLASH),
        description: format!("**Command:** `/{}`", command_name),
        fields: vec![
            webhooks::WebhookField {
                name: format!("{} User Info", Emojis::USER),
                value: format!("**UserName:** {}\n**ID:** {}", user_name, user_id),
                inline: true,
            },
            webhooks::WebhookField {
                name: format!("{} Server", Emojis::SERVER),
                value: guild_name.to_string(),
                inline: true,
            },
            webhooks::WebhookField {
                name: format!("{} Time", Emojis::LOADING),
                value: format!("<t:{}:R>", Utc::now().timestamp()),
                inline: true,
            },
        ],
        footer: webhooks::WebhookFooter {
            text: webhooks::footer_text(&config.bot_name, "Slash Command Logger"),
        },
        timestamp: webhooks::make_timestamp(),
        thumbnail: avatar_url.map(|url| webhooks::WebhookThumbnail {
            url: url.to_string(),
        }),
    };

    if embed.thumbnail.is_none() {
        embed.thumbnail = Some(webhooks::WebhookThumbnail {
            url: "https://cdn.discordapp.com/embed/avatars/0.png".to_string(),
        });
    }

    let _ = webhooks::send_webhook(&config.slash_command_webhook, embed).await;
}

pub async fn send_slash_command_error(
    config: &Config,
    user_id: &str,
    user_name: &str,
    command_name: &str,
    guild_name: &str,
    error_msg: &str,
) {
    if config.slash_command_webhook.is_empty() || config.slash_command_webhook == "#" {
        return;
    }

    let embed = webhooks::WebhookEmbed {
        color: 0xFF0000,
        title: format!("{} Slash Command Error", Emojis::ERROR),
        description: format!("**Command:** `/{}`\n**Error:** {}", command_name, error_msg),
        fields: vec![
            webhooks::WebhookField {
                name: format!("{} User Info", Emojis::USER),
                value: format!("{} ({})", user_name, user_id),
                inline: true,
            },
            webhooks::WebhookField {
                name: format!("{} Server", Emojis::SERVER),
                value: guild_name.to_string(),
                inline: true,
            },
            webhooks::WebhookField {
                name: format!("{} Time", Emojis::LOADING),
                value: format!("<t:{}:F>", Utc::now().timestamp()),
                inline: true,
            },
        ],
        footer: webhooks::WebhookFooter {
            text: webhooks::footer_text(&config.bot_name, "Error Logger"),
        },
        timestamp: webhooks::make_timestamp(),
        thumbnail: None,
    };

    let _ = webhooks::send_webhook(&config.slash_command_webhook, embed).await;
}
