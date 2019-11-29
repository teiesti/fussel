use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Fault {
    pub(crate) msg: Message,
    pub(crate) quote: Quote,
    pub(crate) hints: Vec<Message>,
}

impl Fault {
    pub(crate) fn simple(msg: Message) -> Self {
        Self {
            msg,
            quote: Quote::None,
            hints: Vec::new(),
        }
    }
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Display the message
        writeln!(f, "{}", self.msg)?;

        // Display the quote
        write!(f, "{}", self.quote)?;

        // Display all the hints
        let prefix = self.quote.canvas_prefix();
        for msg in &self.hints {
            writeln!(f, "{} = {}", prefix, msg)?;
        }

        Ok(())
    }
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum Quote {
    None,
    File {
        path: PathBuf
    },
    Excerpt {
        path: PathBuf,
        row: usize,
        col_from: usize,
        col_to: usize,
        txt: String,
        msg: Message,
    },
}

impl Quote {
    pub(self) fn canvas_prefix(&self) -> String {
        let len = match self {
            Self::Excerpt { row, .. } => format!("{}", row).len(),
            _ => 1,
        };
        " ".repeat(len)
    }
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => {},

            Self::File { path } => {
                writeln!(f, "{}--> {}", self.canvas_prefix(), path.display())?;
            },

            Self::Excerpt { path, row, col_from, col_to, txt, msg } => {
                let prefix      = self.canvas_prefix();
                let path        = path.display();
                let offset      = " ".repeat(*col_from);
                let underline   = "^".repeat(*col_to - *col_from + 1);

                writeln!(f, "{}--> {}:{}:{}"    , prefix, path, row, col_from       )?;
                writeln!(f, "{} |"              , prefix                            )?;
                writeln!(f, "{} | {}"           , row, txt                          )?;
                writeln!(f, "{} | {}{} {}"      , prefix, offset, underline, msg    )?;
                writeln!(f, "{} |"              , prefix                            )?;
            },
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Message {
    pub(crate) lvl: Option<Level>,
    pub(crate) txt: String,
}


impl Message {
    pub(crate) fn bare<S>(txt: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lvl: None,
            txt: txt.into(),
        }
    }

    pub(crate) fn with_lvl<S>(lvl: Level, txt: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lvl: Some(lvl),
            txt: txt.into(),
        }
    }

    pub(crate) fn error<S>(txt: S) -> Self
    where
        S: Into<String>,
    {
        Self::with_lvl(Level::Error, txt)
    }

    pub(crate) fn warning<S>(txt: S) -> Self
    where
        S: Into<String>,
    {
        Self::with_lvl(Level::Warning, txt)
    }

    pub(crate) fn note<S>(txt: S) -> Self
    where
        S: Into<String>,
    {
        Self::with_lvl(Level::Note, txt)
    }

    pub(crate) fn help<S>(txt: S) -> Self
    where
        S: Into<String>,
    {
        Self::with_lvl(Level::Help, txt)
    }

    pub(crate) fn is_bare(&self) -> bool {
        self.lvl.is_none()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.lvl {
            Some(lvl) => write!(f, "{}: {}", lvl, self.txt),
            None      => write!(f, "{}", self.txt),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub(crate) enum Level {
    Error,
    Warning,
    Note,
    Help,
}

impl Level {
    pub fn to_str(&self) -> &'static str {
        match *self {
            Level::Error    => "error",
            Level::Warning  => "warning",
            Level::Note     => "note",
            Level::Help     => "help",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str()) // TODO add color support
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fault_simple() {
        let msg = Message::error("lorem ipsum");
        let fault = Fault::simple(msg);
        assert_eq!(format!("{}", fault), "error: lorem ipsum\n");
    }

    #[test]
    fn fault_quote_file() {
        let fault = Fault {
            msg: Message::error("cannot read file"),
            quote: Quote::File { path: "src/main.rs".into() },
            hints: vec![],
        };

        let got = format!("{}", fault);
        let should_be = concat!(
             "error: cannot read file\n",
             " --> src/main.rs\n",
        );

        assert_eq!(got, should_be);
    }

    #[test]
    fn fault_quote_excerpt() {
        let fault = Fault {
            msg: Message::error("tabs should be avoided"),
            quote: Quote::Excerpt {
                path: "src/main.rs".into(),
                row: 42,
                col_from: 0,
                col_to: 3,
                txt: "    let n = 4711;".into(),
                msg: Message::bare("tab found here"),
            },
            hints: vec![
                Message::help("use spaces instead of tabs"),
            ],
        };

        let got = format!("{}", fault);
        let should_be = concat!(
            "error: tabs should be avoided\n",
            "  --> src/main.rs:42:0\n",
            "   |\n",
            "42 |     let n = 4711;\n",
            "   | ^^^^ tab found here\n",
            "   |\n",
            "   = help: use spaces instead of tabs\n",
        );

        assert_eq!(got, should_be);
    }
}
