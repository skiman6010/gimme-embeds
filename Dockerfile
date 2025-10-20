FROM rust:1-trixie as builder
WORKDIR /usr/src/gimme-embeds
COPY . .

RUN cargo install --path .

FROM debian:trixie-slim
WORKDIR /app
RUN apt update && apt upgrade -y && apt install -y libssl3 ca-certificates
COPY --from=builder /usr/local/cargo/bin/gimme-embeds /usr/local/bin/gimme-embeds
CMD ["gimme-embeds"]
