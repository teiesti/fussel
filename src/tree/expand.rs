use {
    super::{wrap, Content, Directory, Entries, File},
    std::{
        fs::{read, read_dir},
        io::Result,
    },
};

pub(crate) trait Expandable {
    fn expand(&mut self) -> Result<()>;

    fn collapse(&mut self);
}

impl Expandable for Directory {
    fn expand(&mut self) -> Result<()> {
        if self.entries.is_some() {
            return Ok(());
        }

        let mut directories = vec![];
        let mut files = vec![];

        for entry in read_dir(self.path())? {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                directories.push(wrap(Directory {
                    path,
                    metadata,
                    entries: None,
                }))
            } else if metadata.is_file() {
                files.push(wrap(File {
                    path,
                    metadata,
                    content: None,
                }))
            }
        }

        self.entries = Some(Entries { directories, files });

        Ok(())
    }

    fn collapse(&mut self) {
        self.entries = None;
    }
}

impl Expandable for File {
    fn expand(&mut self) -> Result<()> {
        if self.content.is_some() {
            return Ok(());
        }

        let raw = read(self.path())?;
        let content = match String::from_utf8(raw) {
            Ok(string) => Content::Text(string),
            Err(error) => Content::Binary(error.into_bytes()),
        };

        self.content = Some(wrap(content));

        Ok(())
    }

    fn collapse(&mut self) {
        self.content = None;
    }
}
