build:
	cargo build
release:
	cargo build --release
clean:
	rm -rf target
.phony: build release build
