FROM alpine:latest
COPY ./target/x86_64-unknown-linux-musl/release/rofs .
VOLUME /static
RUN mkdir -p /certs
COPY ./certs/cert.pem /certs/cert.pem
COPY ./certs/key.pem /certs/key.pem
EXPOSE 4000
CMD ["./rofs"]
