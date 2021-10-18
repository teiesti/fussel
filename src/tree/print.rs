use super::{
    Node,
    traverse::{Visitor},
};

pub(crate) struct Printer {
    level: usize,
}

impl Printer {
    pub(crate) fn new() -> Self {
        Self { level: 0 }
    }
}

impl Visitor for Printer {
    fn enter(&mut self, node: Node) -> bool {
        for _ in 0..self.level {
            print!("    ");
        }
        self.level += 1;

        match node {
            Node::Truncated => println!("Truncated"),
            Node::Project(_) => println!("Project"),
            Node::Directory(directory) => {
                println!("Directory: {}", directory.read().unwrap().path().display())
            },
            Node::File(directory) => println!("File: {}", directory.read().unwrap().path().display()),
            Node::Content(content) => println!("Content") // TODO
        }

        true
    }

    fn leave(&mut self, _: Node) -> bool {
        self.level -= 1;
        true
    }
}
