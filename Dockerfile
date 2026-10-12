# syntax=docker/dockerfile:1
# ---- Build stage: full Rust toolchain (~1.5 GB, discarded afterwards) ----
FROM rust:1-bookworm AS build
WORKDIR /app
COPY . .
# Cache mounts keep the crate download cache and compiled dependencies between
# builds, so rebuilding after a code change takes seconds instead of minutes.
# `target` is a cache mount (not part of the image), so copy the binary out.
# https://docs.docker.com/build/cache/optimize/#use-cache-mounts
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && cp target/release/tfl4rust /tfl4rust

# ---- Runtime stage: tiny Debian with just the binary (~30 MB total) ----
# Same Debian version as the build stage, so the glibc the binary was linked
# against is present.
FROM debian:bookworm-slim
# CA certificates so the app can make HTTPS requests to other services.
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN useradd --system --uid 10001 app
COPY --from=build /tfl4rust /usr/local/bin/tfl4rust
USER app
ENV PORT=8080
EXPOSE 8080
CMD ["tfl4rust"]
