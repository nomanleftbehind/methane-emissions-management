#!/bin/sh
trunk build --release client/index.html
cargo run --release --bin methane_emissions_management_server -- --dir client/dist/./
