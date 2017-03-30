# quickcheck_derive

`#[derive]` for [quickcheck's](https://github.com/burntsushi/quickcheck) `quickcheck::Arbitrary`.

## Usage

In your `Cargo.toml` add:

```toml
[dev-dependencies]
quickcheck_derive = { git = "https://github.com/alanhdu/quickcheck_derive" }
```

Then in your test:

```rust
#[macro_use]
extern crate quickcheck_derive;
extern crate quickcheck;

use quickcheck::{Arbitrary, Gen};

#[derive(Arbitrary, Clone, Debug, PartialEq)]
struct Test {
    a: u32,
    b: Vec<u16>,
    // or whatever fields you need
}
```

## Known Limitations

- `Arbitrary` and `Gen` must be in scope (so you must `use quickcheck::{Arbitrary, Gen}`).
- You must also implement `Clone`, `Debug`, and `PartialEq` (although you can just derive those too)
- All `enum` variants are equally likely to be generated, so be careful with recursive `enum`s.
