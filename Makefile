.PHONY: build
build:
	cargo build

.PHONY: clean
clean:
	rm -rf target

.PHONY: lint
lint:
	cargo clippy -- -D warnings
