mod part1;
mod part2;

use advent_of_code::cli;

fn main() {
    let args = cli::parse_args();
    let input_str = match std::fs::read_to_string(args.file_path) {
        Err(why) => panic!("Could not read file: {}", why),
        Ok(value) => value,
    };

    let initial_state = parse_comma_separated_values(&input_str);

    let days = 80;
    let count = part1::simulate_iters(initial_state.clone(), days);
    println!("After {} days, found {} fish.", days, count);

    let days = 256;
    let count = part2::simulate_iters(&initial_state, days);
    println!("After {} days, found {} fish.", days, count);
}

fn parse_comma_separated_values(value_str: &String) -> Vec<usize> {
    let values = value_str
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().expect("Cannot interpret value as int"))
        .collect();
    values
}
