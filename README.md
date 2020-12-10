# Source Code Parser

## source-code-parser
A library for getting the AST of a source code file wrapping around `rust-code-analysis`. Languages supported are those supported by v0.0.18 of the `rust-code-analysis` crate.

### Run Tests
1. Run `cargo test --lib`

## source-code-parser-web
A REST API for getting the AST of a source code file.

To get the AST of a source code file, you should make a `POST` request to the `/ast` endpoint with the `Content-Type: multipart/form-data` header and the file data for the `file` key, and the file extension type for the uploaded file such as `rs`, `py`, `cpp`, etc. The endpoint will return a JSON response containing the AST.

### Build & Run
1. Have the Rust nightly toolchain installed.
2. Run `cargo run -p source-code-parser-web --release`