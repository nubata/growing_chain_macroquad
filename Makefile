.PHONY: build clean run

SOURCES = src/main.rs src/model.rs
WASMS = target/wasm32-unknown-unknown/release/growing_chain_macroquad.wasm
TARGET = docs/main.wasm

build: $(TARGET)

$(TARGET): $(WASMS)
	cp $^ $@

$(WASMS): $(SOURCES)
	cargo build --target wasm32-unknown-unknown --profile release

clean:
	cargo clean
	rm -f $(TARGET)

run: build
	cargo install basic-http-server
	basic-http-server docs
