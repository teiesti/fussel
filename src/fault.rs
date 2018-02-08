use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.row, self.col)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Range {
    pub lb: Position,
    pub ub: Position,
}

impl Range {
    pub fn new(lb: Position, ub: Position) -> Self {
        if lb <= ub {
            Self { lb, ub }
        } else {
            Self { ub, lb }
        }
    }

    pub fn bounds(&self) -> (Position, Position) {
        (self.lb, self.ub)
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.lb <= other.lb && other.ub <= self.ub
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        unimplemented!()
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (lb, ub) = self.bounds();
        match (lb.row == ub.row, lb.col == ub.col) {
            ( true, true ) => lb.fmt(f),
            ( true, false) => write!(f, "{}:{}...{}", lb.row, lb.col, ub.col),
            (false,     _) => write!(f, "{}...{}", lb, ub),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Spot {
    pub path: PathBuf,
    pub pos: Position,
}

impl Spot {
    pub fn new(path: PathBuf, pos: Position) -> Self {
        Self { path, pos }
    }
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.path.display(), self.pos)
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Scope {
    pub path: PathBuf,
    pub range: Range,
}

impl Scope {
    pub fn new(path: PathBuf, range: Range) -> Self {
        Self { path, range }
    }

    pub fn bounds(&self) -> (Spot, Spot) {
        let lb = Spot {
            path: self.path.clone(),
            pos: self.range.lb,
        };
        let ub = Spot {
            path: self.path.clone(),
            pos: self.range.ub,
        };
        (lb, ub)
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.path == other.path && self.range.contains(&other.range)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.path == other.path && self.range.overlaps(&other.range)
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.path.display(), self.range)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub enum Level {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Message {
    pub lvl: Option<Level>,
    pub txt: String,
}

impl Message {
    pub fn bare(txt: String) -> Self {
        let lvl = None;
        Self { lvl, txt }
    }

    pub fn with_lvl(lvl: Level, txt: String) -> Self {
        let lvl = Some(lvl);
        Self { lvl, txt }
    }

    pub fn error(txt: String) -> Self {
        Self::with_lvl(Level::Error, txt)
    }

    pub fn warning(txt: String) -> Self {
        Self::with_lvl(Level::Warning, txt)
    }

    pub fn note(txt: String) -> Self {
        Self::with_lvl(Level::Note, txt)
    }

    pub fn help(txt: String) -> Self {
        Self::with_lvl(Level::Help, txt)
    }

    pub fn is_bare(&self) -> bool {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Example {
    pub txt: String,
    pub msg: Message,
    pub mark: Scope,
    pub ctx: Scope,
}

impl Example {
    // TODO add one or more ctors

    pub(self) fn canvas_prefix(&self) -> String {
        let len = format!("{}", self.ctx.range.lb.row).len(); // TODO enhance efficiency
        " ".repeat(len)
    }
}

impl fmt::Display for Example {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!() // TODO
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Fault {
    pub msg: Message,
    pub example: Example,
    pub hints: Vec<Message>,
}

impl Fault {
    // TODO add one or more ctors
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the message
        writeln!(f, "{}", self.msg)?;

        // Write the example
        writeln!(f, "{}", self.example)?;

        // Write all the hints
        let prefix = self.example.canvas_prefix();
        for msg in &self.hints {
            writeln!(f, "{} = {}", prefix, msg)?;
        }

        Ok(())
    }
}
