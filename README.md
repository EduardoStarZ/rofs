# Rust Online File System (ROFS)

This is a simple Rust repository created to contain a small and simple HTTP Server that display a simple list view of files and folders on a web page. This is not meant to be accessible outside a local network due to safety considerations.

## SSL Certificates

This servers runs HTTPS primarily with self signed certificates (since there is no need for external access outside a network), but a proper SSL certificate can be added if needed. Make sure that the key.pem and cert.pem files are in the directory before build (in case you are building the docker image) or during runtime (if you are running this locally)

# How to build and run

## Requirements

- `rustc` version 1.95.0 or higher
- `openssl` version 2.x or higher
- `docker` (any version) if you are running this on a container
- `musl` version 1.2.x or higher

## Building

To build the package, ensure `cargo` and `rustc` are installed and configured properly.

### Local setup 

If you are building this for local use, you can simply run `cargo build --release` to build. Ensure that `musl` is installed, or else the compile process for the `openssl-sys` crate will fail.

### Docker setup

Install the `x86_64-unknown-linux-musl` as a target with rustup (`rustup target add x86_64-unknown-linux-musl`) and run `cargo build --release --target x86_64-unknown-linux-musl` or run the script in `build.sh` to compile rofs.

After that, if the compile process was successfull, a binary will on `target/x86_64-unknown-linux-musl/release/rofs` you can run `docker build -t rofs .` or run the script on `docker-build.sh`. Be mindfull that if you recompile the code with any changes and restart the image creation process you will need to remove the current image with name rofs with `docker rm rofs` (the image cannot be running, so if it is, you will need to use `docker stop rofs` to stop the execution of rofs). Keep in mind that the script for building automatically does all of this, so it is recommended to use that instead of manually running the docker commands.


When the image is created, you can now run the image with `docker run --name rofs rofs:latest`. Keep in mind that rofs has a declared docker volume with name `/static`. If you wish to mount this volume on the running machine, pass the `--volume="/path/to/location:/static"` argument for `docker run`. All of this setup can be done with the `docker-run.sh` script, which will automatically:

- Assign name `rofs` for the running image.
- Bind port 4000 for rofs.
- Setup the `/static` volume for the `/opt/rofs/static/` directory.
- Detach from terminal once ran.
- Always restart the process unless `docker stop rofs` is specifically ran.
- Always start rofs whenever docker initializes.
