FROM rust:1.62

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install cmake -y

WORKDIR /usr/src/rusttracks-recorder

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/rusttracks-recorder"]