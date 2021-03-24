FROM rust:1.45

WORKDIR /usr/src/source-code-parser
COPY . .
RUN cargo install --path source-code-parser-web

ENTRYPOINT [ "./target/release/source-code-parser-web", "--host", "0.0.0.0" ]