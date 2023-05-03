# syntax=docker/dockerfile:1

FROM rust:latest
WORKDIR /
COPY . .
RUN cargo build --release

CMD ["target/release/aps-alerts"]