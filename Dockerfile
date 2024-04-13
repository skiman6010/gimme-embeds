FROM rust:latest as builder
WORKDIR /usr/src/gimme-embeds
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/gimme-embeds /usr/local/bin/gimme-embeds
CMD ["gimme-embeds"]
