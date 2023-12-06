FROM rust:1.73-slim as builder

WORKDIR /app

COPY . .

RUN cargo build

CMD ["cargo", "run"]

