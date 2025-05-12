alias g := genkeys
alias w := build-web
alias b := build-back

# Print all recipes
dummy:
	just -l

# Generate RSA-keys
genkeys:
	openssl genrsa -out private_pkcs8.pem 1024
	openssl rsa -in private_pkcs8.pem -pubout -traditional > public_key.pem

# Build for frontend
build-web:
	wasm-pack build --target web

# Build for bakend
build-back:
	wasm-pack build --target nodejs
