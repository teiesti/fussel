use super::{ContentNode, DirectoryNode, FileNode, Node, ProjectNode};

pub(crate) trait Visitor {
    fn enter(&mut self, node: Node) -> bool; // returns whether to descend

    fn leave(&mut self, node: Node) -> bool; // returns whether to maintain
}

pub(crate) trait Traversable {
    fn traverse<V: Visitor>(self, visitor: &mut V) -> bool;
}

impl Traversable for ProjectNode {
    fn traverse<V: Visitor>(self, visitor: &mut V) -> bool {
        if visitor.enter(self.clone().into()) {
            let project = self.read().unwrap();
            project.root.clone().traverse(visitor);
        }
        visitor.leave(self.into())
    }
}

impl Traversable for DirectoryNode {
    fn traverse<V: Visitor>(self, visitor: &mut V) -> bool {
        if visitor.enter(self.clone().into()) {
            let directory = self.read().unwrap();
            if let Some(entries) = &directory.entries {
                let mut proceed = true;
                for directory in &entries.directories {
                    if !proceed {
                        break;
                    }
                    proceed = directory.clone().traverse(visitor);
                }
                for file in &entries.files {
                    if !proceed {
                        break;
                    }
                    proceed = file.clone().traverse(visitor);
                }
            } else {
                visitor.enter(Node::Truncated);
                visitor.leave(Node::Truncated);
            }
        }
        visitor.leave(self.into())
    }
}

impl Traversable for FileNode {
    fn traverse<V: Visitor>(self, visitor: &mut V) -> bool {
        if visitor.enter(self.clone().into()) {
            let file = self.read().unwrap();
            if let Some(content) = &file.content {
                content.clone().traverse(visitor);
            } else {
                visitor.enter(Node::Truncated);
                visitor.leave(Node::Truncated);
            }
        }
        visitor.leave(self.into())
    }
}

impl Traversable for ContentNode {
    fn traverse<V: Visitor>(self, visitor: &mut V) -> bool {
        visitor.enter(self.clone().into());
        visitor.leave(self.into())
    }
}
