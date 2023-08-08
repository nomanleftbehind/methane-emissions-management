FROM rust:latest as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
RUN rustup target add wasm32-unknown-unknown
RUN apt-get update
RUN apt-get install -y curl build-essential libssl-dev cmake pkg-config openssl binaryen
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli
WORKDIR /mem

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /mem/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --bin methane_emissions_management_server --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN trunk build --release client/index.html
RUN cargo build --release --bin methane_emissions_management_server

# Create a new stage with a minimal image
FROM scratch
COPY --from=builder /mem/target/release/methane_emissions_management_server /methane_emissions_management_server
# ENTRYPOINT ["/methane_emissions_management_server"]
CMD ["./methane_emissions_management_server --dir client/dist"]
EXPOSE 8081






# FROM rust:latest as builder

# COPY . .

# RUN rustup target add wasm32-unknown-unknown
# RUN apt-get update
# RUN apt-get install -y curl build-essential libssl-dev cmake pkg-config openssl binaryen
# RUN cargo install --locked trunk
# RUN cargo install --locked wasm-bindgen-cli

# ENV SQLX_OFFLINE true

# RUN trunk build client/index.html
# RUN cargo build --bin methane_emissions_management_server


# #  --target x86_64-unknown-linux-musl

# FROM scratch
# COPY --from=builder /target/release/methane_emissions_management_server /methane_emissions_management_server
# # COPY configuration configuration
# ENTRYPOINT [ "./methane_emissions_management_server" ]
# # WORKDIR /app
# # CMD ["./methane_emissions_management_server"]
# EXPOSE 8081