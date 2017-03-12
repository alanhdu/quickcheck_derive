extern crate rand;
#[macro_use]
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
fn empty() {
    let mut gen = get_gen();
    assert_eq!(Empty::arbitrary(&mut gen), Empty);
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
struct Tuple(u8, u16, u32);

#[test]
fn tuple_arbitrary() {
    let mut gen = get_gen();
    let Tuple(a, b, c) = Tuple::arbitrary(&mut gen);

    gen = get_gen();
    assert_eq!(a, u8::arbitrary(&mut gen));
    assert_eq!(b, u16::arbitrary(&mut gen));
    assert_eq!(c, u32::arbitrary(&mut gen));
}

quickcheck! {
    fn tuple_shrink(a: u8, b: u16, c: u32) -> bool {
        let xs = Tuple(a, b, c).shrink()
            .map(|Tuple(a, b, c)| (a, b, c))
            .collect::<Vec<_>>();
        let ys = (a, b, c).shrink().collect::<Vec<_>>();

        xs == ys
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
struct Struct {
    a: u8,
    b: u16,
    c: u32,
}

#[test]
fn struct_arbitrary() {
    let mut gen = get_gen();
    let Struct {a, b, c} = Struct::arbitrary(&mut gen);

    gen = get_gen();
    assert_eq!(a, u8::arbitrary(&mut gen));
    assert_eq!(b, u16::arbitrary(&mut gen));
    assert_eq!(c, u32::arbitrary(&mut gen));
}

quickcheck! {
    fn struct_shrink(a: u8, b: u16, c: u32) -> bool {
        let xs = Struct{a: a, b: b, c: c}.shrink()
            .map(|Struct{a, b, c}| (a, b, c))
            .collect::<Vec<_>>();
        let ys = (a, b, c).shrink().collect::<Vec<_>>();

        xs == ys
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum EnumVariants {
    Empty1,
    Tuple(u8, u16, u32),
    Struct { a: u32, b: u16, c: u8 },
}

#[test]
fn enum_variants() {
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
fn enum_empty_arbitrary() {
    let mut gen = get_gen();
    assert_eq!(EnumEmpty::arbitrary(&mut gen), EnumEmpty::Empty);
}

#[test]
fn enum_empty_shrink() {
    assert!(
        EnumEmpty::Empty.shrink()
            .collect::<Vec<_>>()
            .is_empty()
    );
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum EnumTuple {
   Tuple(u8, u16)
}

#[test]
fn enum_tuple_arbitrary() {
    let mut gen = get_gen();
    let EnumTuple::Tuple(a, b) = EnumTuple::arbitrary(&mut gen);

    gen = get_gen();
    gen.gen_range::<usize>(0, usize::max_value());   // skip first part for enum
    assert_eq!(a, u8::arbitrary(&mut gen));
    assert_eq!(b, u16::arbitrary(&mut gen));
}

quickcheck!{
    fn enum_tuple_shrink(a: u8, b: u16) -> bool {
        let xs = EnumTuple::Tuple(a, b).shrink()
            .map(|EnumTuple::Tuple(a, b)| (a, b))
            .collect::<Vec<_>>();
        let ys = (a, b).shrink().collect::<Vec<_>>();

        xs == ys
    }
}

#[derive(Arbitrary, Clone, Debug, PartialEq)]
enum EnumStruct {
    Struct{ a: u16, b: u8 },
}

#[test]
fn enum_struct_arbitrary() {
    let mut gen = get_gen();
    let EnumStruct::Struct{a, b} = EnumStruct::arbitrary(&mut gen);

    gen = get_gen();
    gen.gen_range::<usize>(0, usize::max_value());   // skip first part for enum
    assert_eq!(a, u16::arbitrary(&mut gen));
    assert_eq!(b, u8::arbitrary(&mut gen));
}

quickcheck! {
    fn enum_struct_shrink(a: u16, b: u8) -> bool {
        let xs = EnumStruct::Struct {a: a, b: b}.shrink()
            .map(|EnumStruct::Struct{a, b}| (a, b))
            .collect::<Vec<_>>();
        let ys = (a, b).shrink().collect::<Vec<_>>();

        xs == ys
    }
}
