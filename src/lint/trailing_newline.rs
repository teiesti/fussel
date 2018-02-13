use fault::Fault;
use lint::Lint;
use traverse::Project;

use failure::Error;

pub struct TrailingNewline {
    project: Project,
    strict: bool,
}

impl Lint for TrailingNewline {
    fn review(project: Project) -> Self {
        Self {
            project,
        }
    }
}

impl IntoIterator for TrailingNewline {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        unimplemented!();
    }
}
