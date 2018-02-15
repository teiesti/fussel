use std::path::PathBuf;
use structopt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
#[structopt(raw(setting = "structopt::clap::AppSettings::GlobalVersion"))]
pub struct Opt {
    #[structopt(
        short = "C",
        long = "current-dir",
        name = "path",
        help = "Run as if fussel was started in <path> instead of the current working directory",
        parse(from_os_str),
    )]
    pub current_dir: Option<PathBuf>,
}
