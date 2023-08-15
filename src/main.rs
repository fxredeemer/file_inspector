use std::{error::Error, fs::File, io::Read};

use plotter::Plotter;

use crate::histogram::HistogramCreator;

mod entropy;
mod histogram;
mod plotter;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("C:/_Source/_rust/file_inspector/src/main.rs")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let creator = HistogramCreator;
    let histogram = creator.get_file_histogram(buffer);

    let plotter = Plotter;

    plotter.plot_hiostogram(histogram);
    
    Ok(())
}
