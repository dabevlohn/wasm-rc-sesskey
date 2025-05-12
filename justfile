# Print all recipes
dummy:
	just -l

# Build for frontend
build-web:
	wasm-pack build --target web

# Build for bakend
build-back:
	wasm-pack build --target nodejs
