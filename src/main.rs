use miniserde::{Serialize, Deserialize, json};
use miniserde_derive_enum::{Serialize_enum};

pub fn main() {
    #[derive(Serialize_enum)]
    enum E {
        Unit,
        Struct { a: u8, b: u8},
//        Tuple(u8, String)
    }

    let s = E::Struct { a: 0u8, b: 1u8 };
    let u = E::Unit;
    println!("{}\n{}", json::to_string(&s), json::to_string(&u));
}
