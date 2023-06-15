FROM rust:1.65-alpine AS builder

RUN apk update && apk upgrade

RUN apk add --no-cache sqlite-dev musl-dev

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo install diesel_cli --no-default-features --features sqlite

ENV DATABASE_URL=file:///usr/src/shortify.db

WORKDIR /usr/src

COPY . .

RUN cargo install --path .

RUN diesel setup


FROM alpine

RUN apk update && apk upgrade

RUN apk add --no-cache sqlite-dev libgcc

COPY --from=builder /usr/local/cargo/bin/shortify /usr/local/bin/shortify

COPY --from=builder /usr/src/shortify.db /usr/local/shortify.db

ENV DATABASE_URL=file:///usr/local/shortify.db

EXPOSE 5000

CMD [ "shortify" ]
