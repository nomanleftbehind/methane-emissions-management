#!/bin/sh
cargo watch -s "trunk build client/index.html && cargo run --bin methane_emissions_management_server -- --dir client/dist"