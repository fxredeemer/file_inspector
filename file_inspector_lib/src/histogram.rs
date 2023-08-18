use std::collections::HashMap;

pub struct HistogramCreator;

impl HistogramCreator {
    pub fn get_file_histogram(&self, bytes: &Vec<u8>) -> HashMap<u8, i32> {
        let mut values = HashMap::new();

        for byte in bytes {
            let current_value = values.get(byte).unwrap_or(&0);
            values.insert(*byte, current_value + 1);
        }

        values
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn get_file_histogram_numbers_returns_correct_histogram() {
        let vec = vec![1, 2, 3, 3, 3, 2, 4, 155];
        let creator = HistogramCreator;
        let histogram = creator.get_file_histogram(&vec);

        let mut expected_histogram = HashMap::<u8, i32>::new();

        expected_histogram.insert(1, 1);
        expected_histogram.insert(2, 2);
        expected_histogram.insert(3, 3);
        expected_histogram.insert(4, 1);
        expected_histogram.insert(155, 1);

        assert_eq!(histogram, expected_histogram)
    }
}
