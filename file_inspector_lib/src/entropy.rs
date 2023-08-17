use std::collections::HashMap;

pub struct EntropyCalculator;

impl EntropyCalculator {
    pub fn calculate_entropy(&self, data: HashMap<u8, i32>) -> f64 {
        let mut entropy = 0.0;
        let count_sum: i32 = data.iter().map(|d| d.1).sum();

        for byte in 0..u8::MAX {
            match data.get(&byte) {
                Some(count) => {
                    let relative_count = f64::from(*count) / f64::from(count_sum);
                    entropy += relative_count * relative_count.log2();
                }
                None => {}
            }
        }

        -entropy
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    pub fn calculate_entropy_empty_hashmap_given_correctly_calculated() {
        let values = HashMap::new();
        let entropy_calculator = EntropyCalculator;
        let entropy = entropy_calculator.calculate_entropy(values);

        assert_eq!(0.0, entropy);
    }

    #[test]
    pub fn calculate_entropy_values_given_correctly_calculated() {
        let values = (0..u8::MAX).map(|d| (d, 1)).collect();

        let entropy_calculator = EntropyCalculator;
        let entropy = entropy_calculator.calculate_entropy(values);

        assert_approx_eq!(8.0, entropy, 1e-2);
    }
}
