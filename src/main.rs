use miniserde::{json, Deserialize, Serialize};
use miniserde_derive_enum::{Deserialize_enum, Serialize_enum};

pub fn main() {
    #[derive(Debug, Serialize_enum, Deserialize_enum)]
    enum E {
        UnitA,
        UnitB,
        Struct { a: u8, b: Box<E>, c: Box<E> },
        Tuple(u8, String),
    }

    let ua = E::UnitA;
    let ub = E::UnitB;
    let t = E::Tuple(0, "Hello".to_owned());
    let s = E::Struct {
        a: 0,
        b: Box::new(E::Struct {
            a: 42,
            b: Box::new(E::UnitA),
            c: Box::new(E::Tuple(0, "Test".to_owned())),
        }),
        c: Box::new(E::UnitB),
    };

    let json_s = json::to_string(&s);
    let json_t = json::to_string(&t);
    let json_ua = json::to_string(&ua);
    let json_ub = json::to_string(&ub);

    println!("{}", json_ua);
    println!("{}", json_ub);
    println!("{}", json_s);
    println!("{}", json_t);

    let ua: E = json::from_str(&json_ua).unwrap();
    let ub: E = json::from_str(&json_ub).unwrap();
    let s: E = json::from_str(&json_s).unwrap();
    let t: E = json::from_str(&json_t).unwrap();

    dbg!(ua, ub, s, t);
}
