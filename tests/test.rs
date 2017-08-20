#[macro_use]
extern crate random_derive;
extern crate rand;

use rand::{Rand, Rng};

#[test]
fn struct_works() {
    #[derive(RandTrait)]
    struct S {
        x: i32,
        y: i32,
    }

    let s = rand::random::<S>();
}

#[test]
fn enum_works() {
    #[derive(RandTrait)]
    enum E {
        Variant1,
        Variant2,
    }

    let e: E = rand::random();
}

#[test]
fn tuple_works() {
    #[derive(RandTrait)]
    enum E {
        Variant1(u32, u32),
        Variant2(u64),
    }

    let e: E = rand::random();
}

#[test]
fn nested_derives() {
    #[derive(RandTrait)]
    struct S {
        x: i32,
        y: i32,
    }

    #[derive(RandTrait)]
    struct T {
        s1: (S, S),
        s2: S,
    }
    
    let t: T = rand::random();
}
