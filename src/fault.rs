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
    pub fn new(p1: Position, p2: Position) -> Self {
        if p1 <= p2 {
            Self {
                lb: p1,
                ub: p2
            }
        } else {
            Self {
                lb: p2,
                ub: p1
            }
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

    pub fn lb(&self) -> Spot {
        Spot {
            path: self.path.clone(),
            pos: self.range.lb,
        }
    }

    pub fn ub(&self) -> Spot {
        Spot {
            path: self.path.clone(),
            pos: self.range.ub,
        }
    }

    pub fn bounds(&self) -> (Spot, Spot) {
        (self.lb(), self.ub())
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
        // Collect data
        let prefix      = self.canvas_prefix();
        let position    = self.mark.lb();
        let line        = self.mark.range.lb.row;
        let txt         = &self.txt;
        let offset      = " ".repeat(self.mark.range.lb.col - self.ctx.range.lb.col);
        let mark        = "^".repeat(self.mark.range.ub.col - self.mark.range.lb.col + 1);
        let msg         = &self.msg;

        // Write data
        writeln!(f, "{}--> {}"          , prefix, position              )?;
        writeln!(f, "{} |"              , prefix                        )?;
        writeln!(f, "{} |     {}"       , line, txt                     )?;
        writeln!(f, "{} |     {}{} {}"  , prefix, offset, mark, msg     )?;
        writeln!(f, "{} |"              , prefix                        )?;

        Ok(())
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
        write!(f, "{}", self.example)?;

        // Write all the hints
        let prefix = self.example.canvas_prefix();
        for msg in &self.hints {
            writeln!(f, "{} = {}", prefix, msg)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_ord() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 1);
        let p3 = Position::new(1, 0);
        let p4 = Position::new(1, 1);

        assert!(p1 < p2);
        assert!(p2 < p3);
        assert!(p3 < p4);
    }

    #[test]
    fn position_fmt() {
        let pos = Position::new(47, 11);
        assert_eq!(format!("{}", pos), "47:11");
    }

    #[test]
    fn range_ctor_symmetric() {
        let p1 = Position::new(0, 1);
        let p2 = Position::new(1, 0);

        let r1 = Range::new(p1, p2);
        let r2 = Range::new(p2, p1);

        assert_eq!(r1, r2);
    }

    #[test]
    fn range_fmt() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 1);
        let p3 = Position::new(1, 0);
        let p4 = Position::new(1, 1);

        let r1 = Range::new(p1, p1);
        let r2 = Range::new(p1, p2);
        let r3 = Range::new(p1, p3);
        let r4 = Range::new(p1, p4);

        assert_eq!(format!("{}", r1), "0:0");
        assert_eq!(format!("{}", r2), "0:0...1");
        assert_eq!(format!("{}", r3), "0:0...1:0");
        assert_eq!(format!("{}", r4), "0:0...1:1");
    }

    #[test]
    fn spot_fmt() {
        let spot = Spot::new(
            "src/main.rs".into(),
            Position::new(9, 11),
        );
        assert_eq!(format!("{}", spot), "src/main.rs:9:11");
    }

    #[test]
    fn scope_fmt() {
        let scope = Scope::new(
            "src/main.rs".into(),
            Range::new(
                Position::new(0, 815),
                Position::new(47, 11),
            ),
        );
        assert_eq!(format!("{}", scope), "src/main.rs:0:815...47:11");
    }

    #[test]
    fn message_fmt() {
        let error   = Message::error(  "lorem ipsum".into());
        let warning = Message::warning("lorem ipsum".into());
        let note    = Message::note(   "lorem ipsum".into());
        let help    = Message::help(   "lorem ipsum".into());

        assert_eq!(format!("{}", error  ), "error: lorem ipsum"  );
        assert_eq!(format!("{}", warning), "warning: lorem ipsum");
        assert_eq!(format!("{}", note   ), "note: lorem ipsum"   );
        assert_eq!(format!("{}", help   ), "help: lorem ipsum"   );
    }

    #[test]
    fn fault_fmt() {
        let fault = Fault {
            msg: Message::warning("tabs should be avoided".into()),
            example: Example {
                txt: "    let n = 42;".into(),
                msg: Message::bare("tab found here".into()),
                mark: Scope::new(
                    "src/main.rs".into(),
                    Range::new(
                        Position::new(9, 0),
                        Position::new(9, 3),
                    ),
                ),
                ctx: Scope::new(
                    "src/main.rs".into(),
                    Range::new(
                        Position::new(9, 0),
                        Position::new(9, 14),
                    ),
                ),
            },
            hints: vec![
                Message::help("use spaces instead of tabs".into()),
            ],
        };

        let got = format!("{}", fault);
        let should_be = concat!(
            "warning: tabs should be avoided\n",
            " --> src/main.rs:9:0\n",
            "  |\n",
            "9 |         let n = 42;\n",
            "  |     ^^^^ tab found here\n",
            "  |\n",
            "  = help: use spaces instead of tabs\n",
        );

        assert_eq!(got, should_be);
    }
}
