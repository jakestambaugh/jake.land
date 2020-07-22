FROM rust:1.45.0 as cargo-build

WORKDIR /usr/src/app

# Move the Cargo.toml/Cargo.lock files into the container first
COPY Cargo.toml .
COPY Cargo.lock .
COPY ./src src
RUN mkdir .cargo
# Run `cargo vendor` in a separate step to create a cache-able layer
RUN cargo vendor > .cargo/config

RUN cargo build --release
RUN cargo install --path . --verbose

# Create serving image
FROM debian:stable-slim

COPY --from=cargo-build /usr/local/cargo/bin/jakeland /bin
EXPOSE 8000

CMD ["jakeland"]