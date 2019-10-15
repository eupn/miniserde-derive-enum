use miniserde::{json, Deserialize, Serialize};
use miniserde_derive_enum::Serialize_enum;

pub fn main() {
    #[derive(Serialize_enum)]
    enum E {
        Unit,
        Struct { a: u8, b: u8 },
        Tuple(u8, String),
    }

    let s = E::Struct { a: 0u8, b: 1u8 };
    let u = E::Unit;
    let t = E::Tuple(0u8, "Hello".to_owned());
    println!(
        "{}\n{}\n{}",
        json::to_string(&s),
        json::to_string(&u),
        json::to_string(&t)
    );
}
