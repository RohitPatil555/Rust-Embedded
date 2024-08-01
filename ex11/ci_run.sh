#!/bin/bash

set +e

cargo fmt

cargo clean

cargo clippy

cargo nextest run -p logger --features test-utils

cargo nextest run -p pool_allocator --features test-utils

cargo build --target "thumbv7em-none-eabihf"

cargo run --target "thumbv7em-none-eabihf" &

#killall qemu-system-arm
