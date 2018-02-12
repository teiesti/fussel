use fault::{Example, Fault, Message, Scope};
use traverse::Lines;

use failure::Error;
use regex::{Match, Regex};
use std::path::PathBuf;

pub struct TrailingWhitespace {
    lines: Lines,
}

impl TrailingWhitespace {
    pub fn review(lines: Lines) -> Self {
        Self { lines }
    }
}

impl IntoIterator for TrailingWhitespace {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        // Prepare the regular expression
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"\s+$").unwrap();
        }

        // Define how a fault has to be assembled
        let fault = |path: PathBuf, i: usize, line: String, mat: Match| {
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
                hints: vec![],
            }
        };

        // Assemble the iterator
        let iter = self.lines.into_iter().filter_map(move |x| {
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
