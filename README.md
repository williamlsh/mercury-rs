# Mercury-rs

Rust port of [mercury](https://github.com/mercury-hpc/mercury).

This project consists of:

- mercury-sys
- mercury-rs

`mercury-sys` is ready for anyone to use on top of which `mercury-rs` is built. But current `mercury-rs` only encapsulates safe code of a small part of [Mercury RPC Layer](https://mercury-hpc.github.io/user/hg/).

> This project is just a Rust FFI tour. Do not consider it for production use. Contributions are welcome.

## How to build?

> Note: only tested on Debian Linux with sudo permission enabled.

Install `mercury` OFI(libfabric) plugin first to build `mercury-rs`.

```
bash deps.sh
```

Build `mercury-rs`:

```
git submodule update --init --recursive
cargo build -p mercury-rs
```

If you want to view `mercury-sys` doc on your default browser.

```
cargo doc -p mercury-sys --open
```

## How to run examples?

To run `mercury-sys` examples, take `init_server` for example.

```
cargo run -p mercury-sys --example init_server
```

To run `mercury-rs` example, take `init_server` for example.

```
cargo run -p mercury-rs --example init_server
```

## Author

* [William](https://github.com/williamlsh)
