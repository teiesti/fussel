use fault::Fault;
use lint::Lint;
use traverse::Project;

use failure::Error;

pub struct LineBreak {
    pub project: Project,
}

impl Lint for LineBreak {}

impl IntoIterator for LineBreak {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        unimplemented!();
    }
}
