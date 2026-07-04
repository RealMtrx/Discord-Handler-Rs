use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub token: String,
    pub client_id: String,
    pub bot_name: String,
    pub prefix: String,
    pub owner_ids: Vec<String>,
    pub mongodb_uri: String,
    pub error_webhook: String,
    pub slash_command_webhook: String,
    pub prefix_command_webhook: String,
    pub join_guild_webhook: String,
    pub leave_guild_webhook: String,
    pub ready_webhook: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            token: get_env("TOKEN", "#"),
            client_id: get_env("CLIENT_ID", "#"),
            bot_name: get_env("BOT_NAME", "Discord Handler"),
            prefix: get_env("PREFIX", "$"),
            owner_ids: get_env("OWNER_IDS", "#,#")
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            mongodb_uri: get_env("MONGODB_URI", "#"),
            error_webhook: get_env("ERROR_WEBHOOK", "#"),
            slash_command_webhook: get_env("SLASH_WEBHOOK", "#"),
            prefix_command_webhook: get_env("PREFIX_WEBHOOK", "#"),
            join_guild_webhook: get_env("JOIN_WEBHOOK", "#"),
            leave_guild_webhook: get_env("LEAVE_WEBHOOK", "#"),
            ready_webhook: get_env("READY_WEBHOOK", "#"),
        }
    }
}

fn get_env(key: &str, fallback: &str) -> String {
    env::var(key).unwrap_or_else(|_| fallback.to_string())
}
