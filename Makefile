.PHONY: bench
bench:
	RUST_LOG=debug cargo run --release
