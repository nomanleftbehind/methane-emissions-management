#!/bin/sh
trunk build --release client/index.html
cargo run --release --bin emissions_app_server -- --dir client/dist/./
