# miniserde-derive-enum

[Miniserde](https://github.com/dtolnay/miniserde) derive macros that support `enum`s.

Provides a minimal `Serialize_enum`, `Deserialize_enum` derive macros
for `enum` support in Miniserde.

## Example
```rust
use miniserde::{Serialize, Deserialize};
use miniserde_derive_enum::{Serialize_enum, Deserialize_enum};

pub fn main() {
   #[derive(Serialize_enum, Deserialize_enum)]
   enum E {
       Unit,
       Struct { a: u8, b: String, c: Box<E> },
       Tuple(u8, String),
   }
}
```

License: MIT OR Apache-2.0
