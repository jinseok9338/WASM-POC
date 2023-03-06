FROM rust:1.55-slim

WORKDIR /app

RUN rustup default nightly && \
    cargo install cargo-watch

COPY . .


ENV PORT=8080
EXPOSE 8080

CMD ["cargo", "watch", "-x", "run"]

