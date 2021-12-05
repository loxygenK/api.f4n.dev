FROM rust:1.56-buster

RUN mkdir -p app/src
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN echo "// dummy" > ./src/lib.rs

RUN cargo build --release

COPY . /app

RUN cargo build --release

CMD "./target/release/flisan_portfolio_api"
