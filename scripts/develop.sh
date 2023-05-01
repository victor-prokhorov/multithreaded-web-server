#!/bin/sh

cargo watch -c -s 'cargo check && cargo run -- --tracing-level trace --server-addr localhost:3000 --server-thread-pool-size 4'