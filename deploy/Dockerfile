FROM rust:1.81-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
RUN apt-get install -y nodejs   

COPY . .
RUN cargo build --release
RUN strip target/release/heavy-metal-notifier 

FROM gcr.io/distroless/cc-debian12 AS release
WORKDIR /app

COPY --from=builder /app/target/release/heavy-metal-notifier .

EXPOSE 7125

CMD ["./heavy-metal-notifier"]