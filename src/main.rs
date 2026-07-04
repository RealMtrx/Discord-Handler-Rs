mod bot;
mod commands;
mod config;
mod core;
mod database;
mod events;
mod handlers;
mod models;

use serenity::all::GatewayIntents;

use crate::bot::{Bot, Handler};
use crate::config::Config;
use crate::handlers;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    handlers::anticrash::setup_anticrash();

    println!("╔══════════════════════════════════╗");
    println!("║     Starting Discord Handler     ║");
    println!("╚══════════════════════════════════╝");
    println!();

    let config = Config::load();

    println!("  ℹ️ [System] Bot Name: {}", config.bot_name);
    println!("  ℹ️ [System] Prefix: {}", config.prefix);
    println!();

    println!("  📋 [System] Initializing AntiCrash...");
    println!("  ✅ [System] AntiCrash active");
    println!();

    println!("  📋 [System] Connecting to MongoDB...");
    let _mongo_db = database::mongo::connect(&config.mongodb_uri).await;
    println!();

    let mut bot = Bot::new(config);
    bot.register_commands();

    println!("  📋 [System] Loading slash commands...");
    let _slash_count = handlers::commands::load_slash_commands(&bot).await;

    println!("  📋 [System] Loading prefix commands...");
    let _prefix_count = handlers::prefix::load_prefix_commands(&bot);

    println!("  📋 [System] Registering events...");
    let _events_count = handlers::events::register_events();

    println!("  📋 [System] Loading models...");
    let _models_count = handlers::models::load_models();
    println!();

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler { bot };

    let mut client = serenity::all::Client::builder(
        &handler.bot.config.token,
        intents,
    )
    .event_handler(handler)
    .await
    .expect("Failed to create client");

    println!("  📋 [System] Starting bot...");

    if let Err(e) = client.start().await {
        eprintln!("  ❌ [System] Failed to start: {}", e);
    }
}
