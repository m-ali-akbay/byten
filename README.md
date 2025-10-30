# byten

[![Crates.io](https://img.shields.io/crates/v/byten.svg)](https://crates.io/crates/byten)
[![Documentation](https://docs.rs/byten/badge.svg)](https://docs.rs/byten)
[![License](https://img.shields.io/crates/l/byten.svg)](https://github.com/m-ali-akbay/byten#license)

A binary codec library for efficient encoding and decoding of Rust data structures.

> ⚠️ **Early Development**: This library is in active development and the API may change.

## Features

- 🚀 **Derive macros** for automatic codec implementation
- 🔢 **Primitive types** with custom byte ordering (BE/LE)
- 📦 **Variable-length encoding** support
- 🎯 **Type-safe** encoding and decoding
- 🔧 **Flexible** attribute-based customization

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
byten = "0.0"
```

## Basic Usage

```rust
use byten::{Decode, Encode, Measure, prim::U32BE};

#[derive(Debug, Decode, Encode, Measure, PartialEq)]
struct Person {
    #[byten(U32BE)]
    id: u32,
    age: u8,
    name_length: u8,
}

fn main() {
    let person = Person {
        id: 12345,
        age: 30,
        name_length: 4,
    };

    // Encode to Vec
    let encoded = person.encode_to_vec().unwrap();
    
    // Decode from slice
    let decoded = Person::decode(&encoded).unwrap();
    assert_eq!(person, decoded);
}
```

## Advanced Example

```rust
use byten::{Decode, Encode, Measure, prim::{U16BE, U16LE}, var};

#[derive(Debug, Decode, Encode, Measure, PartialEq)]
struct Data {
    #[byten(U32BE)]
    id: u32,
    
    #[byten(var::Vec::<var::USizeBE, SelfCodec::<Item>>::default())]
    items: Vec<Item>,
}

#[derive(Debug, Decode, Encode, Measure, PartialEq)]
#[repr(u16)]
#[byten(U16LE)]
enum Item {
    Empty = 0,
    Value(#[byten(U16BE)] u16) = 1,
    Named { 
        id: u8,
        #[byten(U16BE)] 
        value: u16 
    } = 2,
}
```

## Features Flags

- `derive` (default): Enable derive macros for `Encode`, `Decode`, and `Measure`
- `anyhow` (default): Integration with the `anyhow` error handling crate

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! This project is in early development.

