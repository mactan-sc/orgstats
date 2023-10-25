FROM rust:1.70

WORKDIR /diesel-learn

RUN cargo install diesel_cli --no-default-features --features postgres