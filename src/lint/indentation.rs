use fault::Fault;
use lint::Lint;
use traverse::Project;

use failure::Error;

pub struct Indentation {
    project: Project,
}

impl Lint for Indentation {
    fn review(project: Project) -> Self {
        Self {
            project,
        }
    }
}

impl IntoIterator for Indentation {
    type Item = Result<Fault, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(mut self) -> Self::IntoIter {
        unimplemented!();
    }
}
