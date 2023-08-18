use crate::command_line::CommandLineArguments;
use anyhow::Result;
use clap::Parser;
use file_inspector_lib::histogram::HistogramCreator;
use plotter::Plotter;
use std::{fs::File, io::Read};

mod command_line;
mod plotter;

fn main() -> Result<()> {
    let command_line_args = CommandLineArguments::parse();

    let mut file = File::open(command_line_args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let creator = HistogramCreator;
    let histogram = creator.get_file_histogram(&buffer);

    let plotter = Plotter;

    plotter.plot_histogram(histogram)
}
