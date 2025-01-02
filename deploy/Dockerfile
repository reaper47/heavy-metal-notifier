FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY ./web .
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get update && apt-get install -y nodejs
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin app

FROM gcr.io/distroless/cc-debian12 AS release
LABEL org.opencontainers.image.source=https://github.com/reaper47/heavy-metal-notifier
COPY --from=builder /app/target/release/app /usr/local/bin

EXPOSE 7125

CMD ["/usr/local/bin/app"]
