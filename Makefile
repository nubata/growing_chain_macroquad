.PHONY: build clean run

SOURCES = src/main.rs src/model.rs
TARGET = docs/main.wasm

RUST_PACKAGE = $(shell basename $$(pwd))
RUST_PROFILE = release
RUST_TARGET = wasm32-unknown-unknown
RUST_WASM = target/$(RUST_TARGET)/$(RUST_PROFILE)/$(RUST_PACKAGE).wasm

build: $(TARGET)

$(RUST_WASM): $(SOURCES)
	cargo build --target $(RUST_TARGET) --profile $(RUST_PROFILE)

$(TARGET): $(RUST_WASM)
	cp -f $(RUST_WASM) $(TARGET)

clean:
	cargo clean
	rm -f $(TARGET)

run: build
	cargo install basic-http-server
	(sleep 1; open http://127.0.0.1:4000/) & basic-http-server docs
