# birb

A toy programming language with effects and contracts.

Check out the [website][].

## Dependencies

- [rustc + cargo][rust] (for the CLI and website)
- [node + npm][node] (for the website)

## Development

### One-time setup

```
$ cargo install wasm-pack
$ npm install
```

### CLI dev

```
$ cargo test
$ bin/run-test.sh tests/*
```

### Website dev

```
$ npm start
```

### Build website (for production)

```
$ npm run build
```

[website]: https://birb-lang.netlify.app
[rust]: https://rustup.rs
[node]: https://nodejs.org/en/download/
