.DEFAULT_TARGET: all

.PHONY: all
all: build
	./target/debug/life

.PHONY: build
build:
	cargo build

.PHONY: clean
clean:
	rm -rf target

.PHONY: lint
lint:
	cargo clippy -- -D warnings

.PHONY: lint-all
lint-all:
	cargo clippy -- \
		-D clippy::correctness \
		-D clippy::complexity \
		-D clippy::pedantic \
		-D clippy::nursery \
		-D clippy::perf \
		-D clippy::cargo \
		-D clippy::restriction
