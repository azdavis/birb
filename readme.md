# birb

A programming language

## Dependencies

- [rustc + cargo][rust] (for the CLI and website)
- [node + npm][node] (for the website)

[rust]: https://rustup.rs
[node]: https://nodejs.org/en/download/

## Development

### CLI

```
$ cargo build
$ cargo test
$ bin/run-test tests/*
```

### Website

```
$ cargo install wasm-pack
$ wasm-pack build wasm
$ npm install
$ npm start
```
