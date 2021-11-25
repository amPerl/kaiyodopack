use clap::Parser;

mod actions;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "amPerl")]
struct Opts {
    #[clap(subcommand, about = "file type to perform action on")]
    file_type: actions::FileTypeCommand,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    opts.file_type.process()
}
