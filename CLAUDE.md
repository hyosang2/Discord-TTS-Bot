# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Discord TTS Bot written in Rust using Serenity, Songbird, and Poise. Multi-workspace Cargo project with modular architecture. Currently configured for OpenAI TTS only - other TTS services (gTTS, eSpeak, Polly, gCloud) are temporarily disabled.

### Recent Updates
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
- **Important**: Remove `setup = true` from config.toml after initial setup to prevent migration conflicts

### Docker Development
- `docker compose up --build -d` - Build and run containers
- `docker compose logs bot` - View bot logs
- Note: TTS service is temporarily disabled in docker-compose.yml

### Common Docker Issues
- **"relation 'guilds' does not exist"**: Recurring database migration issue
  - Root cause: Corrupted database volume or interrupted migrations
  - Quick fix: `sudo docker compose down -v && sudo docker compose up --build -d`
  - Always remove `setup = true` from config.toml after initial setup
  - Use PostgreSQL health check in docker-compose.yml for proper startup sequencing

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
- **Voice**: Songbird integration for voice channel management and TTS playback
- **Database**: PostgreSQL with sqlx for persistent storage (guilds, users, voice settings)
- **TTS Integration**: OpenAI TTS API for text-to-speech synthesis (default mode)
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

## TODO / Roadmap

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