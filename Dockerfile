FROM rust:1.80

EXPOSE 5432

COPY . .

RUN cargo build --release

CMD  ["./target/release/stockwatch"]