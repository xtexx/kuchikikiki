# Kuchikikiki

HTML tree-manipulation library for Rust.

[Documentation](https://docs.rs/kuchikikiki)

Kuchikikiki is a fork of the Kuchikiki (口利き) library, which is now unmaintained.
The [mwbot-rs](https://www.mediawiki.org/wiki/Mwbot-rs) project is still using a branch,
and so will continue to support this repository.

> Kuchikiki (口利き) is a fork of the Kuchiki (朽木) library, which is now unmaintained.
> The Brave project is still using a branch, and so will continue to
> support this repository.

You can use this version by updating the name in your `Cargo.toml` (add an extra `kiki`!)
and then remap code references to the new name, e.g. with

```rust
use kuchikikiki as kuchiki
// or use kuchikikiki as kuchikiki
```

See the [Security Policy](SECURITY.md) for information on reporting vulnerabilities.
