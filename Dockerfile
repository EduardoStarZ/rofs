# --- Stage 2: Create the minimal runtime image ---
FROM alpine:latest
COPY ./target/x86_64-unknown-linux-musl/release/rofs .
VOLUME /static
COPY ./cert.pem /cert.pem
COPY ./key.pem /key.pem
CMD ["./rofs"]
