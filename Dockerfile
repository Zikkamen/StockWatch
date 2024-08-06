FROM rust:1.80 as builder

COPY . .

RUN cargo build --release

FROM rust:1.80 as runtime

COPY --from=builder ./credentials ./credentials
COPY --from=builder ./Stocklist.txt ./Stocklist.txt
COPY --from=builder ./target/release/stockwatch ./target/release/stockwatch

CMD ["./target/release/stockwatch"]