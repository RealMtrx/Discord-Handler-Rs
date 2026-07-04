use chrono::Utc;
use reqwest::Client;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WebhookEmbed {
    pub title: String,
    pub description: String,
    pub color: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<WebhookField>,
    pub footer: WebhookFooter,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<WebhookThumbnail>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookField {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookFooter {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct WebhookThumbnail {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
struct WebhookPayload {
    embeds: Vec<WebhookEmbed>,
}

pub async fn send_webhook(url: &str, embed: WebhookEmbed) -> Result<(), String> {
    if url.is_empty() || url == "#" {
        return Ok(());
    }

    let payload = WebhookPayload {
        embeds: vec![embed],
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .post(url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_client_error() || resp.status().is_server_error() {
        return Err(format!("webhook returned status {}", resp.status()));
    }

    Ok(())
}

pub fn make_timestamp() -> String {
    Utc::now().to_rfc3339()
}

pub fn footer_text(bot_name: &str, suffix: &str) -> String {
    format!("{} • {}", bot_name, suffix)
}
