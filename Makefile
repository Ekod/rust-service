ch:
	cargo fmt
	cargo clippy
au:
	cargo audit
r:
	RUST_LOG=trace cargo run