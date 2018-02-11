#![warn(
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
)]

#[macro_use] extern crate failure;
extern crate git2;
extern crate walkdir;

pub mod fault;
pub mod traverse;

pub fn main() {
    println!("Hello, world!");
}
