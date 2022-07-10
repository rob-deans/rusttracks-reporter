FROM rust:1.62-alpine

WORKDIR /rusttracks-recorder
COPY . .

RUN apk add --no-cache~=3.39.0

RUN cargo install --path .

CMD ["rustracks-recorder"]
