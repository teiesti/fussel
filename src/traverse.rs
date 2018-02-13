use failure::Error;
use git2::Repository;
use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

pub struct Project {
    pub root: PathBuf,
    pub respect_gitignore: bool,
    pub extension_blacklist: HashSet<OsString>,
}

impl Project {
    pub fn open(root: PathBuf) -> Self {
        Self {
            root,
            respect_gitignore: true,
            extension_blacklist: HashSet::new(),
        }
    }

    pub fn open_current_dir() -> Result<Self, Error> {
        Ok(Self::open(::std::env::current_dir()?))
    }

    pub fn open_git_workdir() -> Result<Self, Error> {
        let pwd = ::std::env::current_dir().map_err(Error::from)?;
        let repo = Repository::discover(pwd).map_err(Error::from)?;
        let workdir = repo.workdir().ok_or(format_err!("git repository is bare"))?.to_path_buf();
            // TODO improve error message?
        Ok(Self::open(workdir))
    }

    pub fn files(self) -> Files {
        Files {
            project: self
        }
    }

    pub fn readers(self) -> Readers {
        self.files().readers()
    }

    pub fn lines(self) -> Lines {
        self.readers().lines()
    }

    fn try_into_iter(self) -> Result<<Self as IntoIterator>::IntoIter, Error> {
        // Initialize an empty set of filters
        let mut subtree_filter: Vec<Box<Fn(&DirEntry) -> bool>> = vec![];
        let mut node_filter:    Vec<Box<Fn(&DirEntry) -> bool>> = vec![];

        // Add filter: Entries ignored by Git
        if self.respect_gitignore {
            let repo = Repository::discover(&self.root).map_err(Error::from)?;
            subtree_filter.push(Box::new(
                move |entry| !repo.is_path_ignored(entry.path()).unwrap_or(true)
            ));
        }

        // Add filter: Entries that are no files
        node_filter.push(Box::new(
            |entry| entry.path().is_file()
        ));

        // Add filter: Extension blacklist
        if !self.extension_blacklist.is_empty() {
            let exts = self.extension_blacklist; // prevents move of self
            node_filter.push(Box::new(
                move |entry| entry.path().extension().map_or(true, |ext| {
                    !exts.contains(ext)
                })
            ));
        }

        Ok(Box::new(
            // Create an iterator that traverses recursively through the directory
            WalkDir::new(&self.root).into_iter()

            // Apply all subtree filters
            .filter_entry(move |x| {
                subtree_filter.iter().all(|f| f(x))
            })

            // Apply all node filters
            .filter(move |x| {
                match *x {
                    Ok (ref x) => node_filter.iter().all(|f| f(x)),
                    Err(_)     => true,
                }
            })

            // Convert foreign types into ours
            .map(|x| {
                match x {
                    Ok (x) => Ok (x.path().to_path_buf()),
                    Err(e) => Err(e.into()),
                }
            })
        ))
    }
}

impl IntoIterator for Project {
    type Item = Result<PathBuf, Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self
            .try_into_iter()
            .unwrap_or_else(|e| {
                Box::new(
                    iter::once(Err(e))
                )
            })
    }
}

pub struct Files {
    project: Project,
}

impl Files {
    pub fn readers(self) -> Readers {
        Readers {
            files: self
        }
    }

    pub fn lines(self) -> Lines {
        self.readers().lines()
    }
}

impl IntoIterator for Files {
    type Item = Result<(PathBuf, File), Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.project
                .into_iter()
                .map(|x| {
                    x.and_then(|path| {
                        let file = File::open(&path).map_err(Error::from)?;
                        Ok((path, file))
                    })
                })
        )
    }
}

pub struct Readers {
    files: Files,
}

impl Readers {
    pub fn lines(self) -> Lines {
        Lines {
            readers: self,
        }
    }
}

impl IntoIterator for Readers {
    type Item = Result<(PathBuf, BufReader<File>), Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.files
                .into_iter()
                .map(|x| {
                    x.and_then(|(path, file)| {
                        let reader = BufReader::new(file);
                        Ok((path, reader))
                    })
                })
        )
    }
}

pub struct Lines {
    readers: Readers,
}

impl IntoIterator for Lines {
    type Item = Result<(PathBuf, usize, String), Error>;
    type IntoIter = Box<Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.readers
                .into_iter()
                .flat_map(|x| -> Box<Iterator<Item = _>> {
                    match x {
                        Ok((path, reader)) => Box::new(
                            reader
                                .lines()
                                .enumerate()
                                .map(move |(i, line)| {
                                    let line = line.map_err(Error::from)?;
                                    Ok((path.clone(), i + 1, line))
                                })
                        ),
                        Err(err) => Box::new(iter::once(Err(err))),
                    }
                })
        )
    }
}
