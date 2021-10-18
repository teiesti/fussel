pub(crate) mod tree;

use crate::tree::{print::Printer, traverse::Traversable, Project};

fn main() {
    let project = Project::open(".").unwrap() ;
    project.traverse(&mut Printer::new());
}
