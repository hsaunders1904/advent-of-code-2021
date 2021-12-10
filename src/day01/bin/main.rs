use advent_of_code::cli;

use std::str::FromStr;

fn read_lines<T: FromStr>(file_path: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_path)
        .expect(&format!("file '{}' not found!", file_path))
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn count_increasing_window_sums(values: &Vec<i32>, window_size: usize) -> u64 {
    if window_size > values.len() {
        return 0;
    }

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
    let args = cli::parse_args();

    let depth_measurements: Vec<i32> = read_lines::<i32>(&args.file_path)
        .into_iter()
        .flatten()
        .collect();
    println!("Found {} depth measurements.", depth_measurements.len());

    println!(
        "{} measurements were increasing with window size 1",
        count_increasing_window_sums(&depth_measurements, 1)
    );
    println!(
        "{} measurements were increasing with window size 3",
        count_increasing_window_sums(&depth_measurements, 3)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increasing_window_sums_correct_count_given_window_size_is_1() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increasing = count_increasing_window_sums(&depths, 1);
        assert_eq!(num_increasing, 7);
    }

    #[test]
    fn count_increasing_window_sums_correct_count_given_window_size_is_3() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increasing = count_increasing_window_sums(&depths, 3);
        assert_eq!(num_increasing, 5);
    }

    #[test]
    fn count_increasing_window_sums_returns_0_given_window_size_gt_values_len() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let num_increasing = count_increasing_window_sums(&depths, depths.len() + 1);
        assert_eq!(num_increasing, 0);
    }
}
