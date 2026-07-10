FROM rust:1.96.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION

LABEL org.opencontainers.image.title="fyltime" \
      org.opencontainers.image.description="Search files by modification time" \
      org.opencontainers.image.url="https://MSHF0403.github.io/fyt/" \
      org.opencontainers.image.source="https://github.com/MSHF0403/fyt" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.licenses="MIT"

COPY --from=builder /app/target/release/fyltime /usr/local/bin/fyt

WORKDIR /work
ENTRYPOINT ["fyt"]