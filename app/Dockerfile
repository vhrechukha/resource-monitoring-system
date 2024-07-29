FROM rust:latest

WORKDIR /usr/src/app

COPY app/ ./

RUN cargo build --release

CMD ["cargo", "run", "--release"]
