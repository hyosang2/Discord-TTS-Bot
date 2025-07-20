FROM rustlang/rust:nightly-bookworm as builder
ENV RUSTFLAGS="--cfg tokio_unstable"

WORKDIR /bot

RUN apt-get update && apt-get install -y cmake libopus-dev && apt-get clean

# Copy Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY tts_core/Cargo.toml ./tts_core/
COPY tts_commands/Cargo.toml ./tts_commands/
COPY tts_events/Cargo.toml ./tts_events/
COPY tts_migrations/Cargo.toml ./tts_migrations/
COPY tts_tasks/Cargo.toml ./tts_tasks/

# Create dummy source files to allow dependency compilation
RUN mkdir -p src tts_core/src tts_commands/src tts_events/src tts_migrations/src tts_tasks/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "// dummy" > tts_core/src/lib.rs && \
    echo "// dummy" > tts_commands/src/lib.rs && \
    echo "// dummy" > tts_events/src/lib.rs && \
    echo "// dummy" > tts_migrations/src/lib.rs && \
    echo "// dummy" > tts_tasks/src/lib.rs

# Build dependencies
RUN cargo build --release && rm -rf src tts_*/src

# Copy actual source code
COPY . .
RUN cargo build --release

# Now make the runtime container
FROM debian:bookworm-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y ca-certificates mold && rm -rf /var/lib/apt/lists/*

COPY --from=builder /bot/target/release/tts_bot /usr/local/bin/discord_tts_bot

CMD ["/usr/local/bin/discord_tts_bot"]
