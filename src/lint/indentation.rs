use fault::Fault;
use lint::Lint;
use traverse::Project;

use failure::Error;

pub struct Indentation {
    pub project: Project,
}

impl Lint for Indentation {}

impl IntoIterator for Indentation {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        unimplemented!();
    }
}
