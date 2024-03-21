FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef

WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . . 
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates pkg-config \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/axum_tailwind_template axum_tailwind_template 
COPY config config
COPY assets assets
COPY templates templates
ENV APP_ENVIRONMENT production 
ENTRYPOINT ["./axum_tailwind_template]
