use std::collections::HashMap;

use serenity::all::{
    Command, CommandInteraction, Context, CreateCommand, EventHandler, GuildCreate, GuildDelete,
    Interaction, Message, Ready,
};

use crate::commands::prefix::public as prefix_cmds;
use crate::commands::slash::public as slash_cmds;
use crate::config::Config;
use crate::core::cooldown::CooldownManager;
use crate::events;
use crate::handlers;

pub enum SlashCommandHandler {
    Ping,
}

pub struct SlashCommand {
    pub data: CreateCommand,
    pub handler: SlashCommandHandler,
}

pub enum PrefixCommandHandler {
    Ping,
}

pub struct PrefixCommand {
    pub name: String,
    pub handler: PrefixCommandHandler,
}

pub struct Bot {
    pub config: Config,
    pub slash_commands: HashMap<String, SlashCommand>,
    pub prefix_commands: HashMap<String, PrefixCommand>,
    pub cooldowns: CooldownManager,
}

impl Bot {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            slash_commands: HashMap::new(),
            prefix_commands: HashMap::new(),
            cooldowns: CooldownManager::new(),
        }
    }

    pub fn register_commands(&mut self) {
        slash_cmds::ping::register(self);
        prefix_cmds::ping::register(self);
    }
}

pub struct Handler {
    pub bot: Bot,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let data = &self.bot;
        let config = &data.config;

        let user = match ctx.cache.current_user() {
            Some(u) => u,
            None => return,
        };

        ctx.set_activity(serenity::all::ActivityData::playing(&config.bot_name));

        let command_list: Vec<CreateCommand> = data
            .slash_commands
            .values()
            .map(|cmd| cmd.data.clone())
            .collect();

        if !command_list.is_empty() {
            match Command::set_global_commands(&ctx.http, command_list).await {
                Ok(registered) => {
                    println!(
                        "  ✅ [Slash] Registered {} commands with Discord API",
                        registered.len()
                    );
                }
                Err(e) => {
                    println!("  ❌ [System] Failed to register commands: {}", e);
                }
            }
        }

        let server_count = ctx.cache.guilds().len();
        events::ready::handle_ready(&ctx, config, &user.name, &user.id.to_string(), server_count)
            .await;

        handlers::logger::startup_report(&handlers::logger::StartupData {
            name: config.bot_name.clone(),
            slash_count: data.slash_commands.len(),
            prefix_count: data.prefix_commands.len(),
            events_count: 6,
            models_count: 1,
            mongo_status: false,
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::interaction_create::handle_interaction_create(
            &ctx,
            &interaction,
            &self.bot,
            &self.bot.cooldowns,
        )
        .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        events::message_create::handle_message_create(
            &ctx,
            &msg,
            &self.bot,
            &self.bot.cooldowns,
        )
        .await;
    }

    async fn guild_create(&self, ctx: Context, guild: GuildCreate) {
        let guild_data = &guild.guild;
        let icon_url = guild_data.icon_url();
        events::guild_create::handle_guild_create(
            &self.bot.config,
            &guild_data.name,
            &guild_data.id.to_string(),
            &guild_data.owner_id.to_string(),
            guild_data.member_count as u64,
            icon_url.as_deref(),
        )
        .await;
    }

    async fn guild_delete(&self, ctx: Context, guild: GuildDelete) {
        let guild_id = guild.id.to_string();
        let guild_name = guild
            .id
            .to_guild_cached(&ctx.cache)
            .map(|g| g.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        let remaining = ctx.cache.guilds().len();
        events::guild_delete::handle_guild_delete(
            &self.bot.config,
            &guild_id,
            &guild_name,
            0,
            remaining,
        )
        .await;
    }
}
