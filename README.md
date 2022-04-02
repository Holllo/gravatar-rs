# gravatar-rs

> Gravatar image URL library

## API

For full documentation see [docs.rs].

[docs.rs]: https://docs.rs/gravatar-rs

```rust
use gravatar_rs::Generator;

let generator = Generator::default();

let gravatar_url = generator.generate("helllo@holllo.cc");

assert_eq!(
  gravatar_url,
  "https://www.gravatar.com/avatar/ebff9105dce4954b1bdb57fdab079ff3"
);
```

## License

This project is licensed under either of [Apache License, Version 2.0](https://github.com/Holllo/gravatar-rs/blob/main/LICENSE-Apache) or [MIT license](https://github.com/Holllo/gravatar-rs/blob/main/LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in either crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
