![dyn_path](https://github.com/user-attachments/assets/8c6b4cd3-7686-487e-b2d8-0dd63477185b)

[![Crates.io](https://badges.ws/crates/v/dyn_path)](https://crates.io/crates/dyn_path)
[![License](https://badges.ws/crates/l/dyn_path)](./LICENSE)
[![Docs.rs](https://badges.ws/crates/docs/dyn_path)](https://docs.rs/dyn_path)
[![Downloads](https://badges.ws/crates/dt/dyn_path)](https://crates.io/crates/dyn_path)
[![Codecov](https://img.shields.io/codecov/c/github/FlakySL/dyn_path)](https://app.codecov.io/gh/FlakySL/dyn_path)
![tests](https://github.com/FlakySL/dyn_path/actions/workflows/overall-coverage.yml/badge.svg)
[![discord](https://badges.ws/discord/online/1344769456731197450)](https://discord.gg/AJWFyps23a)

A JavaScript-like nested object-like structure non-fallible path access interface.

This library permits the user to access values in nested structures trough the use
of `get` methods that return `Option<T>`. This may be used to access specific API data
without serializing it into multiple structures and making a mess.

## Table of Contents 📖

- [Use Cases](#use-cases-)
- [Installation](#installation-)
- [No STD](#no-std-)
- [Licensing](#license-)

## Use Cases 🔍

Imagine you just accessed a song API, this API describes everything you may or may not
want to know about a specific song or songs. With the help of computed indexes you can
access the same path multiple times in a variable way.

Lets try it.

```rust
use serde_json::Value as JsonValue;
use reqwest::get;
use dyn_path::dyn_access;

let response = get("https://api.platform-ending-with-fy.com/v1/tracks/5cbpoIu3YjoOwbBDGUEp3P")
	.await?
	.json::<JsonValue>()
	.await?;

let song_name = dyn_access!(response.album.name);
let song_main_artist = dyn_access!(response.album.artists[0].name);

assert_eq!(song_name, "Car Radio");
assert_eq!(song_main_artist, "Twenty One Pilots");
```

Obviously this isn't a real call to a real API, but the intention can be inferred from
this example anyways.

## Installation 📦

Add the following to your `Cargo.toml` under the `dependencies` section

```toml
dyn_path = "1.0.7"
```

## No std 💡

This crate supports `no_std`, meaning you can use it in your project without
depending on specific system I/O or anything else.

The crate has a default feature enabled which is `std`, with this feature
the crate doesn't really use `std` but it pre-includes `alloc` which
permits the use of the `dyn_path` macro, which uses `String` to describe
a path.

You can also disable the default features and the crate will annotate `no_std`.
Then use the `alloc` feature if you want to have the `dyn_path` macro enabled.

## License 📜

This repository is dual licensed, TLDR. If your repository is open source, the library
is free of use, otherwise contact [licensing@flaky.es](mailto:licensing@flaky.es) for a custom license for your
use case.

For more information read the [license](./LICENSE) file.
