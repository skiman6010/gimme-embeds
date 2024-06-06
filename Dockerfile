FROM rust:1-bookworm as builder
WORKDIR /usr/src/gimme-embeds
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
WORKDIR /app
RUN apt update && apt install -y libssl3 ca-certificates
COPY --from=builder /usr/local/cargo/bin/gimme-embeds /usr/local/bin/gimme-embeds
CMD ["gimme-embeds"]
