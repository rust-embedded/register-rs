default: check
	cargo build

check:
	cargo run --example=main
	cargo run --example=deref

clippy:
	cargo clippy

fmt:
	cargo fmt

ready: clippy fmt check
	git pull
	cargo package --allow-dirty

clean:
	rm -rf target
