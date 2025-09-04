#!/bin/bash
cd agent-template
cargo build --target wasm32-unknown-unknown --release
cd ..
cp ./target/wasm32-unknown-unknown/release/agent_template.wasm ./host/src/agent_template.wasm
cd host
cargo run
