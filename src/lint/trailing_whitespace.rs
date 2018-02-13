use fault::{Example, Fault, Message, Scope};
use lint::Lint;
use traverse::Project;
use util;

use failure::Error;
use regex::{Match, Regex};
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct TrailingWhitespace {
    project: Project,
    ignore_extensions: HashSet<OsString>,
}

impl Lint for TrailingWhitespace {
    fn review(project: Project) -> Self {
        Self {
            project,
            ignore_extensions: HashSet::new(),
        }
    }
}

impl IntoIterator for TrailingWhitespace {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        // Prepare the regular expression
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"\s+$").unwrap();
        }

        // Prepare the hint messages
        let mut hints = vec![];
        if !self.ignore_extensions.is_empty() {
            hints.push({
                // Format the ignored extensions
                let mut exts: Vec<_> = self.ignore_extensions.iter().map(|ext| {
                    format!("'.{}'", ext.to_str().unwrap())
                }).collect();

                // Sort them
                exts.sort_unstable();

                // Create a list in natural language
                let list = util::list_or(&mut exts.iter());

                // Assemble the hint message
                Message::note(
                    format!("filenames ending with {} are ignored", list)
                )
            })
        }

        // Extend the project's ignore extensions list
        self.project.ignore_extensions.extend(self.ignore_extensions.drain());

        // Define how a fault has to be assembled
        let fault = move |path: PathBuf, i: usize, line: String, mat: Match| {
            Fault {
                msg: Message::warning(
                    "lines should not end with trailing whitespace, \
                     unless the file format requires"
                ),
                example: Example {
                    mark: Scope {
                        path: path.clone(),
                        range: (
                            (i, mat.start()  ).into(),
                            (i, mat.end() - 1).into(),
                        ).into(),
                    },
                    ctx: Scope {
                        path,
                        range: (
                            (i, 0         ).into(),
                            (i, line.len()).into(),
                        ).into(),
                    },
                    txt: line,
                    msg: Message::bare(
                        "whitespace found here"
                    ),
                },
                hints: hints.clone(),
            }
        };

        // Assemble the iterator
        let iter = self.project.lines().into_iter().filter_map(move |x| {
            match x {
                Ok((path, i, line)) => {
                    REGEX.find(line.clone().as_str())
                        .map(|mat| {
                            Ok(fault(path, i, line, mat))
                        })
                },
                Err(err) => Some(Err(err)),
            }
        });

        // Box and return the iterator
        Box::new(iter)
    }
}
