# birb

A programming language

By Vivian Huang (vivianh) and Ariel Davis (azdavis)

Website deployed at https://azdavis.xyz/birb/

## Dependencies

- [rustc + cargo][rust] (for the CLI and website)
- [node + npm][node] (for the website)

[rust]: https://rustup.rs
[node]: https://nodejs.org/en/download/

## Development and Testing

### One-time setup

```
$ cargo install wasm-pack
$ npm install
```

### CLI

```
$ cargo test
$ bin/run-test.sh tests/*
```

### Website

```
$ wasm-pack build crates/wasm
$ npm start
```

## Deploy

```
$ wasm-pack build crates/wasm
$ npm run build
```
