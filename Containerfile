FROM docker.io/rust:1-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM docker.io/debian:bookworm
STOPSIGNAL SIGINT
RUN apt update && apt install -y ca-certificates && apt clean
COPY --from=builder /build/target/release/mijn-host-ddns /usr/bin/mijn-host-ddns
ENTRYPOINT ["mijn-host-ddns"]
