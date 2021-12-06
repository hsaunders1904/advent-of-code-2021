mod cli_parser;

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
    for value in values {
        for bit_idx in 0..num_bits {
            let filter = 1 << (num_bits - bit_idx - 1);
            if value & filter == filter {
                counts[bit_idx] += 1;
            }
        }
    }
    return counts;
}

fn calculate_consumption(values: Vec<u64>, num_bits: usize) -> u64 {
    let bit_counts = get_bit_counts(&values, num_bits);

    let mut gamma_rate: u64 = 0;
    let mut epsilon: u64 = 0;
    for count in &bit_counts {
        if *count as f64 / values.len() as f64 > 0.5 {
            gamma_rate += 1;
        } else {
            epsilon += 1;
        }
        gamma_rate <<= 1;
        epsilon <<= 1;
    }
    gamma_rate >>= 1;
    epsilon >>= 1;

    println!("gamma_rate: {}", gamma_rate);
    println!("epsilon: {}", epsilon);
    return gamma_rate * epsilon;
}

fn main() {
    let args = cli_parser::parse_args();

    let file = match std::fs::File::open(&args.file_path) {
        Err(why) => panic!(
            "Could not open file '{}': {}",
            args.file_path,
            std::io::Error::to_string(&why)
        ),
        Ok(file) => file,
    };

    let reader = std::io::BufReader::new(file);
    let values = read_lines_to_decimal(reader.lines());
    let consumption = calculate_consumption(values, 12);
    println!("Power consumption: {}", consumption);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }

    #[test]
    fn correct_power_consumption() {
        let diag_report =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let buf = std::io::BufReader::new(read_from_string(diag_report));
        let values = read_lines_to_decimal(buf.lines());
        let power_consumption = calculate_consumption(values, 5);

        assert_eq!(power_consumption, 198);
    }
}
