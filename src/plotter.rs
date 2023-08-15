use std::{collections::HashMap, error::Error};
use terminal_size::{terminal_size, Height, Width};

pub struct Plotter;

impl Plotter {
    pub fn plot_hiostogram(&self, counts: HashMap<u8, i32>) -> Result<(), Box<dyn Error>> {
        let max_count = counts
            .iter()
            .map(|d| d.1)
            .max()
            .ok_or(PlottingError::InvalidHistogram)?;

        let (w, h) = terminal_size().ok_or(PlottingError::TerminalSizeNotDeterminable)?;
        println!("The terminal is {} cols wide and {} lines tall", w.0, h.0);

        


        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PlottingError {
    #[error("Unable To Determine the TErminal Size")]
    TerminalSizeNotDeterminable,
    #[error("Invalid histogram")]
    InvalidHistogram,
}
