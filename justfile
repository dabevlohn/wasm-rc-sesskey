alias g := genkeys
alias w := build-web
alias b := build-back
alias d := b64uri-decode
alias r := rsa-decrypt

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

# Decode Base64URI encoded string
b64uri-decode:
	cargo run --bin decode encrypted.key An4HJOZxYdS5_wn_sXL1jqLZZKUh0Unl6NjDpk1IC4OQgV3veuJGIe2fAOmBArXgWeve-1BVjtm1O8sp_MiAbb6uQITC-5ibkfSJc6lRloxrn_GA8edXtcrkckLB8KP2UvmtzaLOSBALh3Nm90FtdFaS7RfZMQ8TS4BXWcHrNJQ

# Decrypt session key
rsa-decrypt:
	openssl pkeyutl -decrypt -inkey private_pkcs8.pem -in encrypted.key
