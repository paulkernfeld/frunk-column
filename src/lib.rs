//! A prototype of columnar data storage using frunk HLists
//!
//! ```
//! #[macro_use]
//! extern crate frunk;
//! extern crate frunk_core;
//! use frunk::indices::Here;
//! use frunk::prelude::*;
//! use frunk::Generic;
//! use frunk_column::*;
//! use std::iter::FromIterator;
//!
//! fn main() {
//!     #[derive(Clone, Copy, Debug, Generic, PartialEq)]
//!     struct Planet {
//!         mass_kg: f64,
//!         radius_m: f64,
//!         habitable: bool,
//!     }
//!
//!     let earth = Planet {
//!         mass_kg: 5.976e+24,
//!         radius_m: 6.37814e6,
//!         habitable: true,
//!     };
//!     let mars = Planet {
//!         mass_kg: 6.421e+23,
//!         radius_m: 3.3972e6,
//!         habitable: false,
//!     };
//!
//!     let mut planets = <Planet as Generic>::Repr::new_frame();
//!     planets.push(frunk::into_generic(earth));
//!     planets.push(frunk::into_generic(mars));
//!
//!     let hlist_pat![_mass_kg, radius_m, ...] = &planets;
//!     assert_eq!(radius_m, &[6.37814e6, 3.3972e6]);
//!
//!     assert_eq!(planets.row(1), Some(frunk::into_generic(mars)));
//!
//!     assert_eq!(
//!         Vec::from_iter(planets.into_iter().map::<Planet, _>(frunk::from_generic)),
//!         vec![earth, mars]
//!     );
//! }
//! ```
use frunk::prelude::*;
use frunk::HCons;
use frunk::HNil;

pub trait Frame: HList {
    type MyRow: HList;
    type Iterator: Iterator<Item = Self::MyRow>;

    fn push(&mut self, row: Self::MyRow);

    fn into_iter(self) -> Self::Iterator;

    fn row(&self, index: usize) -> Option<Self::MyRow>;
}

pub struct HNilIterator {}

impl Iterator for HNilIterator {
    type Item = HNil;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        panic!("I don't think it's possible to get here")
    }
}

// This implementation is kind of janky because HNil doesn't have a way to keep track of state.
// I think the main negative effect of this is that a Frame with 0 columns will behave crazily.
impl Frame for HNil {
    type MyRow = HNil;
    type Iterator = HNilIterator;

    fn push(&mut self, _row: Self::MyRow) {
        // intentionally do nothing
    }

    fn into_iter(self) -> Self::Iterator {
        panic!("I don't think it's possible to get here")
    }

    fn row(&self, _index: usize) -> Option<Self::MyRow> {
        Some(HNil)
    }
}

pub struct HConsIterator<F: Frame> {
    index: usize,
    frame: F,
}

impl<F: Frame> Iterator for HConsIterator<F> {
    type Item = <F as Frame>::MyRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.frame.row(self.index).map(|row| {
            self.index += 1;
            row
        })
    }
}

impl<H, T> Frame for HCons<Vec<H>, T>
where
    H: Clone,
    T: Frame,
{
    type MyRow = HCons<H, T::MyRow>;
    type Iterator = HConsIterator<Self>;

    fn push(&mut self, row: Self::MyRow) {
        let (h, t) = row.pop();
        self.head.push(h);
        self.tail.push(t);
    }

    fn into_iter(self) -> Self::Iterator {
        HConsIterator {
            index: 0,
            frame: self,
        }
    }

    fn row(&self, index: usize) -> Option<Self::MyRow> {
        self.head.get(index).map(|head| HCons {
            head: head.clone(),
            tail: self.tail.row(index).unwrap(),
        })
    }
}

pub trait Row: HList {
    type MyFrame: Frame<MyRow = Self>;

    fn new_frame() -> Self::MyFrame;
}

impl Row for HNil {
    type MyFrame = HNil;

    fn new_frame() -> HNil {
        HNil
    }
}

impl<H, T> Row for HCons<H, T>
where
    H: Clone,
    T: Row,
{
    type MyFrame = HCons<Vec<H>, <T as Row>::MyFrame>;

    fn new_frame() -> Self::MyFrame {
        HCons {
            head: Vec::new(),
            tail: T::new_frame(),
        }
    }
}
