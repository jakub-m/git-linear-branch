build:
	cargo build
release:
	cargo build --release
clean:
	rm -rf target .git-linear-branch-meta.json
.phony: build release build



