use serenity::all::{Context, Message};

use crate::bot::{Bot, PrefixCommandHandler};
use crate::commands::prefix::public::ping;
use crate::core::cooldown::CooldownManager;
use crate::core::prefix_command_webhook;

pub async fn handle_message_create(
    ctx: &Context,
    msg: &Message,
    bot: &Bot,
    cooldowns: &CooldownManager,
) {
    if msg.author.bot || msg.guild_id.is_none() {
        return;
    }

    let prefix = &bot.config.prefix;
    let content = &msg.content;

    if !content.starts_with(prefix) {
        return;
    }

    let args: Vec<&str> = content[prefix.len()..].split_whitespace().collect();
    if args.is_empty() {
        return;
    }

    let cmd_name = args[0].to_lowercase();
    let cmd = match bot.prefix_commands.get(&cmd_name) {
        Some(c) => c,
        None => return,
    };

    let (on_cd, remaining) = cooldowns
        .check(&msg.author.id.to_string(), &cmd_name, 3000)
        .await;
    if on_cd {
        let _ = msg
            .channel_id
            .say(
                &ctx.http,
                format!(
                    "⏰ Please wait {} seconds before using this command again.",
                    remaining
                ),
            )
            .await;
        return;
    }

    let guild_name = msg
        .guild_id
        .and_then(|gid| gid.to_guild_cached(&ctx.cache))
        .map(|g| g.name.clone())
        .unwrap_or_else(|| "Direct Message".to_string());

    let avatar = msg.author.avatar_url().or_else(|| {
        Some(format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            msg.author.discriminator as u64 % 5
        ))
    });

    prefix_command_webhook::send_prefix_command_usage(
        &bot.config,
        &msg.author.id.to_string(),
        &msg.author.name,
        &cmd_name,
        &guild_name,
        avatar.as_deref(),
    )
    .await;

    let cmd_args: Vec<String> = args[1..].iter().map(|s| s.to_string()).collect();

    match &cmd.handler {
        PrefixCommandHandler::Ping => ping::handle(ctx, msg, cmd_args).await,
    }
}
