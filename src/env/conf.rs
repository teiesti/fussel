use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Opt {
    pub trailing_whitespace: Option<TrailingWhitespace>,
}

impl Opt {
    /// Attempt to load a configuration file at `path`.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Open the config file read-only
        let mut file = File::open(&path).map_err(Error::from)?;

        // Read the config file
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(Error::from)?;

        // Decode the configuration
        let opt: Opt = toml::from_str(buf.as_str()).map_err(Error::from)?;

        Ok(opt)
    }

    /// Attempt to load a configuration at or above `path`.
    pub fn discover<P: AsRef<Path>>(path: P) -> Result<(Self, PathBuf), Error> {
        let mut path = path.as_ref();

        loop {
            if path.join("Fussel.toml").is_file() {
                return Ok((Self::load(path.join("Fussel.toml"))?, path.to_path_buf()));
            }

            path = match path.parent() {
                Some(path) => path,
                None => bail!("configuration file not found"),
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TrailingWhitespace {
    pub extension_blacklist: HashSet<String>,
}
