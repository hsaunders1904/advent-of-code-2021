use advent_of_code::cli;
use bit_field::BitField;
use std::io::BufRead;

fn read_lines_to_decimal<T>(lines: std::io::Lines<T>) -> Vec<u64>
where
    T: std::io::BufRead,
{
    let mut decimals: Vec<u64> = Vec::new();
    for (line_num, line) in lines
        .map(|x| u64::from_str_radix(&x.unwrap(), 2))
        .enumerate()
    {
        match line {
            Err(why) => panic!("Could not parse line '{}': {}", line_num + 1, why),
            Ok(line) => decimals.push(line),
        }
    }
    return decimals;
}

// Count the number of 1s in each bit position in each value
fn get_bit_counts(values: &Vec<u64>, num_bits: usize) -> Vec<u64> {
    let mut counts: Vec<u64> = vec![0; num_bits];
    for &value in values {
        for bit_idx in 0..num_bits {
            if value.get_bit(num_bits - bit_idx - 1) {
                counts[bit_idx] += 1;
            }
        }
    }
    return counts;
}

fn calculate_consumption(values: &Vec<u64>, num_bits: usize) -> u64 {
    let bit_counts = get_bit_counts(&values, num_bits);

    let mut gamma_rate: u64 = 0;
    let mut epsilon: u64 = 0;
    for (bit_idx, count) in bit_counts.iter().enumerate() {
        let most_common_bit = *count as f64 / values.len() as f64 > 0.5;
        gamma_rate.set_bit(num_bits - bit_idx - 1, most_common_bit);
        epsilon.set_bit(num_bits - bit_idx - 1, !most_common_bit);
    }
    println!("gamma_rate: {}", gamma_rate);
    println!("epsilon: {}", epsilon);
    return gamma_rate * epsilon;
}

fn count_ones_in_bit_pos(values: &Vec<u64>, bit_pos: usize) -> usize {
    let mut count: usize = 0;
    for value in values {
        if value.get_bit(bit_pos) {
            count += 1;
        }
    }
    count
}

fn get_most_common_bit(values: &Vec<u64>, bit_pos: usize) -> bool {
    let ones_count = count_ones_in_bit_pos(values, bit_pos);
    ones_count as f64 >= 0.5 * values.len() as f64
}

fn get_oxygen_rating(mut values: Vec<u64>, num_bits: usize) -> u64 {
    let mut bit_idx = num_bits;
    while values.len() > 1 {
        bit_idx -= 1;
        let most_common_bit = get_most_common_bit(&values, bit_idx);
        values = values
            .into_iter()
            .filter_map(|x| {
                if x.get_bit(bit_idx) == most_common_bit {
                    Some(x)
                } else {
                    None
                }
            })
            .collect();
    }
    values[0]
}

fn get_co2_rating(mut values: Vec<u64>, num_bits: usize) -> u64 {
    let mut bit_idx = num_bits;
    while values.len() > 1 {
        bit_idx -= 1;
        let most_common_bit = get_most_common_bit(&values, bit_idx);
        values = values
            .into_iter()
            .filter_map(|x| {
                if x.get_bit(bit_idx) == most_common_bit {
                    None
                } else {
                    Some(x)
                }
            })
            .collect();
    }
    values[0]
}

fn get_life_support_rating(values: &Vec<u64>, num_bits: usize) -> u64 {
    let oxygen_rating = get_oxygen_rating(values.clone(), num_bits);
    println!("oxygen_rating: {}", oxygen_rating);
    let co2_rating = get_co2_rating(values.clone(), num_bits);
    println!("co2_rating: {}", co2_rating);
    oxygen_rating * co2_rating
}

fn main() {
    let args = cli::parse_args();

    let file = match std::fs::File::open(&args.file_path) {
        Err(why) => panic!(
            "Could not open file '{}': {}",
            args.file_path,
            std::io::Error::to_string(&why)
        ),
        Ok(file) => file,
    };

    let num_bits = 12;
    let reader = std::io::BufReader::new(file);
    let values = read_lines_to_decimal(reader.lines());
    let consumption = calculate_consumption(&values, num_bits);
    println!("Power consumption: {}", consumption);

    let ls_rating = get_life_support_rating(&values, num_bits);
    println!("Life support rating: {}", ls_rating);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIAG_REPORT: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n\
            11100\n10000\n11001\n00010\n01010";

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }

    #[test]
    fn correct_power_consumption() {
        let buf = std::io::BufReader::new(read_from_string(DIAG_REPORT));
        let values = read_lines_to_decimal(buf.lines());

        let power_consumption = calculate_consumption(&values, 5);

        assert_eq!(power_consumption, 198);
    }

    #[test]
    fn correct_oxygen_rating() {
        let buf = std::io::BufReader::new(read_from_string(DIAG_REPORT));
        let values = read_lines_to_decimal(buf.lines());

        let o_ratio = get_oxygen_rating(values, 5);

        assert_eq!(o_ratio, 23);
    }

    #[test]
    fn correct_co2_rating() {
        let buf = std::io::BufReader::new(read_from_string(DIAG_REPORT));
        let values = read_lines_to_decimal(buf.lines());

        let co2_ratio = get_co2_rating(values, 5);

        assert_eq!(co2_ratio, 10);
    }

    #[test]
    fn correct_life_support_rating() {
        let buf = std::io::BufReader::new(read_from_string(DIAG_REPORT));
        let values = read_lines_to_decimal(buf.lines());

        let ls_rating = get_life_support_rating(&values, 5);

        assert_eq!(ls_rating, 230);
    }
}
