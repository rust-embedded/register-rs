# Changelog

## [v0.3.3] - 2019-06-29

- Update Readme with deref pattern example.

## [v0.3.2] - 2018-12-20

- Update Readme for Rust 2018 as well.

## [v0.3.1] - 2018-12-20

- Depend on tock-registers `0.3.0`.
- Update to Rust 2018.

## v0.3.0 - 2018-12-20

- Release made with erroneous documentation - `yanked`.

## [v0.2.1] - 2018-10-10

- Moved repository to https://github.com/rust-embedded

## [v0.2.0] - 2018-08-23

- Depend on tock-registers `0.2.0`.
- Add `read_as_enum` to `cpu::RegisterReadWrite` and `cpu::RegisterReadOnly`.
- Fix reexports. Too many structs and traits were reexportes under the `mmio`.
  namespace. Reexport them at the toplevel now.
- Remove `#[inline]` from trait methods without body.

## [v0.1.1] - 2018-07-16

- Moved repository to https://github.com/rust-osdev

## v0.1.0 - 2018-07-07

- Initial Release.

[v0.1.1]: https://github.com/rust-embedded/register-rs/compare/v0.1.0...v0.1.1
[v0.2.0]: https://github.com/rust-embedded/register-rs/compare/v0.1.1...v0.2.0
[v0.2.1]: https://github.com/rust-embedded/register-rs/compare/v0.2.0...v0.2.1
[v0.3.1]: https://github.com/rust-embedded/register-rs/compare/v0.2.1...v0.3.1
[v0.3.2]: https://github.com/rust-embedded/register-rs/compare/v0.3.1...v0.3.2
[v0.3.3]: https://github.com/rust-embedded/register-rs/compare/v0.3.2...v0.3.3
