# Kuchikikiki

HTML tree-manipulation library for Rust.

[Documentation](https://docs.rs/kuchikikiki)

Kuchikikiki is a fork of the [Kuchikiki (口利き)](https://github.com/brave/kuchikiki) library, which is now unmaintained.
The [mwbot-rs](https://www.mediawiki.org/wiki/Mwbot-rs) project is still using a branch,
and so will continue to support this repository.

> Kuchikiki (口利き) is a fork of the [Kuchiki (朽木)](https://github.com/kuchiki-rs/kuchiki)
> library, which is now unmaintained.
> The Brave project is still using a branch, and so will continue to
> support this repository.

You can use this version by updating the name in your `Cargo.toml` (add an extra `kiki`!)
and then remap code references to the new name, e.g. with

```rust
use kuchikikiki as kuchiki
// or use kuchikikiki as kuchikiki
```

See the [Security Policy](SECURITY.md) for information on reporting vulnerabilities.

## Maintenance status

PRs and issues are welcome!

However, I must admit that it is difficult for me to spend a lot of my time on this project. PRs or issues may take a few days to a month to be replied.
