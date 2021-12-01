extern crate clap;
use clap::{Arg, App};

use std::str::FromStr;

struct InputArgs {
    file_path: String
}

fn parse_args() -> InputArgs {
    let matches = App::new("Advent of code day 1")
        .version("0.0.1")
        .about("Count the number of times a depth measurement increases")
        .arg(Arg::with_name("data_file")
            .value_name("DATA_FILE")
            .takes_value(true)
            .help("Text file containing data - one integer per line")
            .required(true)
        ).get_matches();

    InputArgs{
        file_path: matches.value_of("data_file").unwrap().to_owned()
    }
}

fn read_lines<T: FromStr>(file_path: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_path)
        .expect(&format!("file '{}' not found!", file_path))
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn count_increasing_numbers<T: std::cmp::PartialOrd>(values: &Vec<T>) -> u64 {
    let mut count = 0;
    for idx in 0..(values.len() - 1) {
        if values[idx] < values[idx + 1] {
            count += 1;
        }
    }
    return count;
}


fn main() {
    let args = parse_args();
    println!("Input file: {}", args.file_path);

    let depth_measurements: Vec<i32> = read_lines::<i32>(&args.file_path).into_iter().flatten().collect();
    println!("Found {} depth measurements.", depth_measurements.len());

    let num_increasing_measurements = count_increasing_numbers(&depth_measurements);
    println!("{} measurements were increasing", num_increasing_measurements);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increasing_numbers_counts_increasing_numbers_correctly() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increasing = count_increasing_numbers(&depths);
        assert_eq!(num_increasing, 7);
    }
}