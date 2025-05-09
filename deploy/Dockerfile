FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY ./web .
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install -y nodejs
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./web .
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS release
LABEL org.opencontainers.image.source=https://github.com/reaper47/heavy-metal-notifier
LABEL org.opencontainers.image.description=" Be notified whenever new heavy metal releases are around the corner."
LABEL org.opencontainers.image.licenses="AGPL-3.0"
WORKDIR /app
COPY --from=builder /app/target/release/heavy-metal-notifier .

CMD ["/app/heavy-metal-notifier"]
