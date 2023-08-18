use std::collections::{BTreeMap, HashMap};
use terminal_size::terminal_size;

pub struct Plotter;

impl Plotter {
    pub fn plot_histogram(&self, counts: HashMap<u8, i32>) -> anyhow::Result<()> {
        let (width, height) = terminal_size().ok_or(PlottingError::TerminalSizeNotDeterminable)?;

        let width = width.0 as i32;
        let height = height.0.clamp(20, 40) as i32;

        if width < 130 {
            Err(PlottingError::TerminalTooSmall)?;
        }

        let max = 127;
        print!("╔");
        for _ in 0..max {
            print!("═");
        }
        println!("╗");

        let max_count: f64 = (*counts
            .iter()
            .map(|d| d.1)
            .max()
            .ok_or(PlottingError::InvalidHistogram)?)
        .into();

        let relative_counts: BTreeMap<u8, f64> = counts
            .into_iter()
            .map(|d| (d.0, (d.1 as f64 / max_count) * height as f64))
            .collect();

        for line in 0..height {
            print!("║");

            let mut first_byte_set = false;

            for byte in 0..max * 2 + 1 {
                let line_threshold = (height - line) as f64;

                if byte % 2 == 0 {
                    first_byte_set = *relative_counts.get(&byte).unwrap_or(&0.0) >= line_threshold;
                } else {
                    let second_byte_set =
                        *relative_counts.get(&(byte + 1)).unwrap_or(&0.0) >= line_threshold;

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
            println!("║");
        }

        print!("╚");
        for _ in 0..max {
            print!("═")
        }
        println!("╝");

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
