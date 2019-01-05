# frunk-column

A prototype of columnar data storage using frunk HLists

```rust
#[macro_use]
extern crate frunk;
extern crate frunk_core;
use frunk::indices::Here;
use frunk::prelude::*;
use frunk::Generic;
use frunk_column::*;
use std::iter::FromIterator;

fn main() {
    #[derive(Clone, Copy, Debug, Generic, PartialEq)]
    struct Planet {
        mass_kg: f64,
        radius_m: f64,
        habitable: bool,
    }

    let earth = Planet {
        mass_kg: 5.976e+24,
        radius_m: 6.37814e6,
        habitable: true,
    };
    let mars = Planet {
        mass_kg: 6.421e+23,
        radius_m: 3.3972e6,
        habitable: false,
    };

    let mut planets = <Planet as Generic>::Repr::new_frame();
    planets.push(frunk::into_generic(earth));
    planets.push(frunk::into_generic(mars));

    let hlist_pat![_mass_kg, radius_m, ...] = &planets;
    assert_eq!(radius_m, &[6.37814e6, 3.3972e6]);

    assert_eq!(planets.row(1), Some(frunk::into_generic(mars)));

    assert_eq!(
        Vec::from_iter(planets.into_iter().map::<Planet, _>(frunk::from_generic)),
        vec![earth, mars]
    );
}
```

License: MIT/Apache-2.0
