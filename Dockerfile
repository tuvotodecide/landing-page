# syntax=docker/dockerfile:1.7
ARG RUST_VERSION=1.88
#1.BUILD
FROM --platform=$BUILDPLATFORM rust:${RUST_VERSION}-slim-bookworm AS builder
ARG TARGETARCH
ARG BUILDPLATFORM

WORKDIR /app

RUN if [ "$TARGETARCH" = "arm64" ]; then \
        apt-get update && \
        dpkg --add-architecture arm64 && \
        apt-get update && \
        apt-get install -y --no-install-recommends \
            gcc-aarch64-linux-gnu \
            binutils-aarch64-linux-gnu \
            libc6-dev-arm64-cross && \
        rm -rf /var/lib/apt/lists/* && \
        rustup target add aarch64-unknown-linux-gnu && \
        mkdir -p .cargo && \
        echo "[target.aarch64-unknown-linux-gnu]" >> .cargo/config.toml && \
        echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml; \
    fi

COPY Cargo.toml Cargo.lock ./

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    mkdir -p src && \
    echo 'fn main() {}' > src/main.rs && \
    CARGO_BUILD_TARGET=$( [ "$TARGETARCH" = "amd64" ] && echo "x86_64-unknown-linux-gnu" || echo "aarch64-unknown-linux-gnu" ) && \
    cargo fetch --target $CARGO_BUILD_TARGET && \
    rm -rf src

COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    CARGO_BUILD_TARGET=$( [ "$TARGETARCH" = "amd64" ] && echo "x86_64-unknown-linux-gnu" || echo "aarch64-unknown-linux-gnu" ) && \
    cargo build --release --target $CARGO_BUILD_TARGET --locked && \
    mkdir -p /output && \
    mv /app/target/$CARGO_BUILD_TARGET/release/landing_tuvotodecide /output/app

# RUNTIME
FROM --platform=$TARGETPLATFORM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY templates ./templates
COPY static    ./static
COPY i18n    ./i18n

COPY --from=builder /output/app /usr/local/bin/app

EXPOSE 8080
CMD ["app"]