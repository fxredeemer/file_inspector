use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CommandLineArguments {
    #[arg(short, long)]
    pub file: PathBuf,
}
