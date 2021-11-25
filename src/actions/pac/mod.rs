use clap::Parser;

mod unpack;
pub use unpack::*;

mod compression;

#[derive(Parser)]
pub enum Command {
    #[clap(about = "Unpack the archive into individual files")]
    Unpack(UnpackOpts),
}

impl Command {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            Command::Unpack(opts) => unpack::unpack_pac(opts),
        }
    }
}
