use std::{error::Error, fs::File, io::Read};

use clap::Parser;
use plotter::Plotter;
use file_inspector_lib::histogram::HistogramCreator;

use crate::command_line::CommandLineArguments;

mod command_line;
mod plotter;

fn main() -> Result<(), Box<dyn Error>> {
    let command_line_args = CommandLineArguments::parse();

    let mut file = File::open(command_line_args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let creator = HistogramCreator;
    let histogram = creator.get_file_histogram(buffer);

    let plotter = Plotter;

    plotter.plot_hiostogram(histogram);

    Ok(())
}
