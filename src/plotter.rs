use anyhow;
use std::collections::{HashMap, BTreeMap};
use terminal_size::terminal_size;

pub struct Plotter;

impl Plotter {
    pub fn plot_hiostogram(&self, counts: HashMap<u8, i32>) -> anyhow::Result<()> {
        let (width, height) = terminal_size().ok_or(PlottingError::TerminalSizeNotDeterminable)?;
        println!(
            "The terminal is {} cols wide and {} lines tall",
            width.0, height.0
        );

        if width.0 < 128 {
            return Err(PlottingError::TerminalTooSmall.into());
        }

        let max_count: f64 = counts
            .iter()
            .map(|d| d.1)
            .max()
            .ok_or(PlottingError::InvalidHistogram)?
            .clone()
            .into();

        let relative_counts: BTreeMap<u8, f64> = counts
            .into_iter()
            .map(|d| (d.0, (d.1 as f64 / max_count) * height.0 as f64))
            .collect();

        println!("{:?}", relative_counts);

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PlottingError {
    #[error("Unable To Determine the TErminal Size")]
    TerminalSizeNotDeterminable,
    #[error("Invalid histogram")]
    InvalidHistogram,
    #[error("Terminal too small")]
    TerminalTooSmall,
}
