mod cli_parser;

use std::io::BufRead;

fn calculate_consumption<T>(mut lines: std::io::Lines<T>) -> usize
where
    T: std::io::BufRead,
{
    let first_line = lines.next().unwrap().unwrap();
    let num_bits = first_line.len();

    let mut running_totals: Vec<f64> = vec![0.0; num_bits];
    let mut num_lines = 0;
    for line in lines {
        let x = line.unwrap();
        for (bit_pos, bit_str) in x.trim().chars().enumerate() {
            match bit_str {
                '0' => {}
                '1' => running_totals[bit_pos] += 1.0,
                ' ' => {}
                _ => {
                    panic!("Bad character '{}' in line", bit_str)
                }
            }
        }
        num_lines += 1;
    }

    let bits: Vec<bool> = running_totals
        .iter()
        .map(|x| (x / f64::from(num_lines)) > 0.5)
        .collect();

    let mut gamma_rate: usize = 0;
    let mut epsilon: usize = 0;
    for bit in bits {
        if bit {
            gamma_rate += 1;
        } else {
            epsilon += 1;
        }
        gamma_rate <<= 1;
        epsilon <<= 1;
    }
    gamma_rate >>= 1;
    epsilon >>= 1;

    println!("gamma rate: {}, {:b}", gamma_rate, gamma_rate);
    println!("epsilon: {}, {:b}", epsilon, epsilon);
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
    let consumption = calculate_consumption(reader.lines());
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
        let diag_report = r"00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        ";
        let buf = std::io::BufReader::new(read_from_string(diag_report));

        let power_consumption = calculate_consumption(buf.lines());

        assert_eq!(power_consumption, 198);
    }
}
