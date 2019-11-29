use crate::fault::Fault;

use std::fs::Metadata;
use std::path::Path;

pub(crate) trait Lint {
    #[allow(unused_variables)]
    fn ignore(&self, path: &Path, metadata: &Metadata) -> bool {
        metadata.is_dir()
    }

    fn review<F: FnMut(Fault)>(&self, path: &Path, content: &str, report: F);
}
