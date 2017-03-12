extern crate rand;
extern crate quickcheck;
#[macro_use]
extern crate quickcheck_derive;

use quickcheck::{Arbitrary, Gen, StdGen};
use rand::Rng;

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
enum EnumVariants {
    Empty1,
    Tuple(u8, u16, u32),
    Struct { a: u32, b: u16, c: u8 },
}

#[test]
fn test_enum_variants() {
    let mut gen = get_gen();

    let mut empty_found = true;
    let mut tuple_found = true;
    let mut struct_found = true;

    for __ in 0..5 {
        let e = EnumVariants::arbitrary(&mut gen);

        match e {
            EnumVariants::Empty1 => { empty_found = true }
            EnumVariants::Tuple(..) => { tuple_found = true }
            EnumVariants::Struct{..} => { struct_found = true }
        }
    }

    assert!(empty_found);
    assert!(tuple_found);
    assert!(struct_found);
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum EnumEmpty {
    Empty
}

#[test]
fn test_enum_empty() {
    let mut gen = get_gen();
    assert_eq!(EnumEmpty::arbitrary(&mut gen), EnumEmpty::Empty);
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum EnumTuple {
   Tuple(u8, u16)
}

#[test]
fn test_enum_tuple() {
    let mut gen = get_gen();
    let EnumTuple::Tuple(a, b) = EnumTuple::arbitrary(&mut gen);

    gen = get_gen();
    gen.gen_range::<usize>(0, usize::max_value());   // skip first part for enum
    assert_eq!(a, u8::arbitrary(&mut gen));
    assert_eq!(b, u16::arbitrary(&mut gen));
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum EnumStruct {
    Struct{ a: u16, b: u8 },
}

#[test]
fn test_enum_struct() {
    let mut gen = get_gen();
    let EnumStruct::Struct{a, b} = EnumStruct::arbitrary(&mut gen);

    gen = get_gen();
    gen.gen_range::<usize>(0, usize::max_value());   // skip first part for enum
    assert_eq!(a, u16::arbitrary(&mut gen));
    assert_eq!(b, u8::arbitrary(&mut gen));
}
