pub mod tests;

use reflection::*;

#[derive(Debug, Default, Reflection, UpdateWithStr)]
struct Inner {
    abc: String,
    d: f64,
}

#[derive(Debug, Default, Reflection)]
struct Test {
    a: i32,
    b: i32,
    string: String,
    double: f64,

    rec: Box<Option<Test>>,

    inner: Inner,
}

fn _vsp(_a: &mut dyn Reflection<Field = <Test as Reflection>::Field>) {}

fn main() {
    let mut t: Test = Default::default();

    let f = t.get_field("abc");
    dbg!(f);
    let f = t.get_field("string");
    dbg!(f);
    let f = t.get_field("double");
    dbg!(f);
    let f = t.get_field("rec");
    dbg!(&f);
    println!();
    match f {
        ReflectionTestFields::rec(r) => *r = Box::new(Some(Default::default())),
        _ => {}
    }
    let f = t.get_field("rec");
    dbg!(f);
    println!();
    dbg!(&t);
    println!();
    dbg!(t.get_field_list());
}
