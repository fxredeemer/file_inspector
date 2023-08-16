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

        let max = 127;

        if width < 130 {
            return Err(PlottingError::TerminalTooSmall.into());
        }

        print!("╔");
        for _ in 0..max {
            print!("═");
        }
        print!("╗");
        println!();

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
            print!("║");
            let mut first_byte_set = false;
            let mut second_byte_set = false;

            for byte in 0..max * 2 + 1 {
                let first_index = byte;
                let second_index = byte + 1;

                if byte % 2 == 0 {
                    first_byte_set = relative_counts.get(&first_index).unwrap_or(&0.0).clone()
                        >= (height - line) as f64;
                } else {
                    second_byte_set = relative_counts.get(&second_index).unwrap_or(&0.0).clone()
                        >= (height - line) as f64;

                    if first_byte_set && second_byte_set {
                        print!("▉");
                    } else if first_byte_set {
                        print!("▌");
                    } else if second_byte_set {
                        print!("▐");
                    } else {
                        print!(" ");
                    }
                }
            }
            print!("║");
            println!();
        }

        print!("╚");
        for _ in 0..max {
            print!("═")
        }
        print!("╝");
        println!();
        for byte in 0..max * 2 + 1 {
            if byte % 16 == 0 {
                print!("{byte:<8}");
            }
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
