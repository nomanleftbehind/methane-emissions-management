#!/bin/sh
cargo watch -s "trunk build client/index.html && cargo run --bin emissions_app_server -- --dir client/dist"