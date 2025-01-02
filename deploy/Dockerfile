FROM rust:1.81-bookworm AS builder
WORKDIR /app

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get update && apt-get install -y nodejs

COPY ./web .
COPY . .
RUN cargo build --release
RUN strip target/release/heavy-metal-notifier 

FROM gcr.io/distroless/cc-debian12 AS release
LABEL org.opencontainers.image.source=https://github.com/reaper47/heavy-metal-notifier
WORKDIR /app

COPY --from=builder /app/target/release/heavy-metal-notifier .

EXPOSE 7125

CMD ["./heavy-metal-notifier"]
