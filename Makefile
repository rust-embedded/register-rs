default: check
	cargo build

check:
	cd example && cargo build

clippy:
	cargo clippy

fmt:
	cargo fmt

ready: clippy fmt check
	git pull
	cargo package --allow-dirty

clean:
	rm -rf target
