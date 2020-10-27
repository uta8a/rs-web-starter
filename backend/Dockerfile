FROM rust:1.47 AS builder

WORKDIR /sandbox
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
# COPY .env .env
# COPY migrations migrations
RUN rm -f target/release/deps/backend*
RUN cargo build --release

FROM debian:10.6
WORKDIR /main
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libpq-dev
COPY --from=builder /sandbox/target/release/backend /usr/local/bin/backend
# COPY --from=builder /sandbox/.env /main/.env
CMD ["backend"]