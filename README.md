# random\_derive
Procedurally defined macro for automatically deriving rand::Rand for structs and enums

# `enums`
Given an enum of multiple variants, this macro allows you to define the [`rand::Rand`](https://doc.rust-lang.org/rand/rand/trait.Rand.html) trait such that calling `rand::random()` will create an enum with a randomly selected variant.

For example:
```rust
#[macro_use]
extern crate random_derive;
extern crate rand;

use rand::Rand;

#[derive(Debug, RandTrait)]
enum Example {
    A,
    B,
}

fn main() {
    // Create 10 random examples
    let examples = vec! [
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
        rand::random::<Example>(),
    ];

    // Let's see if we're about 50/50 A/B
    for example in examples.iter() {
        println!("{:?}", example);
    }
}
```

Would yield similar to the following:
```
B
A
A
B
A
A
A
B
A
B
```

60/40 A/B, not too shabby!

enums composed of structs and tuples are also supported, granted that each struct or item in the tuple has `rand::Rand` already implemented.  For example:
```rust
#[derive(Debug, RandTrait)]
enum Example2 {
    C(u8, u8),
    D(bool),
    E,
}

fn main() {
    println!("{:?}", rand::random::<Example2>());
}
```
will print either:
* `C` with two random `u8`s, or
* `D` with either `true` or `false`, or
* just `E`

with roughly even odds between the three.

# `structs`
`rand::Rand` is implemented simpler for structs - for each field in the struct, `rand::random()` will be called, so it only works if the fields are primitives or if their types have already implemented `rand::Rand`.

For a concrete example, the following struct:
```rust
#[derive(RandTrait)]
struct ColorPoint {
    x: i32,
    y: i32,
    color: (u8, u8, u8),
}
```

will be automatically derived with the following rand function:
```rust
impl Rand for ColorPoint {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        ColorPoint {
            x: rand::random(),
            y: rand::random(),
            color: rand::random(), // rand::Rand is already implemented for tuples
        }
    }
}
```

# Considerations and Acknowledgements
Right now, lifetimes are not implemented, so all variants and fields will have to be fully owned by the struct or enum.

Additionally, the necessary `rng` variable in the rand implementation goes unused, so you will get code warnings when using this crate.

Finally, I was heavily influenced by the [`deep-clone-derive`](https://github.com/asajeffrey/deep-clone/blob/master/deep-clone-derive/lib.rs) crate from the [`syn` README](https://github.com/dtolnay/syn), and have attributed this crate under the same license.
