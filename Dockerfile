ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS builder

# RUN apt-get update -y && apt-get upgrade -y
# RUN apt-get install cmake openssl -y
# RUN rustup target add $TARGET

# WORKDIR /app

COPY --chown=rust:rust . .

#RUN cargo build --release --target $TARGET
RUN cargo build --release

RUN pwd


FROM alpine:3.12

ENV RUST_BACKTRACE=full

# RUN apk update --no-cache && apk add --no-cache sqlite sqlite-libs


WORKDIR /app
#COPY --from=builder /app/target/$TARGET/release/rusttracks-recorder ./
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rusttracks-recorder ./

CMD ["./rusttracks-recorder"]