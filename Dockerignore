FROM docker.io/rust:1.89-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM docker.io/debian:bookworm-slim
RUN apt update && apt install -y ca-certificates && apt clean
COPY --from=builder /build/target/release/mijn-host-ddns /usr/bin/mijn-host-ddns
STOPSIGNAL SIGINT
ENTRYPOINT ["mijn-host-ddns"]
