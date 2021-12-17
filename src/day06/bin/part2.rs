pub fn simulate_iters(initial_state: &Vec<usize>, iters: usize) -> usize {
    // The index represents the number of days until the fish replicate,
    // the value at that index represents the number of fish with that many
    // days until they replicate
    let mut counts_per_days: Vec<usize> = vec![0; 9];

    for count in initial_state {
        counts_per_days[*count] += 1;
    }

    for _ in 0..iters {
        let num_to_regen = counts_per_days[0];
        for j in 0..(counts_per_days.len() - 1) {
            counts_per_days[j] = counts_per_days[j + 1];
        }
        counts_per_days[6] += num_to_regen;
        counts_per_days[8] = num_to_regen;
    }
    counts_per_days.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_count_after_10_iters() {
        let initial_state = vec![3, 4, 3, 1, 2];

        let count = simulate_iters(&initial_state, 10);

        assert_eq!(count, 12);
    }

    #[test]
    fn correct_list_of_numbers_after_18_iters() {
        let initial_state = vec![3, 4, 3, 1, 2];

        let count = simulate_iters(&initial_state, 18);

        assert_eq!(count, 26);
    }
}
