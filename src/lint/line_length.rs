use fault::Fault;
use lint::Lint;
use traverse::Project;

use failure::Error;

pub struct LineLength {
    project: Project,
    length: usize,
}

impl Lint for LineLength {
    fn review(project: Project) -> Self {
        Self {
            project,
        }
    }
}

impl IntoIterator for LineLength {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        unimplemented!();
    }
}
