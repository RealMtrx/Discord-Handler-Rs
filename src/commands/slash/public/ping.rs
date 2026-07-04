use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use std::time::Instant;

use crate::bot::Bot;

pub fn register(bot: &mut Bot) {
    bot.slash_commands.insert(
        "ping".to_string(),
        crate::bot::SlashCommand {
            data: CreateCommand::new("ping").description("Show bot latency and API response times"),
            handler: crate::bot::SlashCommandHandler::Ping,
        },
    );
}

pub async fn handle(ctx: &Context, command: &CommandInteraction) {
    let start = Instant::now();

    let _ = command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new()),
        )
        .await;

    let bot_latency = start.elapsed().as_millis() as u64;

    let ws_latency = ctx
        .shard
        .latency()
        .await
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let rest_start = Instant::now();
    let _ = reqwest::get("https://discord.com/api/v10/users/@me").await;
    let rest_latency = rest_start.elapsed().as_millis() as u64;

    let embed = CreateEmbed::new()
        .title("🏓 Pong!")
        .color(0x00FF00)
        .field("🤖 Bot Latency", format!("`{}ms`", bot_latency), true)
        .field("📡 WebSocket Latency", format!("`{}ms`", ws_latency), true)
        .field("🌐 REST API Latency", format!("`{}ms`", rest_latency), true);

    let _ = command
        .edit_response(
            &ctx.http,
            CreateInteractionResponseMessage::new().embed(embed),
        )
        .await;
}
