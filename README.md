# Source Code Parser

## source-code-parser
A library for getting the analysis context and AST of source code. Languages supported are those supported by v0.0.18 of the `rust-code-analysis` crate.

### Run Tests
1. Run `cargo test --lib`

## source-code-parser-web
A REST API for getting the Analysis Context of a project and raw AST of a source code file.

To get the analysis context of a project, To get the AST of a source code file, you should make a `POST` request to the `/ctx` endpoint with a JSON body like so:
```json
{
    "path": "/directory/path",
    "files": [
        "/directory/path/file1.cpp",
        "/directory/path/file2.cpp"
    ],
    "subDirectories": [
        {
            "path": "/directory/path/sub",
            "files": [
                "/directory/path/sub/other.java",
                "/directory/path/sub/another.java"
            ],
            "subDirectories": []
        }
    ]
}
```
All source code files that you want to be parsed should be included in the respective files array for that directory. The endpoint will return a JSON response containing the analysis context.

To get the AST of a source code file, you should make a `POST` request to the `/ast` endpoint with a JSON body like `{"file_path": "/path/to/source/file.cpp"}` The endpoint will return a JSON response containing the AST.

### Build & Run
1. Run `cargo run -p source-code-parser-web`

Optionally, you can specify the host and port to run on using command line arguments.    
>Example: `cargo run -p source-code-parser-web -- --host 0.0.0.0 --port 3000`.     
 
By default, the server will listen on `http://localhost:8080`.