<div align="center">
  <h1>Discord Handler — Rust</h1>
  <p><strong>A production-ready Discord bot framework built with Serenity and MongoDB — slash commands, prefix commands, anti-crash, webhook logging, and a modular <code>src/</code> architecture.</strong></p>

  <p>
    <a href="https://github.com/RealMtrx/Discord-Handler-Rs/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
    <a href="https://github.com/RealMtrx/Discord-Handler-Rs/releases"><img src="https://img.shields.io/badge/version-0.9.0--beta-yellow" alt="Version 0.9.0 Beta"></a>
    <a href="https://github.com/RealMtrx/Discord-Handler-Rs/stargazers"><img src="https://img.shields.io/github/stars/RealMtrx/Discord-Handler-Rs" alt="Stars"></a>
    <a href="https://github.com/RealMtrx/Discord-Handler-Rs/issues"><img src="https://img.shields.io/github/issues/RealMtrx/Discord-Handler-Rs" alt="Issues"></a>
    <a href="https://github.com/RealMtrx/Discord-Handler-Rs/network"><img src="https://img.shields.io/github/forks/RealMtrx/Discord-Handler-Rs" alt="Forks"></a>
    <a href="https://github.com/RealMtrx/Discord-Handler/graphs/contributors"><img src="https://img.shields.io/badge/ecosystem-26%20repos-brightgreen" alt="26 Repos"></a>
    <a href="https://discord.gg/0hu2"><img src="https://img.shields.io/badge/discord-0hu2-5865F2" alt="Discord"></a>
  </p>

  <br>

  <p>
    <a href="#-features">Features</a> •
    <a href="#-quick-start">Quick Start</a> •
    <a href="#-project-structure">Structure</a> •
    <a href="#-api-reference">API</a> •
    <a href="#-database-edition">SQL Edition</a> •
    <a href="#-related-repositories">Ecosystem</a>
  </p>
</div>

---

## Overview

Discord Handler Rust is a production-ready Discord bot framework built on **Serenity** with **MongoDB** storage. It provides a complete foundation for building Discord bots with slash commands, prefix commands, event handling, anti-crash protection, and webhook-based logging — all organized in a clean, scalable `src/` directory structure.

> **Version:** 0.9.0 (Stable Beta) — Part of the [Discord Handler](https://github.com/RealMtrx/Discord-Handler) ecosystem (26 repos across 13 languages).

## Features

- **Dual Command System** — Slash commands and prefix commands with Serenity's type-safe API
- **MongoDB Integration** — Persistent data storage with the official Rust MongoDB driver (v3, tokio-runtime)
- **Modular Architecture** — Clean separation: Commands, Events, Handlers, Core, Database, Models
- **Anti-Crash Protection** — Panic hooks with webhook notification via `std::panic::set_hook`
- **Async Runtime** — Fully async event-driven architecture powered by Tokio (multi-threaded)
- **Webhook Logging** — Dedicated webhooks for errors, slash/prefix commands, guild joins/leaves, and ready events using reqwest
- **Cooldown System** — Per-command rate limiting with `CooldownManager` using `tokio::sync::Mutex` and spawn-based cleanup
- **Emoji System** — Centralized emoji constants via `Emojis` struct for consistent rendering
- **Environment Configuration** — Secure token and secrets management via `dotenvy`
- **Startup Report** — Terminal banner showing loaded commands, events, and connection status
- **Graceful Shutdown** — Clean connection teardown on exit

## Quick Start

```bash
# Clone the repository
git clone https://github.com/RealMtrx/Discord-Handler-Rs.git
cd Discord-Handler-Rs

# Build dependencies
cargo build --release

# Configure environment
cp .env.example .env
# Edit .env with your bot token, client ID, and MongoDB URI

# Run the bot
cargo run --release
```

### Prerequisites

- **Rust 2021+** — Stable toolchain
- **MongoDB** — Local or Atlas instance
- **Discord Application** — Bot token and client ID from the [Discord Developer Portal](https://discord.com/developers/applications)

### Environment Variables

```env
TOKEN=your_bot_token
CLIENT_ID=your_client_id
BOT_NAME=Discord Handler
OWNER_IDS=owner_id_1,owner_id_2
PREFIX=$
MONGODB_URI=mongodb://localhost:27017/discord_bot
ERROR_WEBHOOK=your_webhook_url
SLASH_WEBHOOK=your_webhook_url
PREFIX_WEBHOOK=your_webhook_url
JOIN_WEBHOOK=your_webhook_url
LEAVE_WEBHOOK=your_webhook_url
READY_WEBHOOK=your_webhook_url
```

## Project Structure

```
Discord-Handler-Rs/
├── Cargo.toml                     # Project manifest and dependencies
├── .env.example                   # Environment template
├── LICENSE
├── src/
│   ├── main.rs                    # Entry point — initializes everything
│   ├── config.rs                  # Config struct loaded from env vars
│   ├── bot.rs                     # Bot, SlashCommand, PrefixCommand, EventHandler impl
│   ├── Core/                      # Shared utilities
│   │   ├── mod.rs
│   │   ├── command_utils.rs       # ErrorReport, format_error, log_command_usage
│   │   ├── cooldown.rs            # CooldownManager with async cleanup
│   │   ├── emojis.rs              # Unicode emoji constants
│   │   ├── error_webhook.rs       # Error reporting webhook
│   │   ├── join_guild_webhook.rs  # Guild join notification
│   │   ├── leave_guild_webhook.rs # Guild leave notification
│   │   ├── prefix_command_webhook.rs
│   │   ├── ready_webhook.rs       # Bot ready event webhook
│   │   ├── slash_command_webhook.rs
│   │   └── webhooks.rs            # WebhookEmbed, send_webhook, shared helpers
│   ├── Database/
│   │   ├── mod.rs
│   │   └── mongo.rs               # MongoDB connection, ping, disconnect
│   ├── Events/                    # Discord event handlers
│   │   ├── mod.rs
│   │   ├── error.rs               # Discord error handler
│   │   ├── guild_create.rs        # Guild join webhook
│   │   ├── guild_delete.rs        # Guild leave webhook
│   │   ├── interaction_create.rs  # Slash command dispatch + cooldown
│   │   ├── message_create.rs      # Prefix command dispatch + cooldown
│   │   └── ready.rs               # Bot ready — sets activity, registers commands
│   ├── Handlers/                  # Loaders and registrars
│   │   ├── mod.rs
│   │   ├── anticrash.rs           # set_hook panic handler
│   │   ├── commands.rs            # load_slash_commands
│   │   ├── events.rs              # register_events
│   │   ├── logger.rs              # startup_report
│   │   ├── models.rs              # load_models
│   │   └── prefix.rs              # load_prefix_commands
│   ├── Models/
│   │   ├── mod.rs
│   │   └── user.rs                # User struct, get_user, create_user
│   └── Commands/
│       ├── mod.rs
│       ├── Slash/
│       │   ├── mod.rs
│       │   └── public/
│       │       ├── mod.rs
│       │       └── ping.rs        # Shows bot, WebSocket, and REST latency
│       └── Prefix/
│           ├── mod.rs
│           └── public/
│               ├── mod.rs
│               └── ping.rs        # Shows bot, WebSocket, and REST latency
```

## API Reference

### Core Types

| Type | Location | Description |
| ---- | -------- | ----------- |
| `Config` | `config.rs` | Application configuration loaded from environment variables |
| `Bot` | `bot.rs` | Main bot struct holding config, command maps, and `CooldownManager` |
| `SlashCommand` | `bot.rs` | Slash command definition with `CreateCommand` data and handler enum |
| `PrefixCommand` | `bot.rs` | Prefix command definition with name and handler enum |
| `Handler` | `bot.rs` | Serenity `EventHandler` impl — delegates to event modules |
| `CooldownManager` | `core/cooldown.rs` | Async per-command cooldown tracker using `tokio::sync::Mutex` |
| `WebhookEmbed` | `core/webhooks.rs` | Discord embed struct with serialization for webhook payloads |
| `User` | `models/user.rs` | MongoDB user model with `userId` and `points`, serde-annotated |

### Core Functions

| Function | Location | Description |
| -------- | -------- | ----------- |
| `Config::load()` | `config.rs` | Loads env vars into a `Config` instance |
| `Bot::new(config)` | `bot.rs` | Creates a new Bot with empty command maps |
| `Bot::register_commands(&mut self)` | `bot.rs` | Registers built-in commands |
| `CooldownManager::new()` | `core/cooldown.rs` | Creates a new cooldown manager |
| `cd.check(user_id, command, ms).await` | `core/cooldown.rs` | Returns `(on_cooldown, remaining_secs)` |
| `send_webhook(url, embed).await` | `core/webhooks.rs` | Posts an embed to a Discord webhook |
| `format_error(error, command_name)` | `core/command_utils.rs` | Returns an `ErrorReport` with timestamp |
| `log_command_usage(user_id, user_name, command, guild)` | `core/command_utils.rs` | Prints command usage to stdout |

### Event Handlers (on `Handler`)

| Method | Description |
| ------ | ----------- |
| `async ready(ctx, ready)` | Sets activity, registers global commands, sends ready webhook |
| `async interaction_create(ctx, interaction)` | Routes slash commands with cooldown + webhook |
| `async message(ctx, msg)` | Routes prefix commands with cooldown + webhook |
| `async guild_create(ctx, guild)` | Sends guild join notification |
| `async guild_delete(ctx, guild)` | Sends guild leave notification |

### Database

| Function | Location | Description |
| -------- | -------- | ----------- |
| `connect(uri).await` | `database/mongo.rs` | Connects to MongoDB with 10s timeout |
| `get_user(db, user_id).await` | `models/user.rs` | Fetches user from MongoDB |
| `create_user(db, user_id).await` | `models/user.rs` | Creates a user document |

## Adding Commands

### Slash Command

Create a new file in `src/Commands/Slash/[category]/[name].rs`:

```rust
use serenity::all::*;
use crate::bot::Bot;

pub fn register(bot: &mut Bot) {
    bot.slash_commands.insert(
        "hello".to_string(),
        crate::bot::SlashCommand {
            data: CreateCommand::new("hello").description("Say hello!"),
            handler: crate::bot::SlashCommandHandler::Ping, // extend the enum
        },
    );
}

pub async fn handle(ctx: &Context, command: &CommandInteraction) {
    let _ = command
        .create_response(&ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Hello! 👋"),
            ),
        )
        .await;
}
```

### Prefix Command

Create a new file in `src/Commands/Prefix/[category]/[name].rs`:

```rust
use serenity::all::*;
use crate::bot::Bot;

pub fn register(bot: &mut Bot) {
    bot.prefix_commands.insert(
        "hello".to_string(),
        crate::bot::PrefixCommand {
            name: "hello".to_string(),
            handler: crate::bot::PrefixCommandHandler::Ping, // extend the enum
        },
    );
}

pub async fn handle(ctx: &Context, msg: &Message, _args: Vec<String>) {
    let _ = msg.channel_id.say(&ctx.http, "Hello! 👋").await;
}
```

## Database Edition

This is the **MongoDB edition**. A **SQL edition** using Sequelize ORM is also available:

| Feature | MongoDB Edition | SQL Edition |
| ------- | --------------- | ----------- |
| Repository | [Discord-Handler-Rs](https://github.com/RealMtrx/Discord-Handler-Rs) | [Discord-Handler-Rs-Sequelize](https://github.com/RealMtrx/Discord-Handler-Rs-Sequelize) |
| Database | MongoDB | SQLite, PostgreSQL, MySQL, MSSQL |
| Driver | mongodb v3 | Diesel / SQLx |
| Dialects | MongoDB only | Multi-dialect via config |

## Related Repositories

Discord Handler Rust is part of a **26-repo ecosystem**. Here are the other repositories:

### Core Framework (MongoDB)

| Language | Repository |
| -------- | ---------- |
| JavaScript | [Discord-Handler-Js](https://github.com/RealMtrx/Discord-Handler-Js) |
| TypeScript | [Discord-Handler-Ts](https://github.com/RealMtrx/Discord-Handler-Ts) |
| Go | [Discord-Handler-Go](https://github.com/RealMtrx/Discord-Handler-Go) |
| Python | [Discord-Handler-Py](https://github.com/RealMtrx/Discord-Handler-Py) |
| C# | [Discord-Handler-Cs](https://github.com/RealMtrx/Discord-Handler-Cs) |
| Java | [Discord-Handler-Java](https://github.com/RealMtrx/Discord-Handler-Java) |
| Kotlin | [Discord-Handler-Kt](https://github.com/RealMtrx/Discord-Handler-Kt) |
| C++ | [Discord-Handler-Cpp](https://github.com/RealMtrx/Discord-Handler-Cpp) |
| Dart | [Discord-Handler-Dart](https://github.com/RealMtrx/Discord-Handler-Dart) |
| Ruby | [Discord-Handler-Rb](https://github.com/RealMtrx/Discord-Handler-Rb) |
| Lua | [Discord-Handler-Lua](https://github.com/RealMtrx/Discord-Handler-Lua) |
| PHP | [Discord-Handler-Php](https://github.com/RealMtrx/Discord-Handler-Php) |

### Database Editions (SQL)

| Language | Repository |
| -------- | ---------- |
| JavaScript | [Discord-Handler-Js-Sequelize](https://github.com/RealMtrx/Discord-Handler-Js-Sequelize) |
| TypeScript | [Discord-Handler-Ts-Sequelize](https://github.com/RealMtrx/Discord-Handler-Ts-Sequelize) |
| Go | [Discord-Handler-Go-Sequelize](https://github.com/RealMtrx/Discord-Handler-Go-Sequelize) |
| Rust | [Discord-Handler-Rs-Sequelize](https://github.com/RealMtrx/Discord-Handler-Rs-Sequelize) |
| Python | [Discord-Handler-Py-Sequelize](https://github.com/RealMtrx/Discord-Handler-Py-Sequelize) |
| C# | [Discord-Handler-Cs-Sequelize](https://github.com/RealMtrx/Discord-Handler-Cs-Sequelize) |
| Java | [Discord-Handler-Java-Sequelize](https://github.com/RealMtrx/Discord-Handler-Java-Sequelize) |
| Kotlin | [Discord-Handler-Kt-Sequelize](https://github.com/RealMtrx/Discord-Handler-Kt-Sequelize) |
| C++ | [Discord-Handler-Cpp-Sequelize](https://github.com/RealMtrx/Discord-Handler-Cpp-Sequelize) |
| Dart | [Discord-Handler-Dart-Sequelize](https://github.com/RealMtrx/Discord-Handler-Dart-Sequelize) |
| Ruby | [Discord-Handler-Rb-Sequelize](https://github.com/RealMtrx/Discord-Handler-Rb-Sequelize) |
| Lua | [Discord-Handler-Lua-Sequelize](https://github.com/RealMtrx/Discord-Handler-Lua-Sequelize) |
| PHP | [Discord-Handler-Php-Sequelize](https://github.com/RealMtrx/Discord-Handler-Php-Sequelize) |

### Hub

| Repository | Description |
| ---------- | ----------- |
| [Discord-Handler](https://github.com/RealMtrx/Discord-Handler) | Central hub — documentation, examples, changelog, roadmap |

## License

MIT License — Copyright © 2026 Mtrx

---

<div align="center">
  <sub>Built by <strong>Mtrx</strong> — Discord: <strong>0hu2</strong></sub>
  <br>
  <sub><a href="https://github.com/RealMtrx/Discord-Handler">Discord Handler Ecosystem</a></sub>
</div>
