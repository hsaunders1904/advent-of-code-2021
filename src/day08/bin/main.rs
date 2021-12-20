mod signal;

use advent_of_code::aoc_io;
use advent_of_code::cli;
use signal::Signal;

use std::error::Error;
use std::iter::FromIterator;

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
    println!("Part 2: outputs summed to {}.", part2(&signals));

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

fn part2(signals: &Vec<Signal>) -> u64 {
    let mut total: u64 = 0;
    for signal in signals {
        let signal_encodings = decode_signals(&signal.signals);

        let mut number: u64 = 0;
        for output in &signal.outputs {
            number += decode_signal(&output, &signal_encodings);
            number *= 10;
        }
        total += number;
    }

    total / 10
}

fn decode_signal(signal: &str, encodings: &[String; 10]) -> u64 {
    match encodings.iter().position(|r| r == &sort_chars(signal)) {
        Some(x) => x as u64,
        None => panic!("Could not decode signal '{}'.", signal),
    }
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

fn first_contains_chars_in_second(first: &str, second: &str) -> bool {
    if first.len() < second.len() {
        return false;
    }
    for ch in second.chars() {
        if !first.contains(ch) {
            return false;
        }
    }
    true
}

fn is_five_not_two(signal: &str, encoded_four: &str, encoded_one: &str) -> bool {
    let mut top_left_and_middle = String::from(encoded_four);
    for ch in encoded_one.chars() {
        top_left_and_middle = top_left_and_middle.replace(ch, "");
    }

    first_contains_chars_in_second(&signal, &top_left_and_middle)
}

fn sort_chars(string: &str) -> String {
    let mut char_vec: Vec<char> = string.chars().collect();
    char_vec.sort_by(|a, b| b.cmp(a));
    String::from_iter(char_vec)
}

fn decode_signals(signals: &[String; 10]) -> [String; 10] {
    let mut encodings: [String; 10] = Default::default();
    for signal in signals {
        match signal.len() {
            2 => encodings[1] = sort_chars(signal),
            4 => encodings[4] = sort_chars(signal),
            3 => encodings[7] = sort_chars(signal),
            7 => encodings[8] = sort_chars(signal),
            _ => {}
        }
    }

    for signal in signals {
        match signal.len() {
            5 => {
                // Of the 3 digits that are encoded using 5 characters, only
                // "3" contains all the characters in "7"
                if first_contains_chars_in_second(signal, &encodings[7]) {
                    encodings[3] = sort_chars(signal);
                } else if is_five_not_two(signal, &encodings[4], &encodings[1]) {
                    encodings[5] = sort_chars(signal);
                } else {
                    encodings[2] = sort_chars(signal);
                }
            }
            6 => {
                // Of the 3 digits that are encoded using 6 characters, only
                // "9" contains all the characters used to encode a "4"
                if first_contains_chars_in_second(signal, &encodings[4]) {
                    encodings[9] = sort_chars(signal);
                } else if first_contains_chars_in_second(signal, &encodings[1]) {
                    encodings[0] = sort_chars(signal);
                } else {
                    encodings[6] = sort_chars(signal);
                }
            }
            _ => {}
        }
    }
    encodings
}

#[test]
fn signal_patterns_decoded_correctly() {
    let signal_patterns = [
        String::from("acedgfb"),
        String::from("cdfbe"),
        String::from("gcdfa"),
        String::from("fbcad"),
        String::from("dab"),
        String::from("cefabd"),
        String::from("cdfgeb"),
        String::from("eafb"),
        String::from("cagedb"),
        String::from("ab"),
    ];

    let decodings = decode_signals(&signal_patterns);

    assert_eq!(decodings[0], "gedcba");
    assert_eq!(decodings[1], "ba");
    assert_eq!(decodings[2], "gfdca");
    assert_eq!(decodings[3], "fdcba");
    assert_eq!(decodings[4], "feba");
    assert_eq!(decodings[5], "fedcb");
    assert_eq!(decodings[6], "gfedcb");
    assert_eq!(decodings[7], "dba");
    assert_eq!(decodings[8], "gfedcba");
    assert_eq!(decodings[9], "fedcba");
}
