FROM rust:1.72-alpine as build
WORKDIR /app

RUN apk add pkgconfig musl-dev libressl-dev

COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=build /app/target/release/qbittorrent-connection-monitor /app/qbittorrent-connection-monitor

CMD ["/app/qbittorrent-connection-monitor"]