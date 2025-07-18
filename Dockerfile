FROM rustlang/rust:nightly-bookworm as builder
ENV RUSTFLAGS="-C target-cpu=skylake --cfg tokio_unstable"

WORKDIR /bot

RUN apt-get update && apt-get install -y cmake && apt-get clean

COPY . .
RUN cargo build

# Now make the runtime container
FROM debian:bookworm-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y ca-certificates mold && rm -rf /var/lib/apt/lists/*

COPY --from=builder /bot/target/debug/tts_bot /usr/local/bin/discord_tts_bot

CMD ["/usr/local/bin/discord_tts_bot"]
