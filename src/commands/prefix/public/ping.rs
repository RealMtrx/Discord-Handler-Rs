use serenity::all::{CommandInteraction, Context, CreateCommand, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage, Message};
use std::time::Instant;

use crate::bot::Bot;

pub fn register(bot: &mut Bot) {
    bot.prefix_commands.insert(
        "ping".to_string(),
        crate::bot::PrefixCommand {
            name: "ping".to_string(),
            handler: crate::bot::PrefixCommandHandler::Ping,
        },
    );
}

pub async fn handle(ctx: &Context, msg: &Message, _args: Vec<String>) {
    let sent = msg
        .channel_id
        .say(&ctx.http, "⏳ Loading...")
        .await
        .ok();

    let ws_latency = ctx
        .shard
        .latency()
        .await
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let rest_start = Instant::now();
    let _ = reqwest::get("https://discord.com/api/v10/users/@me").await;
    let rest_latency = rest_start.elapsed().as_millis() as u64;

    let bot_latency = if let Some(sent_msg) = &sent {
        let duration = sent_msg.timestamp.signed_duration_since(msg.timestamp);
        duration.num_milliseconds().max(0) as u64
    } else {
        0
    };

    let embed = CreateEmbed::new()
        .title("🏓 Pong!")
        .color(0x00FF00)
        .field("🤖 Bot Latency", format!("`{}ms`", bot_latency), true)
        .field("📡 WebSocket Latency", format!("`{}ms`", ws_latency), true)
        .field("🌐 REST API Latency", format!("`{}ms`", rest_latency), true);

    if let Some(sent_msg) = sent {
        let _ = sent_msg
            .edit(
                &ctx.http,
                EditMessage::new().content("").embed(embed),
            )
            .await;
    }
}
