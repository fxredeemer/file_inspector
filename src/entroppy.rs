use std::{collections::HashMap, intrinsics::log2f64};

pub struct EntropyCalculator;

impl EntropyCalculator {
    pub fn calculate_entropy(data: HashMap<u8, i32>) -> f64 {

        let mut entropy = 0;
        let count_sum: i32 = data.iter().map(|d| d.1).sum();

        for byte in 0..u8::MAX {
            match data.get(&byte)
            {
                Some(count) => {
                    let relative_count = count / count_sum;
                    let asd = relative_count * relative_count.;
                    entropy = entropy + 
                },
                None => {},
            }
        }

        1.0
    }
}
