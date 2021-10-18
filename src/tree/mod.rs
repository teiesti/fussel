pub(crate) mod expand;
pub(crate) mod load;
pub(crate) mod print;
pub(crate) mod traverse;

use {
    self::{load::Loader, traverse::Traversable},
    std::{
        sync::{Arc, RwLock},
        fs::{canonicalize, metadata, Metadata},
        io::Result,
        path::{Path, PathBuf},
    },
};

#[derive(Clone, Debug)]
pub(crate) enum Node {
    Truncated,
    Project(ProjectNode),
    Directory(DirectoryNode),
    File(FileNode),
    Content(ContentNode),
}

macro_rules! impl_node_from {
    ($variant:ident, $from:ident, $conv:expr) => {
        impl From<$from> for Node {
            fn from(x: $from) -> Node {
                Node::$variant($conv(x))
            }
        }
    };
}

impl_node_from!(Project, Project, wrap);
impl_node_from!(Project, ProjectNode, |x| x);
impl_node_from!(Directory, Directory, wrap);
impl_node_from!(Directory, DirectoryNode, |x| x);
impl_node_from!(File, File, wrap);
impl_node_from!(File, FileNode, |x| x);
impl_node_from!(Content, Content, wrap);
impl_node_from!(Content, ContentNode, |x| x);

type Wrapped<I> = Arc<RwLock<I>>;

pub(crate) fn wrap<I>(inner: I) -> Wrapped<I> {
    Arc::new(RwLock::new(inner))
}

pub(crate) type ProjectNode = Wrapped<Project>;

pub(crate) type DirectoryNode = Wrapped<Directory>;

pub(crate) type FileNode = Wrapped<File>;

pub(crate) type ContentNode = Wrapped<Content>;

#[derive(Debug)]
pub(crate) struct Project {
    root: Wrapped<Directory>,
}

impl Project {
    pub(crate) fn open<P: AsRef<Path>>(path: P) -> Result<ProjectNode> { // TODO return type?
        let path = canonicalize(path)?;
        let metadata = metadata(path.as_path())?;

        let root = wrap(Directory {
            path,
            metadata,
            entries: None,
        });

        let project = wrap(Project { root });

        project.clone().traverse(&mut Loader);

        Ok(project)
    }

    pub(crate) fn discover<P: AsRef<Path>>(path: P) -> Result<ProjectNode> {
        todo!()
    }
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
    content: Option<Wrapped<Content>>,
}

impl File {
    pub(crate) fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub(crate) fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[derive(Debug)]
pub(crate) enum Content {
    Binary(Vec<u8>),
    Text(String),
}
