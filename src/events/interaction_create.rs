use serenity::all::{CommandInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, Interaction};

use crate::bot::{Bot, SlashCommandHandler};
use crate::commands::slash::public::ping;
use crate::core::cooldown::CooldownManager;
use crate::core::slash_command_webhook;

pub async fn handle_interaction_create(
    ctx: &Context,
    interaction: &Interaction,
    bot: &Bot,
    cooldowns: &CooldownManager,
) {
    let Interaction::Command(command) = interaction else {
        return;
    };

    let cmd_name = &command.data.name;
    let cmd = match bot.slash_commands.get(cmd_name) {
        Some(c) => c,
        None => return,
    };

    let user = &command.user;

    let (on_cd, remaining) = cooldowns.check(&user.id.to_string(), cmd_name, 3000).await;
    if on_cd {
        let _ = command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(format!(
                            "⏰ Please wait {} seconds before using this command again.",
                            remaining
                        ))
                        .ephemeral(true),
                ),
            )
            .await;
        return;
    }

    let guild_name = command
        .guild_id
        .and_then(|gid| gid.to_guild_cached(&ctx.cache))
        .map(|g| g.name.clone())
        .unwrap_or_else(|| "Direct Message".to_string());

    let avatar = user.avatar_url().or_else(|| {
        Some(format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            user.discriminator as u64 % 5
        ))
    });

    slash_command_webhook::send_slash_command_usage(
        &bot.config,
        &user.id.to_string(),
        &user.name,
        cmd_name,
        &guild_name,
        avatar.as_deref(),
    )
    .await;

    match &cmd.handler {
        SlashCommandHandler::Ping => ping::handle(ctx, command).await,
    }
}
