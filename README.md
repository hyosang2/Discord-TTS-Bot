# TTS Bot - Rust Rewrite

Text to speech Discord Bot using Serenity, Songbird, and Poise with **OpenAI TTS API support** including GPT-4o Mini TTS model.

## Features

- **Multiple TTS Engines**: gTTS, eSpeak, Amazon Polly, Google Cloud TTS, and **OpenAI TTS**
- **OpenAI TTS Integration**: Full support for OpenAI's TTS API with 6 voices (alloy, echo, fable, onyx, nova, shimmer)
- **Premium Features**: Advanced TTS modes with subscription support
- **Voice Customization**: Configurable speaking rates, voice selection, and audio settings
- **Discord Integration**: Seamless voice channel integration with advanced error handling

## System Requirements

### Required Dependencies
- **Rust nightly toolchain** (1.83+)
- **PostgreSQL** (13+)
- **FFmpeg** with Opus codec support
- **Git** for cloning repository

### Installation Commands

**macOS (Homebrew):**
```bash
# Install Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly

# Install system dependencies
brew install postgresql@14 ffmpeg opus git
brew services start postgresql@14
```

**Ubuntu/Debian:**
```bash
# Install Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly

# Install system dependencies
sudo apt update
sudo apt install postgresql postgresql-contrib ffmpeg libopus-dev git
sudo systemctl start postgresql
```

**Note**: Cargo automatically handles all Rust crate dependencies - no virtual environment needed.

## Setup Guide

### Easy (Public Bot):
- Invite the bot with [this invite](https://bit.ly/TTSBotSlash)
- Run `-setup #text_channel_to_read_from`
- Run `-join` in that text channel, while being in a voice channel
- Type normally in the setup text channel!

### Docker Setup:
1. **Install Prerequisites:**
   ```bash
   # Install Docker and Docker Compose
   # Install Git
   ```

2. **Clone and Configure:**
   ```bash
   git clone https://github.com/Discord-TTS/Bot.git
   cd Bot
   
   # Copy and configure
   cp config-docker.toml config.toml
   cp docker-compose-example.yml docker-compose.yml
   cp Dockerfile-prod Dockerfile  # or Dockerfile-dev for faster builds
   ```

3. **Edit Configuration:**
   Edit `config.toml` (copied from `config-docker.toml`) and fill out these **required** fields:
   ```toml
   [Main]
   # Discord Bot Token (REQUIRED - from Discord Developer Portal)
   token = "your_discord_bot_token_here"
   
   # TTS Service URL (already configured for Docker)
   tts_service = "http://localhost:20310"
   
   # OpenAI TTS API Key (OPTIONAL - for OpenAI TTS mode)
   openai_api_key = "sk-your-openai-api-key-here"
   
   [PostgreSQL-Info]
   # Database connection (already configured for Docker)
   database = "tts"
   password = "tts_password" 
   host = "localhost"
   user = "tts"
   
   [Webhook-Info]
   # Discord webhook URLs for logging (OPTIONAL - for error tracking)
   logs = "https://discord.com/api/webhooks/..."
   errors = "https://discord.com/api/webhooks/..."
   dm_logs = "https://discord.com/api/webhooks/..."
   ```
   
   **Optional Administrative Features** (uncomment if you want them):
   ```toml
   [Main]
   # Support/Management Server Configuration (OPTIONAL)
   # These are for bot administration, not deployment restrictions
   # main_server = 1234567890123456789           # Your support server ID
   # announcements_channel = 1234567890123456789  # Channel for bot announcements
   # invite_channel = 1234567890123456789         # Channel with invite info
   # ofs_role = 1234567890123456789               # Role for server owners
   # main_server_invite = "https://discord.gg/your-invite"
   ```
   
   Also edit `docker-compose.yml` with your environment variables if needed.

4. **Run:**
   ```bash
   docker-compose up --build -d
   docker-compose logs bot  # Check status
   ```

### Self-Hosting Setup:

1. **Install System Dependencies** (see System Requirements above)

2. **Clone Repository:**
   ```bash
   git clone https://github.com/Discord-TTS/Bot.git
   cd Bot
   ```

3. **Configure Database:**
   ```bash
   # Create PostgreSQL database and user
   sudo -u postgres psql
   ```
   ```sql
   CREATE DATABASE tts_bot;
   CREATE USER tts_user WITH PASSWORD 'your_secure_password';
   GRANT ALL PRIVILEGES ON DATABASE tts_bot TO tts_user;
   \q
   ```

4. **Configure Bot:**
   ```bash
   cp config-selfhost.toml config.toml
   # Edit config.toml with your settings (see Configuration section below)
   ```

5. **Build and Run:**
   ```bash
   cargo build --release
   ./target/release/tts_bot
   ```

## Configuration

### Required Configuration (`config.toml`)

Copy `config-selfhost.toml` to `config.toml` and fill out these **required** fields:

```toml
[Main]
# Discord Bot Token (REQUIRED - from Discord Developer Portal)
token = "your_discord_bot_token_here"

# TTS Service URL (REQUIRED - external service)
tts_service = "https://your-tts-service-instance.com"

# OpenAI TTS API Key (OPTIONAL - for OpenAI TTS mode)
openai_api_key = "sk-your-openai-api-key-here"

[PostgreSQL-Info]
host = "localhost"
user = "tts_user"
password = "your_secure_password"
database = "tts_bot"

[Webhook-Info]
# Discord webhook URLs for logging (OPTIONAL - for error tracking)
logs = "https://discord.com/api/webhooks/..."
errors = "https://discord.com/api/webhooks/..."
dm_logs = "https://discord.com/api/webhooks/..."
```

**Optional Administrative Features** (uncomment if you want them):
```toml
[Main]
# Support/Management Server Configuration (OPTIONAL)
# These are for bot administration, not deployment restrictions
# main_server = 1234567890123456789           # Your support server ID
# announcements_channel = 1234567890123456789  # Channel for bot announcements
# invite_channel = 1234567890123456789         # Channel with invite info
# ofs_role = 1234567890123456789               # Role for server owners
# main_server_invite = "https://discord.gg/your-invite"
```

### Optional Configuration

```toml
[Main]
log_level = "info"                    # Logging level
tts_service_auth_key = "auth_key"     # TTS service authentication
website_url = "https://your-site.com" # Bot website
proxy_url = "proxy_url_if_needed"     # HTTP proxy

[Website-Info]
url = "https://your-website.com"
stats_key = "your_stats_key"

[Premium-Info]
# Premium subscription configuration
# (See original config files for details)

[Bot-List-Tokens]
# Bot list API tokens for statistics
# (See original config files for details)
```

## OpenAI TTS Setup

1. **Get OpenAI API Key:**
   - Sign up at [OpenAI Platform](https://platform.openai.com/)
   - Create an API key in your dashboard
   - Add credits to your account

2. **Configure:**
   ```toml
   [Main]
   openai_api_key = "sk-your-openai-api-key-here"
   ```

3. **Use OpenAI TTS:**
   ```
   /set mode OpenAI          # Switch to OpenAI TTS (premium required)
   /set voice alloy          # Choose voice (alloy, echo, fable, onyx, nova, shimmer)
   /speaking_rate 1.5        # Adjust speed (0.25x to 4.0x)
   /voices                   # List all available voices
   ```

## Discord Bot Setup

1. **Create Discord Application:**
   - Go to [Discord Developer Portal](https://discord.com/developers/applications)
   - Create a new application
   - Go to "Bot" section and create a bot
   - Copy the bot token

2. **Set Bot Permissions:**
   - In the OAuth2 > URL Generator section
   - Select "bot" scope
   - Select these permissions:
     - Send Messages
     - Use Slash Commands
     - Connect (to voice channels)
     - Speak (in voice channels)
     - Embed Links

3. **Invite Bot to Server:**
   - Use the generated OAuth2 URL to invite the bot
   - Ensure the bot has the required permissions

## Database Setup

The bot uses PostgreSQL with automatic schema migration. The database schema is created automatically on first run.

**Docker Setup** (Automatic):
```bash
# PostgreSQL is included in docker-compose-example.yml
docker-compose up -d database
```

**Manual Setup:**
```bash
# Install PostgreSQL
sudo apt install postgresql postgresql-contrib  # Ubuntu
brew install postgresql@14                      # macOS

# Create database and user
sudo -u postgres createuser --interactive tts_user
sudo -u postgres createdb tts_bot -O tts_user
sudo -u postgres psql -c "ALTER USER tts_user PASSWORD 'your_password';"
```

## Usage

Once configured and running:

1. **Setup in Discord:**
   ```
   /setup #text-channel-name    # Set channel for TTS
   /join                        # Join voice channel
   ```

2. **TTS Commands:**
   ```
   /set mode OpenAI            # Use OpenAI TTS (premium)
   /set voice alloy            # Choose voice
   /speaking_rate 1.2          # Adjust speed
   /voices                     # List voices
   ```

3. **Type in setup channel** - Messages will be read aloud in voice channel

## Troubleshooting

### Common Issues:

1. **Build Errors:**
   - Ensure Rust nightly is installed: `rustup default nightly`
   - Update Rust: `rustup update`

2. **Database Connection:**
   - Verify PostgreSQL is running: `systemctl status postgresql`
   - Check connection details in `config.toml`

3. **Voice Connection (4006 errors):**
   - This bot uses Songbird which handles Discord's voice protocol changes
   - Ensure bot has proper voice permissions in Discord

4. **OpenAI TTS:**
   - Verify API key is correct
   - Check OpenAI account has sufficient credits
   - Ensure premium subscription for OpenAI mode access

### Logs:
```bash
# Check bot logs
docker-compose logs bot          # Docker setup
./target/release/tts_bot         # Self-hosted (outputs to terminal)
```

## Development

```bash
# Development build
cargo build

# Run in development mode
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy
```
