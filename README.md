# Source Code Parser

## source-code-parser
A library for getting the analysis context and AST of source code. Languages supported are those supported by v0.0.18 of the `rust-code-analysis` crate.

### Run Tests

#### Rust Tests
1. Have the nightly toolchain installed: `rustup toolchain install nightly-x86_64-unknown-linux-gnu`
2. Run `cargo +nightly test --lib`


#### Postman Tests

These tests reproduce and verify the results of the initial ReSSA paper, "Advancing static code analysis with language-agnostic component identification". To run these tests:
1. Clone the following additional required repositories:
	* DeathStarBench (https://github.com/delimitrou/DeathStarBench), using commit b509c933faca3e5b4789c6707d3b3976537411a9
    * TrainTicket (https://github.com/FudanSELab/train-ticket), using commit a4ed2433b0b6ab6e0d60115fc19efecb2548c6cd
1. Import the Postman collection
1. Set the following environment variables:
    * DSB_ROOT: Path to the root directory of DeathStarBench
    * TT_ROOT: Path to the root directory of TrainTicket
1. Build and run the project on port 8080. You can now run the tests
1. Since TrainTicket's parsing takes longer than Postman's default bulk timeout, it is best to run the tests individually. The test script attached to each request automatically runs a deep equality check on the returned values against cached versions of our results.


### Run Benchmarks
1. Install the `perf` Linux utility program, `jemalloc` and `libc` on your machine
1. Have the nightly toolchain installed: `rustup toolchain install nightly-x86_64-unknown-linux-gnu`
1. Initialize the target folder so flamegraphs can get created appropriately by running `benches/init.sh`
1. Run `cargo +nightly bench`.

__As of December 2021 there appears to be issues building the `jemalloc` crate with recent installations of jemalloc, which may hinder your ability to run the benchmarks__

**Note that it may take a significant amount of time to run benchmarks as they go through many iterations**

Criterion and Iai will generate output to stdout and additionally files in `target/criterion/**` for the various benchmarks, and each benchmark also has a `flamegraph.svg` in its output directory.

Benchmarks run in alphabetical order of the filenames in the `benches` folder, so the output will first show the regular Criterion benchmarks, then those for Iai, and then the memory benchmarking for ReSSA which also outputs flamegraphs.

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
>Example: `RUST_LOG=info cargo run -p source-code-parser-web -- --host 0.0.0.0 --port 3000`.     
 
By default, the server will listen on `http://localhost:8080`.
