# Discord Handler Rust

A modern, feature-rich Discord bot handler built with **Serenity**, featuring both slash commands and prefix commands with a robust modular architecture designed for scalability and maintainability.

## 🚀 Features

- **Dual Command System**: Support for both slash commands and prefix commands
- **Modular Architecture**: Clean separation of concerns with dedicated handlers
- **Anti-Crash System**: Panic hooks and comprehensive error handling
- **Event-Driven**: Fully event-driven async architecture
- **Webhook Logging**: Real-time logging for errors and guild events
- **MongoDB Integration**: Persistent data storage with mongodb driver
- **Cooldown System**: Per-command cooldown management
- **Environment Configuration**: Secure configuration via dotenv

## 📁 Project Structure

```
Discord-Handler-Rs/
├── Cargo.toml                    # Rust project configuration and dependencies
├── src/                          # Source code
│   ├── main.rs                   # Main bot entry point
│   ├── config.rs                 # Bot configuration from .env
│   ├── bot.rs                    # Bot initialization
│   ├── Core/                     # Core utilities
│   │   ├── commandUtils.rs       # Cooldown and utilities
│   │   ├── emojis.rs             # Centralized emoji definitions
│   │   └── webhookUtil.rs        # Webhook utility
│   ├── Database/
│   │   └── mongo.rs              # MongoDB connection setup
│   ├── Events/                   # Discord event handlers
│   │   ├── guildCreate.rs        # Handler when bot joins a server
│   │   ├── guildDelete.rs        # Handler when bot leaves a server
│   │   ├── interactionCreate.rs  # Handles slash command interactions
│   │   ├── messageCreate.rs      # Handles prefix commands
│   │   └── ready.rs              # Bot ready event
│   ├── Handlers/                 # Handlers for modularity
│   │   ├── AntiCrash.rs          # Crash prevention and error handling
│   │   └── logger.rs             # Logger for bot activity
│   ├── Models/
│   │   └── userModel.rs          # User data model
│   └── Commands/
│       ├── Prefix/               # Prefix commands
│       │   └── ping.rs           # Example prefix ping command
│       └── Slash/                # Slash commands
│           └── ping.rs           # Example slash ping command
```

## 🔧 Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/RealMtrx/Discord-Handler-Rs.git
   cd Discord-Handler-Rs
   ```

2. **Build dependencies**

   ```bash
   cargo build --release
   ```

3. **Environment Setup**

   Copy `.env.example` to `.env` and fill in your values:

   ```env
   TOKEN=your_bot_token_here
   PREFIX=!
   BOT_NAME=Discord Handler
   MONGO_URI=mongodb://localhost:27017/discord-handler
   ERROR_WEBHOOK=https://discord.com/api/webhooks/your_webhook
   GUILD_LOG_WEBHOOK=https://discord.com/api/webhooks/your_webhook
   ```

4. **Run the bot**

   ```bash
   cargo run --release
   ```

## 📋 Dependencies

- **serenity**: v0.12 - Discord API wrapper
- **tokio**: v1 - Async runtime
- **dotenv**: v0.15 - Environment variable management
- **mongodb**: v2.3 - MongoDB driver
- **reqwest**: v0.11 - HTTP client for webhooks
- **serde_json**: v1 - JSON serialization

## 📝 Command Development

### Creating Slash Commands

Create a new file in `src/Commands/Slash/[category]/[name].rs`:

```rust
use serenity::all::*;
use crate::Handler;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), SerenityError> {
    interaction
        .create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content("Pong! 🏓"),
        ))
        .await?;
    Ok(())
}
```

### Creating Prefix Commands

Create a new file in `src/Commands/Prefix/[category]/[name].rs`:

```rust
use serenity::all::*;

pub async fn run(ctx: &Context, msg: &Message, args: Vec<&str>) -> Result<(), SerenityError> {
    msg.channel_id.say(&ctx.http, "Pong! 🏓").await?;
    Ok(())
}
```

---

**Discord Handler** — Built by **Mtrx** — Discord: **0hu2**
