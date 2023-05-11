FROM rust:alpine as builder

WORKDIR /server
COPY . .

RUN apk add protoc protobuf-dev musl-dev
RUN cargo build --release --bin server

FROM rust:alpine

COPY --from=builder /server/target/release/server /server/server

CMD /server/server 