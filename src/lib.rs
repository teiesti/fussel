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
#[macro_use] extern crate lazy_static;
extern crate regex;
#[macro_use] extern crate structopt;
extern crate walkdir;
extern crate yansi;

pub mod cli;
pub mod fault;
pub mod lint;
pub mod traverse;
pub mod util;

pub use cli::main;
