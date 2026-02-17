# Pin the Rust toolchain version used in the build stage.
ARG RUST_VERSION=1.92

# Name of the compiled binary produced by Cargo (must match Cargo.toml package name).
ARG APP_NAME=catch


# Use Debian-based Rust image for building (glibc) to get prebuilt V8 binaries
FROM docker.io/library/rust:${RUST_VERSION}-slim-bookworm AS build

# Re-declare args inside the stage if you want to use them here.
ARG APP_NAME

# All build steps happen inside /app.
WORKDIR /app

# Install build dependencies needed to compile Rust crates on Debian
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    curl \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Build the application with glibc target (fast V8 prebuilt download)
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=../../Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=../../Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=../../assets,target=assets \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release && \
    cp ./target/release/$APP_NAME /bin/catch

FROM docker.io/library/debian:bookworm-slim AS final

# Create a non-privileged user (recommended best practice)
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Drop privileges for runtime.
USER appuser

# Copy only the compiled binary from the build stage.
COPY --from=build /bin/catch /bin/

# Document the port your app listens on.
EXPOSE 8111

# Start the application.
CMD ["/bin/catch"]
