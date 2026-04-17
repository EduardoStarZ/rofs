# --- Stage 2: Create the minimal runtime image ---
FROM alpine:latest
COPY ./target/release/rofs .
VOLUME /static
# COPY cert.pem /cert.pem
# COPY key.pem /key.pem
CMD ["./rofs"]
