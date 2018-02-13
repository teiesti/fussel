use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub trailing_whitespace: Option<TrailingWhitespace>,
}

impl Config {
    /// Loads a configuration from file given a path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // Open the config file read-only
        let mut file = File::open(&path).map_err(Error::from)?;

        // Read the config file
        let mut buf = String::new();
        file.read_to_string(&mut buf).map_err(Error::from)?;

        // Decode configuration
        let config: Config = toml::from_str(buf.as_str()).map_err(Error::from)?;

        Ok(config)
    }
}

#[derive(Debug, Deserialize)]
pub struct TrailingWhitespace {
    pub extension_blacklist: HashSet<String>,
}
