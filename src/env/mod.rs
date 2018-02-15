pub mod args;
pub mod conf;
pub mod vars;

use failure::Error;
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug)]
pub struct Env {
    pub current_dir: PathBuf,
    pub working_dir: PathBuf,
    pub lints: Lints,
}

impl Env {
    pub fn init() -> Result<Self, Error> {
        // Parse the arguments
        let args = args::Opt::from_args();

        // Determine the current directory
        let current_dir = args.current_dir.unwrap_or(
            ::std::env::current_dir().map_err(Error::from)?
        );

        // Discover the configuration
        let (conf, working_dir) = conf::Opt::discover(&current_dir).map_err(Error::from)?;

        // Lift the lint configuration
        let lints = Lints {
            trailing_whitespace: {
                conf.trailing_whitespace.map(|x| {
                    TrailingWhitespace {
                        extension_blacklist: {
                            x.extension_blacklist.iter().map(|s| s.into()).collect()
                        },
                    }
                })
            }
        };

        Ok(Self {
            current_dir,
            working_dir,
            lints,
        })
    }
}

#[derive(Debug)]
pub struct Lints {
    pub trailing_whitespace: Option<TrailingWhitespace>,
}

#[derive(Debug)]
pub struct TrailingWhitespace {
    pub extension_blacklist: HashSet<OsString>,
}
