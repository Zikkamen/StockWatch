FROM rust:1.80

COPY . .

RUN cargo install --path .

CMD ["stockwatch"]