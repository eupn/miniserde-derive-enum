use miniserde::{json, Deserialize, Serialize};
use miniserde_derive_enum::{Deserialize_enum, Serialize_enum};

pub fn main() {
    #[derive(Debug, Serialize_enum, Deserialize_enum)]
    enum E {
        A { a: u8, b: Box<E> },
        B { b: u8 },
    }

    let s_a = E::A {
        a: 0,
        b: Box::new(E::B { b: 1 }),
    };
    let s_b = E::B { b: 1 };

    /*let u = E::Unit;
    let t = E::Tuple(0u8, "Hello".to_owned());

    println!(
        "{}\n{}\n{}",
        json::to_string(&s),
        json::to_string(&u),
        json::to_string(&t)
    );*/

    let json_a = json::to_string(&s_a);
    println!("{}", json_a);

    let json_b = json::to_string(&s_b);
    println!("{}", json_b);

    let s_a: E = json::from_str(&json_a).unwrap();
    let s_b: E = json::from_str(&json_b).unwrap();

    dbg!(s_a, s_b);
}
