.PHONY: examples

examples:
	cargo build --target=wasm32-wasi --example=hello_world
