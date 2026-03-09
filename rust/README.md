# Known Errors

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/known-errors)](https://crates.io/crates/known-errors)
[![Documentation](https://docs.rs/known-errors/badge.svg)](https://docs.rs/known-errors/)

Well-known errors for Rust.

## ✨ Features

- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add known-errors
```

### Installation in `Cargo.toml` (with all features enabled)

```toml
[dependencies]
known-errors = "0.1"
```

### Installation in `Cargo.toml` (with only specific features enabled)

```toml
[dependencies]
known-errors = { version = "0.1", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the library

```rust
use known_errors;
```

## 📚 Reference

https://docs.rs/known-errors/

### Integrations

Crate (Feature) | Version | Usage | Summary
:--- | :--- | :--- | :---
[error-stack] &nbsp;<sub>(`"error-stack"`)</sub> | 0.5 | [![error-stack](https://docs.rs/error-stack/badge.svg)](https://docs.rs/error-stack/) | Implements `error_stack::Context`
[gofer] &nbsp;<sub>(`"gofer"`)</sub> | 0.1 | [![gofer](https://docs.rs/gofer/badge.svg)](https://docs.rs/gofer/) | Implements `From<gofer::Error>`
[serde] &nbsp;<sub>(`"serde"`)</sub> | 1 | [![serde](https://docs.rs/serde/badge.svg)](https://docs.rs/serde/) | Enables the `serde-json` feature
[serde_json] &nbsp;<sub>(`"serde-json"`)</sub> | 1 | [![serde_json](https://docs.rs/serde_json/badge.svg)](https://docs.rs/serde_json/) | Implements `From<serde_json::Error>`
[tokio] &nbsp;<sub>(`"tokio"`)</sub> | 1 | [![tokio](https://docs.rs/tokio/badge.svg)](https://docs.rs/tokio/) | Implements `From<tokio::task::JoinError>`
<img width="220" height="1"/> | <img width="110" height="1"/> | <img width="100" height="1"/> | &nbsp;

## 👨‍💻 Development

```bash
git clone https://github.com/known-facts/known-errors.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/known-facts/known-errors&text=Known%20Errors)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/known-facts/known-errors&title=Known%20Errors)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/known-facts/known-errors&t=Known%20Errors)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/known-facts/known-errors)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/known-facts/known-errors)

[feature flags]: https://github.com/known-facts/known-errors/blob/master/lib/known-errors/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html

[error-stack]: https://crates.io/crates/error-stack
[gofer]: https://crates.io/crates/gofer
[serde]: https://crates.io/crates/serde
[serde_json]: https://crates.io/crates/serde_json
[tokio]: https://crates.io/crates/tokio
