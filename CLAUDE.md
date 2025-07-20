# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Discord TTS Bot written in Rust using Serenity, Songbird, and Poise. Multi-workspace Cargo project with modular architecture. Currently configured for OpenAI TTS only - other TTS services (gTTS, eSpeak, Polly, gCloud) are temporarily disabled.

### Recent Updates
- **OpenAI Speech Style Instructions**: Added support for controlling TTS speaking style with natural language instructions:
  - Command: `/set instruction [instruction]` for persistent user-level instructions
  - Temporary per-message instructions: `\instruction text` (single word) or `[instruction] text` (multi-word)
  - Works only with `gpt-4o-mini-tts` model (instructions ignored for tts-1 and tts-1-hd)
  - Database: Added `openai_instruction` column (varchar(500)) to `user_voice` and `guild_voice` tables
  - Fallback logic: temporary instruction → user-level instruction → guild-level instruction → none
  - Dependencies: Updated async-openai from 0.25 to 0.29 for instructions parameter support
- **User Opt-Out Feature**: Added per-server user privacy controls:
  - Command: `/opt_out true/false` for per-server TTS processing control
  - Database: New `user_opt_out` table with foreign key constraints
  - Users can independently opt out of TTS processing on each server
  - Bot-banned status overrides opt-out preferences
- **OpenAI Model Selection**: Added support for switching between three OpenAI TTS models:
  - `tts-1`: Faster generation, lower quality
  - `tts-1-hd`: High definition audio (default)
  - `gpt-4o-mini-tts`: Experimental GPT-4o mini TTS model
- **Command**: `/set openai_model [model]` for model selection
- **Database**: Added `openai_model` column to `user_voice` and `guild_voice` tables

## Build and Development Commands

### Building
- `cargo build` - Build debug version
- `cargo build --release` - Build optimized release version
- `cargo run` - Run in development mode

### Testing and Quality
- `cargo test` - Run all tests
- `cargo clippy` - Run linter (uses custom clippy.toml config)
- `cargo clippy --all-targets --all-features` - Run comprehensive linting

### Configuration
- Copy `config-selfhost.toml` to `config.toml` for self-hosting setup
- Copy `config-docker.toml` to `config.toml` for Docker setup
- Fill out PostgreSQL connection details, Discord bot token, and OpenAI API key
- Enable privileged gateway intents (Server Members Intent, Message Content Intent) in Discord Developer Portal
- **Required fields**: `token`, `openai_api_key`, `announcements_channel`, `invite_channel`, `main_server`, `main_server_invite`, `ofs_role`
- **Database migration**: Now handled automatically with enhanced recovery mechanisms
- **Docker**: Use `host = "database"` in PostgreSQL config (not `localhost`)

### Docker Development Commands

#### Building and Running
- `docker compose build bot` - Build bot container only (faster for code changes)
- `docker compose up --build -d` - Build and run all containers in background
- `docker compose up -d` - Start containers without rebuilding (if already built)
- `docker compose down` - Stop and remove containers
- `docker compose down -v` - Stop containers and remove volumes (complete reset)

#### Monitoring and Debugging
- `docker compose logs bot` - View bot logs (one-time)
- `docker compose logs bot -f` - Follow bot logs in real-time
- `docker compose logs bot -f --tail=50` - Show last 50 lines and follow
- `docker compose ps` - Show container status
- `docker compose exec bot sh` - Access bot container shell

#### Development Workflow
1. **Make code changes**
2. **Rebuild bot container**: `docker compose build bot`
3. **Restart bot**: `docker compose up -d bot`
4. **Check logs**: `docker compose logs bot -f`

#### Troubleshooting Commands
- **Check container health**: `docker compose ps`
- **Database issues**: `docker compose logs database`
- **Full restart**: `docker compose restart`
- **Clean rebuild**: `docker compose down && docker compose up --build -d`

### ✅ Docker Database Issues (RESOLVED)
- **"relation 'guilds' does not exist"**: **PERMANENTLY FIXED**
  - **Root cause**: Missing persistent PostgreSQL volumes + setup flag mismatch
  - **Fixes implemented**:
    - Added persistent volume (`postgres_data`) to `docker-compose.yml`
    - Enhanced migration logic with table existence validation in `tts_migrations/src/lib.rs`
    - Fixed database host configuration in `config-docker.toml` (`database` not `localhost`)
    - Added post-migration validation and automatic recovery mechanisms
  - **Result**: Database data persists across container restarts, automatic self-healing
  - **Emergency reset** (rarely needed): `docker compose down -v && docker compose up --build -d`

## Architecture

### Workspace Structure
Multi-crate workspace with specialized modules:
- **tts_core**: Core functionality, database models, error handling, structs, analytics
- **tts_commands**: All bot commands (slash and prefix), premium checks, command validation
- **tts_events**: Discord event handlers for messages, guilds, voice states, members
- **tts_tasks**: Background tasks (analytics, bot list updates, web updates) using Looper trait
- **tts_migrations**: Database schema and migrations

### Key Components
- **Main Bot**: Entry point in `src/main.rs` using jemalloc allocator and tokio runtime
- **Framework**: Poise framework for command handling with prefix and slash command support
- **Voice**: Songbird integration for voice channel management and TTS playbook
- **Database**: PostgreSQL with sqlx for persistent storage (guilds, users, voice settings, opt-out preferences, instruction settings)
- **TTS Integration**: OpenAI TTS API for text-to-speech synthesis (default mode) with instruction support
- **Privacy Controls**: Per-server user opt-out system with database-backed persistence
- **Premium System**: Role-based premium features with subscription validation
- **Analytics**: Background collection and processing of usage metrics
- **Webhook Logging**: Discord webhook integration for error and event logging (optional)

### Configuration Files
- `clippy.toml`: Custom linting rules, disallows specific methods
- Rust nightly features used: `let_chains`, `debug_closure_helpers`
- Dependencies include serenity (next branch), poise, songbird for Discord functionality

### Key Patterns
- Extensive use of `Result<T>` error handling throughout codebase
- Database handlers created via macros in startup
- Background tasks implement `Looper` trait for consistent interval execution
- Premium command validation via `premium_command_check`
- Event-driven architecture with centralized error handling

### Database Migration System
- **Migration entry point**: `tts_migrations::load_db_and_conf()` in `src/main.rs`
- **Setup logic**: Enhanced with table existence validation (`table_exists()` function)
- **Recovery mechanism**: Automatic detection of database/config state mismatches
- **Transaction safety**: All migrations run in single transaction with post-validation
- **Robustness**: Setup runs if config flag missing OR critical tables don't exist
- **User opt-out table**: `user_opt_out` with foreign key constraints to `userinfo` and `guilds`

## Privacy and User Controls

### User Opt-Out System
Implemented per-server user privacy controls in `tts_commands/src/settings/mod.rs:227`:

- **Command**: `/opt_out true/false` allows users to control TTS processing per server
- **Database**: `user_opt_out` table tracks preferences with composite primary key (user_id, guild_id)
- **Foreign Key Safety**: Ensures userinfo and guilds records exist before creating opt-out entries
- **Message Processing**: `tts_events/src/message/tts.rs:24` checks opt-out status before TTS processing
- **Override Behavior**: Bot-banned users are excluded regardless of opt-out preferences

### Implementation Details
- **Database Handler**: `user_opt_out_db` in Data struct for caching and performance
- **Foreign Key Fix**: `tokio::try_join!` ensures parent records exist before opt-out insertion
- **Default Behavior**: Users are opted-in by default (no record = participation)
- **Per-Server Granularity**: Users can have different opt-out settings across servers

## OpenAI Instructions System

### Speech Style Instructions Implementation
Added comprehensive support for controlling OpenAI TTS speaking style and tone using natural language instructions in `tts_core/src/common.rs:94` and `tts_events/src/message/tts.rs:20`:

- **Instruction Parsing**: `parse_instruction()` function supports two formats:
  - Single word: `\instruction text` (e.g., `\happy Hello world!`)
  - Multi-word: `[instruction] text` (e.g., `[speak like a narrator] Once upon a time...`)
- **Command Interface**: `/set instruction [instruction]` in `tts_commands/src/settings/mod.rs:1173`
- **Database Storage**: `openai_instruction` column (varchar(500)) in `user_voice` table (per-user storage)
- **Scope**: Instructions are stored per-user, not per-guild - each user has their own persistent instruction
- **API Integration**: Updated `fetch_openai_audio()` to use async-openai 0.29 instructions parameter
- **Model Compatibility**: Instructions only work with `gpt-4o-mini-tts` model (ignored for tts-1/tts-1-hd)

### Fallback Logic
Instruction selection follows priority order in `tts_events/src/message/tts.rs:159`:
1. **Temporary instruction**: Parsed from message content (highest priority)
2. **User-level instruction**: Persistent setting via `/set instruction`
3. **Guild-level instruction**: Server-wide default (future feature)
4. **None**: No instruction applied

### Technical Implementation
- **Dependencies**: Updated `async-openai` from 0.25 to 0.29 for instructions parameter support
- **Request Building**: Conditional request building in `fetch_openai_audio()` to add instructions only for compatible models
- **Validation**: 500-character limit on instruction length with user-friendly error messages
- **Settings Display**: Instructions shown in `/settings` command output when set
- **Database Migration**: Automatic schema updates add `openai_instruction` columns to existing tables

### Key Files Modified
- `tts_core/src/common.rs`: OpenAI API integration with instructions parameter
- `tts_events/src/message/tts.rs`: Message parsing and instruction extraction
- `tts_commands/src/settings/mod.rs`: Command interface and database operations
- `tts_core/src/database_models.rs`: Database schema updates for instruction storage
- `tts_migrations/src/lib.rs`: Database migration for new instruction columns
- `Cargo.toml`: Updated async-openai dependency to 0.29

## TODO / Roadmap

### Recently Completed
- [x] **OpenAI Speech Style Instructions** - ✅ **COMPLETED**: Added support for controlling TTS speaking style
  - [x] Temporary per-message instructions: `\instruction text` and `[instruction] text`
  - [x] Persistent user-level instructions via `/set instruction`
  - [x] Database storage with 500-character limit
  - [x] Integration with OpenAI TTS API (gpt-4o-mini-tts model only)
  - [x] Fallback logic: temporary → user-level → guild-level → none
  - [x] Updated async-openai dependency to 0.29 for instructions parameter support

- [x] **OpenAI Voice Command Fix** - ✅ **COMPLETED**: Fixed "Invalid voice" errors
  - [x] Changed OpenAI TTS from premium to non-premium mode in `tts_core/src/structs.rs`
  - [x] Users now need to set TTS mode first: `/set mode OpenAI TTS (high quality)`
  - [x] Then set voice: `/set voice alloy` (or any OpenAI voice)
  - [x] Fixed compilation errors and container build issues

### High Priority
- [ ] **Disable "premium feature" warnings** - Remove or make optional the premium-only restrictions
  - Currently shows warnings when trying to use premium TTS modes
  - Consider making all features available in self-hosted version
  - Keep premium system for public bot only

- [ ] **Admin-level control on certain commands** - Implement role-based command permissions
  - Add permission checks for destructive commands
  - Server admin override for user settings
  - Command allowlist/denylist per role

- [ ] **Automatic sentiment adjustment** - Adjust TTS parameters based on message content
  - Analyze message sentiment/emotion
  - Adjust speaking rate for excitement/calmness
  - Select appropriate voice based on context
  - Consider punctuation and capitalization

### Medium Priority
- [ ] **Other TTS services with funny voices** - Re-enable and expand TTS service support
  - Re-enable gTTS, eSpeak, Polly, gCloud in codebase
  - Add novelty voices (robot, alien, cartoon characters)
  - Voice effects (echo, reverb, pitch shift)
  - Custom voice packs support

### Future Features  
- [ ] **STT (Speech-to-Text)** - Transcribe voice channel conversations
  - OpenAI Whisper integration for transcription
  - Real-time voice channel transcription to text
  - Voice commands for bot control
  - Meeting summaries and notes
  - Multi-language transcription support
  - Speaker identification

### Code Quality Improvements
- [ ] Reduce compilation warnings
- [ ] Add comprehensive test suite
- [ ] Improve error messages for users
- [ ] Add metrics/monitoring dashboard
- [ ] Optimize database queries

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