use super::{
    Node,
    expand::Expandable,
    traverse::{Visitor},
};

pub(crate) struct Loader;

impl Visitor for Loader {
    fn enter(&mut self, node: Node) -> bool {
        match node {
            Node::Truncated => unreachable!(),
            Node::Project(_) => true,
            Node::Directory(directory) => {
                directory.write().unwrap().expand().unwrap(); // TODO remove unwrap
                true
            }
            Node::File(file) => {
                file.write().unwrap().expand().unwrap(); // TODO remove unwrap
                false
            }
            Node::Content(_) => unreachable!(),
        }
    }

    fn leave(&mut self, _: Node) -> bool {
        true
    }
}
