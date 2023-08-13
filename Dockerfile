FROM rust:latest as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update
RUN apt-get install -y curl build-essential libssl-dev cmake pkg-config openssl binaryen
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli
WORKDIR /app

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --bin methane_emissions_management_server --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN trunk build --release client/index.html
RUN cargo build --release --bin methane_emissions_management_server

# Create a new stage with a minimal image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/methane_emissions_management_server methane_emissions_management_server
COPY --from=builder /app/client/dist client/dist
COPY configuration configuration

# RUN apt-get update && apt install -y openssl

# CMD ["ls", "-la"]
ENV APP_ENVIRONMENT production
ENTRYPOINT ["/bin/bash", "-c", "configuration/start_container.sh"]
EXPOSE 8081