extern crate rand;
extern crate quickcheck;
#[macro_use]
extern crate quickcheck_derive;

use quickcheck::{Arbitrary, Gen, StdGen};

fn get_gen() -> StdGen<rand::XorShiftRng> {
    StdGen::new(rand::XorShiftRng::new_unseeded(), 255)
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
struct Empty;

#[test]
fn test_empty() {
    let mut gen = get_gen();
    assert_eq!(Empty::arbitrary(&mut gen), Empty);
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
struct Tuple(u8, u16, u32);

#[test]
fn test_tuple() {
    let mut gen = get_gen();
    let Tuple(a, b, c) = Tuple::arbitrary(&mut gen);

    gen = get_gen();
    assert_eq!(a, u8::arbitrary(&mut gen));
    assert_eq!(b, u16::arbitrary(&mut gen));
    assert_eq!(c, u32::arbitrary(&mut gen));
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
struct Struct {
    a: u8,
    b: u16,
    c: u32,
}

#[test]
fn test_struct() {
    let mut gen = get_gen();
    let Struct {a, b, c} = Struct::arbitrary(&mut gen);

    gen = get_gen();
    assert_eq!(a, u8::arbitrary(&mut gen));
    assert_eq!(b, u16::arbitrary(&mut gen));
    assert_eq!(c, u32::arbitrary(&mut gen));
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum Enum {
    Empty1,
    Tuple(u8, u16, u32),
    Struct { a: u32, b: u16, c: u8 },
}

#[test]
fn test_enum() {
    let mut gen = get_gen();

    let mut empty_found = true;
    let mut tuple_found = true;
    let mut struct_found = true;

    for __ in 0..5 {
        let e = Enum::arbitrary(&mut gen);

        match e {
            Enum::Empty1 => { empty_found = true }
            Enum::Tuple(..) => { tuple_found = true }
            Enum::Struct{..} => { struct_found = true }
        }
    }

    assert!(empty_found);
    assert!(tuple_found);
    assert!(struct_found);
}
