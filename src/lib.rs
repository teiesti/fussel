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
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt;
extern crate toml;
extern crate walkdir;
extern crate yansi;

pub mod cli;
pub mod config;
pub mod fault;
pub mod lint;
pub mod traverse;
pub mod util;

use cli::Opt;
use lint::{Lint, TrailingWhitespace};
use traverse::Project;

use failure::Error;
use structopt::StructOpt;

pub fn main() {
    if let Err(err) = try_main() {
        handle_error(&err);
    }
}

fn try_main() -> Result<(), Error> {
    // Parse the arguments
    let opt = Opt::from_args();

    /*
     * TODO:
     *   - Load and use the configuration
     *   - Add the option the change the "current directory" via an argument (like 'git -C <path>')
     */

    // Configure the lints
    let project = Project::open_git_workdir()?;
    let lints = vec![
        TrailingWhitespace::review(project),
    ];

    // Run the lints
    for lint in lints {
        for fault in lint {
            println!("{}", fault?);
        }
    }

    Ok(())
}

/// Handles a given error.
///
/// This function handles a given error. That includes:
///
/// 1. Printing the error message.
/// 2. Printing the error message of every causing error, recursively.
/// 3. Exiting the process.
///
/// This function is intended to be used once per application at the very top layer. It should
/// handle every occuring error.
pub fn handle_error(err: &Error) -> ! {
    use yansi::Paint;

    // Print the error
    eprintln!("{} {}", Paint::red("error:").bold(), err);

    // Print the causing errors
    for fail in err.causes().skip(1) {
        eprintln!("{} {}", Paint::blue("caused by:").bold(), fail);
    }

    ::std::process::exit(1);
}
