mod indentation;
mod line_break;
mod line_length;
mod trailing_newline;
mod trailing_whitespace;

pub use self::indentation::Indentation;
pub use self::line_break::LineBreak;
pub use self::line_length::LineLength;
pub use self::trailing_newline::TrailingNewline;
pub use self::trailing_whitespace::TrailingWhitespace;

use fault::Fault;

use failure::Error;

pub trait Lint: IntoIterator<
    Item = Result<Fault, Error>,
    IntoIter = Box<Iterator<Item = <Self as IntoIterator>::Item>>
> {}
