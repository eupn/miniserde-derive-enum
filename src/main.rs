use miniserde::{json, Deserialize, Serialize};
use miniserde_derive_enum::{Deserialize_enum, Serialize_enum};

pub fn main() {
    #[derive(Debug, Serialize_enum, Deserialize_enum)]
    enum E {
//        Unit,
        Struct { a: u8, b: Box<E> },
        Tuple(u8, String),
    }
    let t = E::Tuple(0, "Hello".to_owned());


    let s = E::Struct {
        a: 0,
        b: Box::new(E::Tuple (0, "hello".to_owned())),
    };
    let t = E::Tuple (0, "hi".to_owned());

    let json_s = json::to_string(&s);
    let json_t = json::to_string(&t);
    println!("{}", json_s);
    println!("{}", json_t);

    let s: E = json::from_str(&json_s).unwrap();
    let t: E = json::from_str(&json_t).unwrap();

    dbg!(s, t);
}
