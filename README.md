# `DW3XXX`
[![crates.io](https://img.shields.io/crates/v/dw3xxx.svg)](https://crates.io/crates/dw3xxx) [![Documentation](https://docs.rs/dw3xxx/badge.svg)](https://docs.rs/dw3xxx) ![License](https://img.shields.io/crates/l/dw3xxx.svg)

An experimental driver crate for the Qorvo (formerly Decawave) DW3XXX series of UWB ranging modules.

## Usage

To include the crate in your project run:
```bash
$ cargo add dw3xxx
```
or add the following to your `Cargo.toml` file:
```toml
[dependencies]
dw3xxx = "0.1.0"
```

## Roadmap

This crate is still a work in progress, however, the following is a list of currently implemented features and features that have yet to be implemented.

[X] Low-level register bindings
    [ ] Sanity checked by an actual human being
        * Partially complete
[X] SPI transaction helper functions
[ ] SPI device implementation using `embedded-hal` traits
[ ] Functions for executing device fast commands
    * Partially complete
[ ] Smart interrupt handling
    * Partially complete
[ ] High-level driver interface
[ ] Device soft-reset protocol
[ ] Two-way ranging protocols

## Alternatives

If you need something that works today you may instead be able to use [dw3000-ng](https://crates.io/crates/dw3000-ng) or [dw3000](https://crates.io/crates/dw3000) depending on your use case.

## License

This crate is dual licensed under either the Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://opensource.org/license/apache-2-0), or the MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/license/MIT)

### Contribution

Any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.