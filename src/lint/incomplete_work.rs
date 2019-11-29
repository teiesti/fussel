use crate::fault::{Fault, Message, Quote};
use crate::lint::Lint;

use std::path::Path;

pub(crate) struct IncompleteWork {
    keywords: Vec<String>,
}

impl IncompleteWork {
    pub(crate) fn init(keywords: Vec<String>) -> Self {
        Self { keywords }
    }
}

impl Lint for IncompleteWork {
    fn review<F: FnMut(Fault)>(&self, path: &Path, content: &str, mut report: F) {
        for (row, txt) in content.lines().enumerate() {
            for keyword in &self.keywords {
                for (col_from, found) in txt.match_indices(keyword) {

                    let fault = Fault {
                        msg: Message::warning("there should be no incomplete work"),

                        quote: Quote::Excerpt {
                            path: path.to_path_buf(),
                            row: row + 1,
                            col_from,
                            col_to: col_from + found.len() - 1,
                            txt: txt.to_string(),
                            msg: Message::bare("keyword suggests incomplete work"),
                        },

                        hints: vec![
                            Message::help("remove keyword after completing work"),
                            // TODO: report keywords that have been searched for
                        ],
                    };

                    report(fault);

                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test() {
        let lint = IncompleteWork::init(vec![
            "TODO".into(),
            "FIXME".into(),
            "DEBUG".into(),
        ]);

        let path: PathBuf = "README.md".into();
        let content = concat!(
            "fn main() {\n",
            "    // TODO\n",
            "}\n",
        );

        let mut faults = vec![];
        let mut report = |fault| faults.push(fault);

        lint.review(&path, content, report);

        assert_eq!(faults.len(), 1);

        let got = format!("{}", faults[0]);
        let should_be = concat!(
            "warning: there should be no incomplete work\n",
            " --> README.md:2:7\n",
            "  |\n",
            "2 |     // TODO\n",
            "  |        ^^^^ keyword suggests incomplete work\n",
            "  |\n",
            "  = help: remove keyword after completing work\n",
        );

        assert_eq!(got, should_be);
    }
}
