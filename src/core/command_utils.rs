use chrono::Utc;

#[derive(Debug, Clone)]
pub struct ErrorReport {
    pub message: String,
    pub command_name: String,
    pub timestamp: String,
}

pub fn format_error(error: &str, command_name: &str) -> ErrorReport {
    ErrorReport {
        message: error.to_string(),
        command_name: command_name.to_string(),
        timestamp: Utc::now().to_rfc3339(),
    }
}

pub fn log_command_usage(user_id: &str, user_name: &str, command_name: &str, guild_name: &str) {
    println!(
        "[Command] {} ({}) used {} in {}",
        user_name, user_id, command_name, guild_name
    );
}
