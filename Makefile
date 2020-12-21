build:
	@wasm-pack build --target nodejs

run: build
	@node ./test.js