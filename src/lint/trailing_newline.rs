use fault::Fault;
use lint::Lint;
use traverse::Project;

use failure::Error;

pub struct TrailingNewline {
    pub project: Project,
}

impl Lint for TrailingNewline {}

impl IntoIterator for TrailingNewline {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        unimplemented!();
    }
}
