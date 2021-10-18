use std::{
    sync::{Arc, RwLock},
    path::{PathBuf, Path},
    fs::Metadata,
};

#[derive(Clone, Debug)]
pub(crate) enum Node {
    Truncated,
    Project(ProjectNode),
    Directory(DirectoryNode),
    File(FileNode),
    Content(ContentNode),
}

type Wrapped<I> = Arc<RwLock<I>>;

pub(crate) type ProjectNode = Wrapped<Project>;

pub(crate) type DirectoryNode = Wrapped<Directory>;

pub(crate) type FileNode = Wrapped<File>;

pub(crate) type ContentNode = Wrapped<Content>;

#[derive(Debug)]
pub(crate) struct Project {
    root: Wrapped<Directory>,
}

#[derive(Debug)]
pub(crate) struct Directory {
    path: PathBuf,
    metadata: Metadata,
    entries: Option<Entries>,
}

impl Directory {
    pub(crate) fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub(crate) fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Debug)]
struct Entries {
    directories: Vec<Wrapped<Directory>>,
    files: Vec<Wrapped<File>>,
}

#[derive(Debug)]
pub(crate) struct File {
    path: PathBuf,
    metadata: Metadata,
    entries: Option<Wrapped<Content>>,
}

#[derive(Debug)]
pub(crate) enum Content {
    Binary(Vec<u8>),
    Text(String),
}
