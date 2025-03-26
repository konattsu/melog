FROM rust

RUN apt update && apt install -y \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/melog"]
