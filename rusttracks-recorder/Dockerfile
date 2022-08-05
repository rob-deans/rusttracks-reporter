ARG BASE_IMAGE=messense/rust-musl-cross:armv7-musleabihf

FROM ${BASE_IMAGE} AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:3.12

ENV RUST_BACKTRACE=full

WORKDIR /app
COPY --from=builder /app/target/armv7-unknown-linux-musleabihf/release/rusttracks-recorder ./

CMD ["./rusttracks-recorder"]