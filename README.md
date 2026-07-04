# Discord Handler (Rust)

A modern, feature-rich Discord bot handler built with [Serenity](https://github.com/serenity-rs/serenity) (Rust), featuring both slash commands and prefix commands with a robust modular architecture designed for scalability and maintainability.

## Features

- **Dual Command System**: Support for both slash commands and prefix commands
- **Modular Architecture**: Clean separation of concerns with dedicated handlers
- **Anti-Crash System**: Comprehensive error handling and monitoring via panic hooks
- **Event-Driven**: Fully event-driven architecture
- **Webhook Logging**: Real-time logging for errors, commands, guild events, and bot status
- **MongoDB Integration**: Persistent data storage with the official MongoDB driver
- **Cooldown System**: Per-command cooldown management

## Project Structure

```
Discord-Handler/
├── Cargo.toml                  # Rust project dependencies
├── .env.example                # Environment variables template
├── src/
│   ├── main.rs                 # Main bot entry point
│   ├── config.rs               # Bot configuration (env vars)
│   ├── bot.rs                  # Bot struct, commands registry, event handler
│   ├── core/                   # Core utilities and webhooks
│   │   ├── mod.rs
│   │   ├── emojis.rs           # Centralized emoji definitions
│   │   ├── cooldown.rs         # Per-command cooldown manager
│   │   ├── command_utils.rs    # Error formatting, usage logging
│   │   ├── webhooks.rs         # Base webhook types and sender
│   │   ├── error_webhook.rs    # Error reporting via webhook
│   │   ├── join_guild_webhook.rs
│   │   ├── leave_guild_webhook.rs
│   │   ├── prefix_command_webhook.rs
│   │   ├── ready_webhook.rs
│   │   └── slash_command_webhook.rs
│   ├── database/               # MongoDB connection
│   │   ├── mod.rs
│   │   └── mongo.rs
│   ├── events/                 # Discord event handlers
│   │   ├── mod.rs
│   │   ├── error.rs
│   │   ├── guild_create.rs
│   │   ├── guild_delete.rs
│   │   ├── interaction_create.rs
│   │   ├── message_create.rs
│   │   └── ready.rs
│   ├── handlers/               # Loaders and registrars
│   │   ├── mod.rs
│   │   ├── anticrash.rs        # Panic hook setup
│   │   ├── commands.rs         # Slash command loader
│   │   ├── events.rs           # Event registry
│   │   ├── logger.rs           # Startup report
│   │   ├── models.rs           # Model loader
│   │   └── prefix.rs           # Prefix command loader
│   ├── models/                 # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   └── commands/               # Command implementations
│       ├── mod.rs
│       ├── slash/public/ping.rs
│       └── prefix/public/ping.rs
```

## Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/RealMtrx/Discord-Handler-Rs.git
   cd Discord-Handler-Rs
   ```

2. **Environment Setup**

   Copy `.env.example` to `.env` and fill in your values:

   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your bot token and configuration:

   ```
   TOKEN=your_bot_token
   CLIENT_ID=your_client_id
   BOT_NAME=Discord Handler
   PREFIX=$
   MONGODB_URI=mongodb://localhost:27017/discord_bot
   ERROR_WEBHOOK=your_webhook_url
   SLASH_WEBHOOK=your_webhook_url
   PREFIX_WEBHOOK=your_webhook_url
   JOIN_WEBHOOK=your_webhook_url
   LEAVE_WEBHOOK=your_webhook_url
   READY_WEBHOOK=your_webhook_url
   ```

3. **Build and run**

   ```bash
   cargo build --release
   cargo run --release
   ```

## Dependencies

- **serenity**: Discord API wrapper for Rust
- **tokio**: Async runtime
- **mongodb**: Official MongoDB driver
- **reqwest**: HTTP client for webhooks
- **chrono**: Date and time handling
- **serde / serde_json**: Serialization

## Command Development

### Creating Slash Commands

1. Create a new file in `src/commands/slash/public/` (or a new category folder)
2. Add a variant to `SlashCommandHandler` enum in `src/bot.rs`
3. Implement the `register` function and `handle` function
4. Add the match arm in `src/events/interaction_create.rs`

### Creating Prefix Commands

1. Create a new file in `src/commands/prefix/public/` (or a new category folder)
2. Add a variant to `PrefixCommandHandler` enum in `src/bot.rs`
3. Implement the `register` function and `handle` function
4. Add the match arm in `src/events/message_create.rs`

## License

This project is licensed under the MIT License - see the LICENSE file for details.
