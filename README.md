# Discord Handler TypeScript

A modern, feature-rich Discord bot handler built with **Discord.js v14** and **TypeScript**, featuring both slash commands and prefix commands with a robust modular architecture designed for scalability and maintainability.

## 🚀 Features

- **Dual Command System**: Support for both slash commands and prefix commands
- **Modular Architecture**: Clean separation of concerns with dedicated handlers
- **Anti-Crash System**: Comprehensive error handling and monitoring
- **Event-Driven**: Fully event-driven architecture
- **TypeScript**: Full type safety with strict TypeScript configuration
- **Webhook Logging**: Real-time logging for errors, commands, guild events, and bot status
- **MongoDB Integration**: Persistent data storage with Mongoose
- **Cooldown System**: Per-command cooldown management
- **Environment Configuration**: Secure configuration management with dotenv

## 📁 Project Structure

```
Discord-Handler/
├── package.json                  # Project dependencies and scripts
├── tsconfig.json                 # TypeScript configuration
├── src/                          # TypeScript source code
│   ├── index.ts                  # Main bot entry point
│   ├── config.ts                 # Bot configuration
│   ├── types.ts                  # Shared type definitions
│   ├── Core/                     # Core utilities and webhooks
│   │   ├── commandUtils.ts       # Cooldown, error formatting, logging
│   │   ├── emojis.ts             # Centralized emoji definitions
│   │   ├── errorWebhook.ts       # Error reporting via webhook
│   │   ├── joinGuildWebhook.ts   # Webhook when the bot joins a guild
│   │   ├── leaveGuildWebhook.ts  # Webhook when the bot leaves a guild
│   │   ├── prefixCommandWebhook.ts
│   │   ├── readyWebhook.ts
│   │   └── slashCommandWebhook.ts
│   ├── Database/
│   │   └── mongo.ts              # MongoDB connection setup
│   ├── Events/                   # Discord event handlers
│   │   ├── error.ts
│   │   ├── guildCreate.ts
│   │   ├── guildDelete.ts
│   │   ├── interactionCreate.ts
│   │   ├── messageCreate.ts
│   │   └── ready.ts
│   ├── Handlers/                 # Loaders and registrars
│   │   ├── AntiCrash.ts
│   │   ├── Commands.ts
│   │   ├── Events.ts
│   │   ├── logger.ts
│   │   ├── Models.ts
│   │   └── Prefix.ts
│   ├── Models/
│   │   └── userModel.ts
│   └── Commands/
│       ├── Prefix/Public/ping.ts
│       └── Slash/Public/ping.ts
```

## 🔧 Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/RealMtrx/Discord-Handler-Ts.git
   cd Discord-Handler-Ts
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Environment Setup**

   Edit `src/config.ts` with your values:

   ```typescript
   token: "your_bot_token",
   clientId: "your_client_id",
   botName: "Discord Handler",
   prefix: "$",
   ownerIds: ["owner_id_1", "owner_id_2"],
   MONGODB_URI: "your_mongo_uri",
   errorWebhook: "#",
   slashCommandWebhook: "#",
   prefixCommandWebhook: "#",
   joinGuildWebhook: "#",
   leaveGuildWebhook: "#",
   readyWebhook: "#",
   ```

4. **Run the bot**

   ```bash
   npm run dev    # Development with tsx
   npm run build  # Production build
   npm start      # Run built version
   ```

## 📋 Dependencies

- **discord.js**: ^14 - Discord API wrapper
- **mongoose**: ^8 - MongoDB ODM
- **dotenv**: ^16 - Environment variable management
- **chalk**: ^5 - Terminal colors
- **boxen**: ^8 - Boxed UI for startup report

## 📝 Command Development

### Creating Slash Commands

Create a new file in `src/Commands/Slash/[category]/[name].ts`:

```typescript
import { SlashCommandBuilder } from "discord.js";
import type { ExtendedClient } from "../../types.js";

export default {
  data: new SlashCommandBuilder()
    .setName("commandname")
    .setDescription("Command description"),
  cooldown: 3000,
  async execute(client: ExtendedClient, interaction: any) {
    await interaction.reply("Response");
  },
};
```

### Creating Prefix Commands

Create a new file in `src/Commands/Prefix/[category]/[name].ts`:

```typescript
import type { ExtendedClient } from "../../types.js";

export default {
  name: "commandname",
  description: "Command description",
  cooldown: 3000,
  async run(client: ExtendedClient, message: any, args: string[]) {
    message.reply("Response");
  },
};
```

---

**Discord Handler** - A modern, scalable Discord bot framework built with TypeScript.
