FROM docker.io/library/rust:slim-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release

FROM docker.io/library/debian:bookworm-slim
STOPSIGNAL SIGINT
RUN apt update && apt install -y ca-certificates && apt clean
COPY --from=builder /build/target/release/mijn-host-ddns /usr/bin/mijn-host-ddns
ENTRYPOINT ["mijn-host-ddns"]
