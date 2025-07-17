# Discord TTS Bot

A powerful, self-hostable Text-to-Speech Discord bot with support for multiple TTS services, currently optimized for OpenAI's high-quality TTS models.

## 🚀 Key Features

- **🎙️ OpenAI TTS Integration**: 
  - Full support for OpenAI's TTS API with 6 voices (alloy, echo, fable, onyx, nova, shimmer)
  - **NEW: Multiple OpenAI Models**:
    - `tts-1`: Faster generation, lower quality
    - `tts-1-hd`: High definition audio (default)
    - `gpt-4o-mini-tts`: Experimental GPT-4o mini TTS model
- **🎛️ Voice Customization**: Configurable speaking rates (0.25x-4.0x), voice selection, and per-user settings
- **💬 Discord Integration**: Seamless voice channel integration with slash commands and prefix commands
- **🌐 Multi-Server Support**: Works across multiple Discord servers with independent configurations
- **🔧 Flexible Configuration**: Per-server and per-user voice settings, customizable prefixes, and more
- **🔒 Privacy Controls**: Per-server user opt-out functionality for users who don't want their messages processed

**Note**: Other TTS engines (gTTS, eSpeak, Amazon Polly, Google Cloud TTS) are temporarily disabled in this configuration.

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
   ```

3. **Edit Configuration:**
   Edit `config.toml` and fill out these **required** fields:
   ```toml
   [Main]
   # Discord Bot Token (REQUIRED - from Discord Developer Portal)
   token = "your_discord_bot_token_here"
   
   # OpenAI TTS API Key (REQUIRED - for TTS functionality)
   openai_api_key = "sk-your-openai-api-key-here"
   
   [PostgreSQL-Info]
   # Database connection (already configured for Docker)
   database = "tts"
   password = "tts_password" 
   host = "database"  # Uses Docker service name for networking
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
   
4. **Run:**
   ```bash
   docker compose up --build -d
   docker compose logs bot  # Check status
   ```
   
   **✅ Enhanced Docker Setup:**
   - **Persistent Database Storage**: PostgreSQL data survives container restarts
   - **Automatic Migration Recovery**: Database issues self-heal on startup
   - **Improved Reliability**: No more "relation 'guilds' does not exist" errors

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

# OpenAI TTS API Key (REQUIRED - for TTS functionality)
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

2. **Enable Privileged Gateway Intents:**
   - In the "Bot" section of your application
   - Enable these privileged intents:
     - **Server Members Intent** (required)
     - **Message Content Intent** (required)

3. **Set Bot Permissions:**
   - In the OAuth2 > URL Generator section
   - Select "bot" scope
   - Select these permissions:
     - Send Messages
     - Use Slash Commands
     - Connect (to voice channels)
     - Speak (in voice channels)
     - Embed Links
     - Read Messages/View Channels

4. **Invite Bot to Server:**
   - Use the generated OAuth2 URL to invite the bot
   - Ensure the bot has the required permissions
```
https://discord.com/oauth2/authorize?client_id=1394185157472423966&permissions=2284932096&integration_type=0&scope=bot
```

## Database Setup

The bot uses PostgreSQL with automatic schema migration. The database schema is created automatically on first run.

**Docker Setup** (Automatic):
```bash
# PostgreSQL is included in docker-compose-example.yml
docker compose up -d database
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
   /set voice alloy            # Choose voice (alloy, echo, fable, onyx, nova, shimmer)
   /set openai_model           # Choose OpenAI model (tts-1, tts-1-hd, gpt-4o-mini-tts)
   /speaking_rate 1.2          # Adjust speed (0.25x to 4.0x)
   /voices                     # List all available voices
   /settings                   # View current settings including OpenAI model
   ```

3. **Privacy Controls:**
   ```
   /opt_out true               # Opt out of TTS processing in this server
   /opt_out false              # Opt back into TTS processing in this server
   ```

4. **OpenAI Model Selection (NEW):**
   ```
   /set openai_model tts-1              # Fast generation, lower quality
   /set openai_model tts-1-hd            # High quality (default)
   /set openai_model gpt-4o-mini-tts     # Experimental GPT-4o mini model
   /set openai_model                    # Reset to default (tts-1-hd)
   ```

5. **Type in setup channel** - Messages will be read aloud in voice channel

## Privacy & User Controls

The bot includes comprehensive privacy controls for users who do not wish their messages to be processed:

- **Per-Server Opt-Out**: Users can opt out of TTS processing on a per-server basis using `/opt_out true`
- **Global Bot Ban**: Server owners can request global bans for users who violate terms of service
- **No Data Retention**: Messages are processed in real-time and not stored
- **OpenAI Processing**: When using OpenAI TTS, messages are sent to OpenAI's API for voice synthesis

**Important**: Users concerned about their messages being processed by OpenAI for TTS generation should use the `/opt_out true` command to exclude themselves from TTS processing in specific servers.

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
   - Note: OpenAI TTS is now the default mode

5. **Docker Database Issues ("relation 'guilds' does not exist"):**
   
   **✅ RESOLVED: This issue has been permanently fixed** with the following improvements:
   - **Persistent Database Storage**: PostgreSQL data now persists across container restarts
   - **Enhanced Migration Logic**: Automatic detection and recovery from database/config mismatches
   - **Robust Error Handling**: Validation checks ensure tables exist before proceeding
   
   **If you still encounter this issue (unlikely):**
   ```bash
   # Emergency reset (only if absolutely necessary)
   sudo docker compose down -v
   sudo docker compose up --build -d
   ```
   
   **What was fixed:**
   - Added persistent volume for PostgreSQL data in `docker-compose.yml`
   - Enhanced migration logic to validate table existence regardless of setup flag
   - Automatic recovery when database state doesn't match config expectations
   - Post-migration validation to ensure critical tables were created successfully
   
   **Prevention (now automatic):**
   - Database persistence prevents data loss on container restart
   - Migration system automatically detects and recovers from inconsistent states
   - No manual intervention required for database management

### Logs:
```bash
# Check bot logs
docker compose logs bot          # Docker setup
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

## 📋 TODO / Roadmap

### High Priority
- [ ] **Disable "premium feature" warnings** - Remove or make optional the premium-only restrictions
- [ ] **Admin-level control on certain commands** - Implement role-based command permissions
- [ ] **Automatic sentiment adjustment** - Adjust TTS tone/speed based on message sentiment

### Medium Priority  
- [ ] **Other TTS services with funny voices** - Re-enable and expand support for:
  - gTTS with language variety
  - eSpeak with robotic effects
  - Amazon Polly with neural voices
  - Google Cloud TTS with WaveNet
  - Novelty/character voices

### Future Features
- [ ] **STT (Speech-to-Text)** - Transcribe voice channel conversations:
  - Real-time transcription to text channel
  - Voice command recognition
  - Meeting notes/summaries
  - Multi-language support

### Contributing
Contributions are welcome! Please check the TODO list above for areas where help is needed.

## License
This project is licensed under the AGPL-3.0 License - see the LICENSE file for details.
