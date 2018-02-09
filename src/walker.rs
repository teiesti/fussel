use failure::Error;
use git2::Repository;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Walker {
    paths: Box<Iterator<Item = Result<PathBuf, Error>>>,
}

impl Walker {
    pub fn default() -> Result<Self, Error> {
        Self::build()?.finalize()
    }

    pub fn build() -> Result<WalkerBuilder, Error> {
        WalkerBuilder::default()
    }
}

impl Iterator for Walker {
    type Item = Result<PathBuf, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.paths.next()
    }
}

pub struct WalkerBuilder {
    pub root: PathBuf,
    pub respect_gitignore: bool,
}

impl WalkerBuilder {
    pub fn default() -> Result<Self, Error> {
        let root = git_workdir()?;
        let respect_gitignore = true;
        Ok(Self{ root, respect_gitignore })
    }

    pub fn finalize(self) -> Result<Walker, Error> {
        // Create an iterator that traverses recursively through a directory
        let paths = WalkDir::new(&self.root).into_iter();

        // Filter files ignored by Git -- if necessary
        let repo = Repository::discover(&self.root).map_err(Error::from)?;
        let paths = paths.filter_entry(
            move |entry| {
                !self.respect_gitignore ||
                !repo.is_path_ignored(entry.path()).unwrap()
                    // TODO remove unwrap --> .or_else(true) ???
            }
        );

        // Filter entries that are no files
        let paths = paths.filter(|x| {
            match *x {
                Ok(ref entry) => entry.path().is_file(),
                Err(_) => true,
            }
        });

        // Convert foreign types to ours
        let paths = paths.map(|x| {
            match x {
                Ok(entry) => Ok(entry.path().to_path_buf()),
                Err(err) => Err(err.into()),
            }
        });

        // Box the iterator
        let paths = Box::new(paths);

        // Return
        Ok(Walker{ paths })
    }

    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = root;
        self
    }

    pub fn root_current_dir(mut self) -> Result<Self, Error> {
        self.root = ::std::env::current_dir().map_err(Error::from)?.to_path_buf();
        Ok(self)
    }

    pub fn root_git_workdir(mut self) -> Result<Self, Error> {
        self.root = git_workdir()?;
        Ok(self)
    }

    pub fn respect_gitignore(mut self, yes: bool) -> Self {
        self.respect_gitignore = yes;
        self
    }
}

fn git_workdir() -> Result<PathBuf, Error> {
    let pwd = ::std::env::current_dir().map_err(Error::from)?;
    let repo = Repository::discover(pwd).map_err(Error::from)?;
    let workdir = repo.workdir().ok_or(format_err!("git repository is bare"))?.to_path_buf();
        // TODO improve error message?
    Ok(workdir)
}
