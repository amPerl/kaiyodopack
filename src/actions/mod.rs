use clap::Parser;

mod pac;

#[derive(Parser)]
pub enum FileTypeCommand {
    #[clap(about = "Actions for PAC archive files")]
    Pac {
        #[clap(subcommand, about = "subcommand to run")]
        cmd: pac::Command,
    },
}

impl FileTypeCommand {
    pub fn process(self) -> anyhow::Result<()> {
        match self {
            FileTypeCommand::Pac { cmd } => cmd.process(),
        }
    }
}
