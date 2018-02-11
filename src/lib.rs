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
extern crate walkdir;

pub mod fault;
pub mod lint;
pub mod traverse;

use lint::TrailingWhitespace;
use traverse::Project;

pub fn main() {
    let project = Project::open_git_workdir().unwrap(); // TODO remove unwrap
    let lint = TrailingWhitespace::review(project.lines());

    for fault in lint {
        println!("{}", fault.unwrap()); // TODO remove unwrap
    }
}
