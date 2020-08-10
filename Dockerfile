FROM rust:1.45.0 as cargo-build

WORKDIR /usr/src/app

# Move the Cargo.toml/Cargo.lock files into the container first
COPY Cargo.toml .
COPY Cargo.lock .
COPY ./src src

RUN cargo build --release
RUN cargo install --path . --verbose

# Create serving image
FROM debian:stable-slim

# Move static assets into the same directory as the binary
COPY ./static/ /bin/static
COPY --from=cargo-build /usr/local/cargo/bin/jakeland /bin

CMD ROCKET_PORT=$PORT /bin/jakeland