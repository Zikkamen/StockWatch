FROM rust:1.80 as builder

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder ./target/release/stockwatch ./target/release/stockwatch

CMD  ["./target/release/stockwatch"]