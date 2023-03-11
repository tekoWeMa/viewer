#!/usr/bin/env sh
cargo run --release -- \
	--host localhost \
	--port 4223 \
	--uid Hit \
	--path ./data/HelloWorld.txt
