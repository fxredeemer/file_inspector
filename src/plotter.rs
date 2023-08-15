use anyhow;
use std::collections::{BTreeMap, HashMap};
use terminal_size::terminal_size;

pub struct Plotter;

impl Plotter {
    pub fn plot_hiostogram(&self, counts: HashMap<u8, i32>) -> anyhow::Result<()> {
        let (width, height) = terminal_size().ok_or(PlottingError::TerminalSizeNotDeterminable)?;
        println!(
            "The terminal is {} cols wide and {} lines tall",
            width.0, height.0
        );
        let width = width.0 as i32;
        let height = height.0.clamp(0, 30) as i32;

        if width < 128 {
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
            .map(|d| (d.0, (d.1 as f64 / max_count) * height as f64))
            .collect();

        for line in 0..height {
            print!("|");
            for byte in 0..127 {
                if relative_counts.get(&byte).unwrap_or(&0.0).clone() >= (height - line) as f64 {
                    print!("0");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PlottingError {
    #[error("Unable To Determine the Terminal Size")]
    TerminalSizeNotDeterminable,
    #[error("Invalid histogram")]
    InvalidHistogram,
    #[error("Terminal too small")]
    TerminalTooSmall,
}
