# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Discord TTS Bot written in Rust using Serenity, Songbird, and Poise. Multi-workspace Cargo project with modular architecture.

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
- Fill out PostgreSQL connection details and Discord bot token

### Docker Development
- `docker-compose up --build -d` - Build and run containers
- `docker-compose logs bot` - View bot logs

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
- **Premium System**: Role-based premium features with subscription validation
- **Analytics**: Background collection and processing of usage metrics
- **Webhook Logging**: Discord webhook integration for error and event logging

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