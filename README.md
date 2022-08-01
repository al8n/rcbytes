# RcBytes

The aim for this crate is to implement a Rc version bytes, which means that the structs in this crate does not implement the `Sync` and `Send`.

This crate is heavily based on the [bytes](https://github.com/tokio-rs/bytes) (A utility library for working with bytes).

[![Crates.io][crates-badge]][crates-url]
[![Build Status][ci-badge]][ci-url]

[crates-badge]: https://img.shields.io/crates/v/bytes.svg
[crates-url]: https://crates.io/crates/bytes
[ci-badge]: https://github.com/tokio-rs/bytes/workflows/CI/badge.svg
[ci-url]: https://github.com/tokio-rs/bytes/actions

[Documentation](https://docs.rs/bytes)

## Usage

To use `rcbytes`, first add this to your `Cargo.toml`:

```toml
[dependencies]
rcbytes = "1"
```

Next, add this to your crate:

```rust
use rcbytes::{Bytes, BytesMut, Buf, BufMut};
```

## Serde support

Serde support is optional and disabled by default. To enable use the feature `serde`.

```toml
[dependencies]
rcbytes = { version = "1", features = ["serde"] }
```

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `rcbytes` by you, shall be licensed as MIT, without any additional
terms or conditions.
