FROM rust:1.62-alpine

RUN apk upgrade --update-cache --available && \
    apk add openssl && \
    rm -rf /var/cache/apk/*


# RUN apt-get update -y && apt-get upgrade -y
# RUN apt-get install cmake -y
# RUN apk add cm

WORKDIR /usr/src/rusttracks-recorder

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rusttracks-recorder"]