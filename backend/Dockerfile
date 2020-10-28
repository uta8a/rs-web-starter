FROM rust:1.47 AS builder

WORKDIR /sandbox
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
COPY .env.dev .env
COPY ./wait-db.sh ./wait-db.sh
RUN rm -f target/release/deps/backend*
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libpq-dev

RUN cargo build --release

FROM debian:10.6
WORKDIR /main
ENV RUST_BACKTRACE 1
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y libpq-dev netcat
COPY --from=builder /sandbox/target/release/backend /usr/local/bin/backend
COPY --from=builder /sandbox/wait-db.sh /wait-db.sh
RUN chmod +x /wait-db.sh
COPY --from=builder /sandbox/.env /main/.env
CMD ["/wait-db.sh"]