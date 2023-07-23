FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown
RUN apt-get update
RUN apt-get install -y curl build-essential libssl-dev cmake pkg-config openssl binaryen
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli

WORKDIR /usr/src/fullstackrust
COPY . .

RUN cd client && trunk build --release
# RUN trunk build --release client/index.html
RUN cargo build --release --bin methane_emissions_management_server

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/fullstackrust/target/release /usr/local/bin

COPY --from=build /usr/src/fullstackrust/client/dist /usr/local/bin/dist

WORKDIR /usr/local/bin 

# RUN cargo install --path .

# CMD ["methane_emissions_management_server"]