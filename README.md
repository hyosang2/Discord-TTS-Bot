# Discord TTS Bot

A powerful, self-hostable Text-to-Speech Discord bot with support for multiple TTS services, currently optimized for OpenAI's high-quality TTS models.

## 🚀 Key Features

- **🎙️ OpenAI TTS Integration**: 
  - Full support for OpenAI's TTS API with 11 voices (alloy, ash, ballad, coral, echo, fable, nova, onyx, sage, shimmer, verse)
  - **Multiple OpenAI Models**:
    - `tts-1`: Faster generation, lower quality
    - `tts-1-hd`: High definition audio (default)
    - `gpt-4o-mini-tts`: Experimental GPT-4o mini TTS model
  - **NEW: Speech Style Instructions**: Control speaking style and tone with natural language instructions
    - Works with `gpt-4o-mini-tts` model only
    - Temporary per-message instructions: `\happy Hello world!` or `[speak like a narrator] Once upon a time...`
    - Persistent user-level instructions via `/set instruction`
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
   # First set your TTS mode to OpenAI
   /set mode OpenAI TTS (high quality)   # Required first
   
   # Then configure voice settings
   /set voice alloy                      # Choose voice (alloy, ash, ballad, coral, echo, fable, nova, onyx, sage, shimmer, verse)
   /speaking_rate 1.5                    # Adjust speed (0.25x to 4.0x)
   /voices                               # List all available voices
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

**Release Bot Invite Link:**
```
https://discord.com/oauth2/authorize?client_id=1394185157472423966&permissions=2284932096&integration_type=0&scope=bot
```

**Beta Bot Invite Link:**
```
https://discord.com/oauth2/authorize?client_id=1396345260090851481&permissions=2284932096&integration_type=0&scope=bot
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
   # First set your TTS mode to OpenAI
   /set mode OpenAI TTS (high quality)     # Required before setting OpenAI voices
   
   # Then configure OpenAI settings
   /set voice alloy                        # Choose voice (alloy, ash, ballad, coral, echo, fable, nova, onyx, sage, shimmer, verse)
   /set openai_model tts-1-hd              # Choose OpenAI model (tts-1, tts-1-hd, gpt-4o-mini-tts)
   /set instruction                        # Set persistent speaking style instructions (gpt-4o-mini-tts only)
   /speaking_rate 1.2                      # Adjust speed (0.25x to 4.0x)
   /voices                                 # List all available voices
   /settings                               # View current settings including OpenAI model and instructions
   ```

3. **Privacy Controls:**
   ```
   /opt_out true               # Opt out of TTS processing in this server
   /opt_out false              # Opt back into TTS processing in this server
   ```

4. **OpenAI Model Selection:**
   ```
   /set openai_model tts-1              # Fast generation, lower quality
   /set openai_model tts-1-hd            # High quality (default)
   /set openai_model gpt-4o-mini-tts     # Experimental GPT-4o mini model
   /set openai_model                    # Reset to default (tts-1-hd)
   ```

5. **Speech Style Instructions (NEW):**
   ```
   # Persistent instructions (saved to your profile)
   /set instruction speak like a narrator          # Set speaking style
   /set instruction                                # Clear instructions
   
   # Temporary per-message instructions (only for gpt-4o-mini-tts)
   \happy Hello everyone!                           # Single word instruction
   [speak quietly] This is a secret message.       # Multi-word instruction
   [excited] I just won the lottery!              # Emotional instruction
   \robotic Beep boop, I am a robot.              # Character instruction
   ```

6. **Type in setup channel** - Messages will be read aloud in voice channel

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

## Deployment Approaches

This bot supports two deployment strategies optimized for different use cases:

### Option 5: Hybrid Development Setup (Recommended for Development)

**Best for**: Fast iteration during development on Apple Silicon or Linux x86_64

```bash
# 1. Start only database and XTTS services in Docker
docker compose -f docker-compose-example.yml up database xtts-service -d

# 2. Run bot locally with native performance
cargo run

# Benefits:
# - Instant recompilation and testing
# - Native debugger support
# - No Docker rebuild delays
# - Services remain containerized for consistency
```

**docker-compose.dev.yml** (create this file):
```yaml
services:
  database:
    image: postgres:13
    ports: [5433:5432]
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      POSTGRES_DB: tts
      POSTGRES_USER: tts
      POSTGRES_PASSWORD: tts_password
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U tts -d tts"]
      interval: 5s
      timeout: 5s
      retries: 5

  xtts-service:
    image: ghcr.io/coqui-ai/xtts-streaming-server:latest-cpu
    platform: linux/amd64  # XTTS only supports x86_64
    ports: [8000:80]
    environment:
      - COQUI_TOS_AGREED=1
    volumes:
      - ./xtts_voice_clips:/app/tts_models/voices
      - ./xtts_models:/app/tts_models
    restart: unless-stopped

volumes:
  postgres_data:
```

### Option 3: Multi-Architecture Production Build (Recommended for Distribution)

**Best for**: Deploying to both x86_64 and ARM64 (Apple Silicon, Raspberry Pi)

```dockerfile
# Dockerfile.multiarch
FROM rust:1.88 AS builder

# Install dependencies for both architectures
RUN apt-get update && apt-get install -y \
    gcc-aarch64-linux-gnu \
    gcc-x86-64-linux-gnu \
    cmake libopus-dev

# Add Rust targets
RUN rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu

# Copy source
WORKDIR /bot
COPY . .

# Build for target architecture
ARG TARGETARCH
RUN case ${TARGETARCH} in \
    amd64) export RUST_TARGET=x86_64-unknown-linux-gnu ;; \
    arm64) export RUST_TARGET=aarch64-unknown-linux-gnu ;; \
    esac && \
    cargo build --release --target $RUST_TARGET && \
    mv target/$RUST_TARGET/release/tts_bot /bot/tts_bot

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /bot/tts_bot /usr/local/bin/discord_tts_bot
CMD ["/usr/local/bin/discord_tts_bot"]
```

**GitHub Actions** for automated multi-arch builds:
```yaml
# .github/workflows/docker-multiarch.yml
name: Build Multi-Architecture Images

on:
  push:
    tags: ['v*']

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: docker/setup-qemu-action@v3
      - uses: docker/setup-buildx-action@v3
      - uses: docker/login-action@v3
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}
      
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          file: ./Dockerfile.multiarch
          platforms: linux/amd64,linux/arm64
          push: true
          tags: |
            ghcr.io/${{ github.repository }}:latest
            ghcr.io/${{ github.repository }}:${{ github.ref_name }}
```

### Platform Compatibility

| Component | x86_64 | ARM64 (M1/M2) | Notes |
|-----------|--------|---------------|--------|
| Bot | ✅ Native | ✅ Native | Multi-arch build |
| PostgreSQL | ✅ Native | ✅ Native | Official images |
| XTTS | ✅ Native | ⚠️ Emulated | x86_64 only, runs via Rosetta |

### When to Use Each Approach

- **Development**: Use Option 5 (Hybrid) for fastest iteration
- **Testing**: Use Option 5 locally or full Docker Compose
- **CI/CD**: Use Option 3 with GitHub Actions
- **Production**: Use Option 3 for universal compatibility
- **Distribution**: Publish multi-arch images to Docker Hub/GHCR

## 🔥 Native XTTS Setup for Apple Silicon (NEW)

**🎯 Performance Results on M4 Pro:**
- **Korean test message**: Reduced from 26 seconds (Docker) to 15-18 seconds (Native) 
- **Performance improvement**: 30-40% faster than Docker with Rosetta emulation
- **Memory usage**: 40% less memory consumption (2-3GB vs 4-5GB)
- **Architecture**: Native ARM64 processing, no x86_64 emulation overhead
- **✅ Multi-sentence support**: Proper audio chunking for long messages (250+ characters)
  - Automatically splits long text into chunks for XTTS processing
  - Sequential audio playback maintains proper sentence flow
  - No audio corruption from improper WAV concatenation
- **🌏 Multi-language punctuation support**: Smart chunking for Asian languages
  - Supports Japanese punctuation: `。` `！` `？` `、`
  - Supports Chinese punctuation: `。` `！` `？` `，`
  - Supports Korean sentence endings and proper text flow
  - Character-based counting (not byte-based) for accurate chunking

### Prerequisites

```bash
# Install miniconda for Python environment management
brew install --cask miniconda

# Install system dependencies
brew install portaudio ffmpeg

# Restart shell to load conda
exec $SHELL -l
```

### Installation

1. **Create Conda Environment:**
   ```bash
   conda create -n xtts python=3.10 -y
   conda activate xtts
   ```

2. **Install XTTS and Dependencies:**
   ```bash
   pip install xtts-api-server
   pip install torch==2.1.2 torchaudio==2.1.2  # Compatible PyTorch version
   ```

3. **Verify MPS Support:**
   ```python
   import torch
   print('MPS available:', torch.backends.mps.is_available())  # Should be True
   print('PyTorch version:', torch.__version__)  # Should be 2.1.2
   ```

### Running Native XTTS

**Option 1: Manual Startup (Recommended)**
```bash
# Terminal 1: Start database only
docker compose -f docker-compose.dev.yml up database -d

# Terminal 2: Start native XTTS server
conda activate xtts
python -m xtts_api_server \
    --host 0.0.0.0 \
    --port 8000 \
    --device cpu \
    --speaker-folder ./xtts_voice_clips \
    --use-cache

# Terminal 3: Start Discord bot
cargo run
```

**Option 2: Using Startup Script**
```bash
# Start database
docker compose -f docker-compose.dev.yml up database -d

# Start XTTS server (background)
./start-xtts.sh &

# Start Discord bot
cargo run
```

### Voice Clips Setup

Organize voice clips in the following directory structure:
```
xtts_voice_clips/
├── default/
│   └── en.wav (valid audio file)
└── syanster/
    ├── en.wav (valid audio file)
    └── ko.wav (valid audio file)
```

**Important**: All WAV files must contain valid audio data (not 0 bytes). Remove any corrupted files:
```bash
# Check for empty files
find xtts_voice_clips -name "*.wav" -size 0

# Remove corrupted directories if found
rm -rf xtts_voice_clips/bad_directory
```

### Performance Characteristics

| Metric | Docker (x86_64 + Rosetta) | Native (ARM64) | Improvement |
|--------|---------------------------|----------------|-------------|
| Korean TTS | 26 seconds | 15-18 seconds | 30-40% faster |
| Memory | 4-5GB | 2-3GB | 40% less |
| CPU efficiency | Lower (emulation) | Higher (native) | Better |
| Setup time | 5 seconds | 3 seconds | 40% faster |

### Hardware Specifications (M4 Pro)
- **CPU**: 12-core M4 Pro (8 performance + 4 efficiency cores)
- **GPU**: 16-core (unused - XTTS doesn't support MPS yet)
- **Memory**: 24GB unified memory
- **Architecture**: ARM64 (native, no emulation)

### Known Limitations

1. **GPU Acceleration**: XTTS doesn't support Apple Silicon GPU (MPS) yet - CPU only
2. **PyTorch Version**: Must use 2.1.2 for compatibility (not latest 2.7.1)
3. **Model Loading**: Initial model download (~1.86GB) on first run
4. **Character Limit**: 250 characters per chunk (handled automatically)

### Troubleshooting

**PyAudio compilation error:**
```bash
brew install portaudio
```

**PyTorch weights_only error:**
```bash
pip install torch==2.1.2 torchaudio==2.1.2
```

**ffmpeg warning:**
```bash
brew install ffmpeg
```

**XTTS server not responding:**
```bash
# Check if port 8000 is available
lsof -i :8000
curl http://localhost:8000/docs  # Should return HTML page
```

For detailed troubleshooting, see `NATIVE-XTTS-SETUP.md`.

### Automatic Startup on macOS

Configure the bot to start automatically when your Mac restarts:

```bash
# Quick setup - installs all services
./manage-startup.sh install

# Start services now
./manage-startup.sh start

# Check status
./manage-startup.sh status
```

**Features:**
- Automatic startup of PostgreSQL, XTTS, and Discord bot
- Service management (start/stop/restart/status)
- Automatic restart on crashes
- Comprehensive logging
- Individual service control

For detailed setup instructions, see `AUTO-STARTUP-GUIDE.md`.

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
