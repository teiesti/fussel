mod trailing_whitespace;

pub use self::trailing_whitespace::TrailingWhitespace;

use fault::Fault;
use traverse::Project;

use failure::Error;

pub trait Lint: IntoIterator<
    Item = Result<Fault, Error>,
    IntoIter = Box<Iterator<Item = <Self as IntoIterator>::Item>>
> {
    fn review(project: Project) -> Self;
}
