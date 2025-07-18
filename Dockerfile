# syntax=docker/dockerfile:1.7
ARG RUST_VERSION=1.88
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-slim-bookworm AS builder
ARG TARGETARCH

# 1) Instalar toolchain/linker adecuado
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      apt-get update && \
      apt-get install -y --no-install-recommends gcc-aarch64-linux-gnu && \
      rustup target add aarch64-unknown-linux-gnu; \
    else \
      rustup target add x86_64-unknown-linux-gnu; \
    fi

WORKDIR /app

# 2) Caché de dependencias
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      cargo build --release --target aarch64-unknown-linux-gnu; \
    else \
      cargo build --release --target x86_64-unknown-linux-gnu; \
    fi
RUN rm -rf src

# 3) Código de la app
COPY src       ./src
COPY templates ./templates
COPY static    ./static
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      cargo build --release --target aarch64-unknown-linux-gnu; \
    else \
      cargo build --release --target x86_64-unknown-linux-gnu; \
    fi

# 4) Recolectar binario final en /app/dist
RUN mkdir -p /app/dist && \
    if [ "$TARGETARCH" = "arm64" ]; then \
      cp target/aarch64-unknown-linux-gnu/release/wira_page /app/dist/wira_page; \
    else \
      cp target/x86_64-unknown-linux-gnu/release/wira_page /app/dist/wira_page; \
    fi

# 5) Imagen final minimalista
FROM gcr.io/distroless/cc-debian12 AS runtime
EXPOSE 8080
HEALTHCHECK --interval=5s --timeout=2s --retries=3 \
  CMD curl -fsS http://127.0.0.1:8080/health || exit 1
COPY --from=builder /app/dist/wira_page /usr/local/bin/wira_page
ENTRYPOINT ["/usr/local/bin/wira_page"]
