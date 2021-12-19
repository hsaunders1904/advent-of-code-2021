mod signal;

use advent_of_code::aoc_io;
use advent_of_code::cli;
use signal::Signal;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse_args();

    let mut signals: Vec<Signal> = Vec::new();
    for line in aoc_io::read_lines(args.file_path)? {
        let line = line?;
        if !line.trim().is_empty() {
            signals.push(Signal::new(&line)?);
        }
    }

    println!("Part 1: counted {} 1s, 4s, 7s and 8s.", part1(&signals));

    Ok(())
}

fn part1(signals: &Vec<Signal>) -> u64 {
    let digit_counts = count_digits_part1(signals);
    let mut count = 0;
    for digit in [1, 4, 7, 8] {
        count += digit_counts[digit];
    }
    count as u64
}

fn count_digits_part1(signals: &Vec<Signal>) -> [usize; 10] {
    let mut digit_counts = [0; 10];
    for signal in signals {
        for output in &signal.outputs {
            match output.len() {
                2 => digit_counts[1] += 1,
                3 => digit_counts[7] += 1,
                4 => digit_counts[4] += 1,
                7 => digit_counts[8] += 1,
                _ => {}
            }
        }
    }
    digit_counts
}
