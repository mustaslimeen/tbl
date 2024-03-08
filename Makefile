dev:
	cargo build

# usage: make run ARGS="...."
run:
	cargo run $(ARGS)

release:
	cargo build --release

check:
	cargo check
