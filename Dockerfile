# --- Stage 1: Build the application ---
FROM rust:1.93.0 AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /usr/src/rofs
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN pwd

# --- Stage 2: Create the minimal runtime image ---
FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/rofs ./rofs
RUN mkdir static
CMD ["./rofs"]
