use advent_of_code::cli;

fn main() {
    let args = cli::parse_args();
    let positions: Vec<i64> = std::fs::read_to_string(args.file_path)
        .expect("Could not read file.")
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().expect("Cannot parse value as i64."))
        .collect();

    println!("Constant fuel cost  : {:?}", find_cost_part1(&positions));
    println!("Cumulative fuel cost: {:?}", find_cost_part2(&positions));
}

fn find_cost_part1(positions: &Vec<i64>) -> (i64, i64) {
    determine_cheapest_position(positions, &sum_distances)
}

fn find_cost_part2(positions: &Vec<i64>) -> (i64, i64) {
    determine_cheapest_position(positions, &sum_cost)
}

fn sum_distances(positions: &Vec<i64>, pos: i64) -> i64 {
    positions
        .iter()
        .fold(0, |sum, pos_i| sum + (pos - pos_i).abs())
}

fn sum_cost(positions: &Vec<i64>, pos: i64) -> i64 {
    positions.iter().fold(0, |sum, pos_i| {
        let dist = (pos - pos_i).abs();
        let cost = (dist * (dist + 1)) / 2;
        sum + cost
    })
}

fn find_min_max(values: &Vec<i64>) -> (i64, i64) {
    values
        .iter()
        .fold((i64::MAX, i64::MIN), |m, &v_i| (m.0.min(v_i), m.1.max(v_i)))
}

fn determine_cheapest_position(
    positions: &Vec<i64>,
    cost_func: &dyn Fn(&Vec<i64>, i64) -> i64,
) -> (i64, i64) {
    let mut min_cost_pos = i64::MAX;
    let mut min_cost = i64::MAX;
    let (min, max) = find_min_max(&positions);
    for pos in min..max {
        let cost = cost_func(&positions, pos);
        if cost < min_cost {
            min_cost_pos = pos;
            min_cost = cost;
        }
    }
    (min_cost_pos, min_cost)
}
