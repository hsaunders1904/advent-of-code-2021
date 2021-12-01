extern crate clap;
use clap::{App, Arg};

use std::str::FromStr;

struct InputArgs {
    file_path: String,
}

fn parse_args() -> InputArgs {
    let matches = App::new("Advent of code day 1")
        .version("0.0.1")
        .about("Count the number of times a depth measurement increases")
        .arg(
            Arg::with_name("data_file")
                .value_name("DATA_FILE")
                .takes_value(true)
                .help("Text file containing data - one integer per line")
                .required(true),
        )
        .get_matches();

    InputArgs {
        file_path: matches.value_of("data_file").unwrap().to_owned(),
    }
}

fn read_lines<T: FromStr>(file_path: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_path)
        .expect(&format!("file '{}' not found!", file_path))
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn count_increasing_numbers(values: &Vec<i32>, window_size: usize) -> u64 {
    let mut count = 0;

    for idx in 0..(values.len() - window_size) {
        let window1 = &values[idx..(idx + window_size)];
        let window2 = &values[(idx + 1)..(idx + window_size + 1)];
        if window1.iter().sum::<i32>() < window2.iter().sum::<i32>() {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let args = parse_args();
    println!("Input file: {}", args.file_path);

    let depth_measurements: Vec<i32> = read_lines::<i32>(&args.file_path)
        .into_iter()
        .flatten()
        .collect();
    println!("Found {} depth measurements.", depth_measurements.len());

    println!(
        "{} measurements were increasing with window size 1",
        count_increasing_numbers(&depth_measurements, 1)
    );
    println!(
        "{} measurements were increasing with window size 3",
        count_increasing_numbers(&depth_measurements, 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increasing_numbers_counts_increasing_numbers_correctly() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increasing = count_increasing_numbers(&depths, 1);
        assert_eq!(num_increasing, 7);
    }

    #[test]
    fn count_increasing_numbers_counts_correctly_with_3_value_sliding_window() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increasing = count_increasing_numbers(&depths, 3);
        assert_eq!(num_increasing, 5);
    }
}
