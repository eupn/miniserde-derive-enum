//! [Miniserde](https://github.com/dtolnay/miniserde) derive macros that support `enum`s.
//!
//! Provides a minimal `Serialize_enum`, `Deserialize_enum` derive macros
//! for `enum` support in Miniserde.
//!
//! # Example
//! ```
//! # use miniserde_derive_enum::{Deserialize_enum, Serialize_enum};
//!
//! # pub fn main() {
//!    #[derive(Serialize_enum, Deserialize_enum)]
//!    enum E {
//!        Unit,
//!        Struct { a: u8, b: String, c: Box<E> },
//!        Tuple(u8, String),
//!    }
//! # }
//! ```
#![recursion_limit = "128"]

extern crate proc_macro;

mod de;
mod ser;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Serialize_enum, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    ser::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_derive(Deserialize_enum, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    de::derive(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
