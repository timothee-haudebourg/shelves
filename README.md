# Shelves: Storing values referenced by a unique typed index.

[![CI](https://github.com/timothee-haudebourg/shelves/workflows/CI/badge.svg)](https://github.com/timothee-haudebourg/shelves/actions)
[![Crate informations](https://img.shields.io/crates/v/shelves.svg?style=flat-square)](https://crates.io/crates/shelves)
[![License](https://img.shields.io/crates/l/shelves.svg?style=flat-square)](https://github.com/timothee-haudebourg/shelves#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/shelves)

This is a small utility library for storing values of and
reference them using a unique typed index, `Ref<T>`,
which is a simple typed wrapper around `usize`.

Any data structure can be used behind the shelf as long as it provides
a way to store and fetch values by `usize` through the implementation of the `Storage` trait.
This library provides a `Storage` implementation for `Vec`, `BTreeMap` and `HashMap`.
In addition, a `Storage` implementation is provided for the `slab::Slab` type by enabling
the `slab-storage` feature.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
