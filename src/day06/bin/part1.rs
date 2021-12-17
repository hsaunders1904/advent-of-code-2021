pub fn simulate_iters(mut state: Vec<usize>, iters: usize) -> usize {
    for _ in 0..iters {
        for idx in 0..state.len() {
            if state[idx] == 0 {
                state[idx] = 6;
                state.push(8);
            } else {
                state[idx] -= 1;
            }
        }
    }
    state.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_count_after_10_iters() {
        let initial_state = vec![3, 4, 3, 1, 2];

        let count = simulate_iters(initial_state, 10);

        assert_eq!(count, 12);
    }

    #[test]
    fn correct_list_of_numbers_after_18_iters() {
        let initial_state = vec![3, 4, 3, 1, 2];

        let count = simulate_iters(initial_state, 18);

        assert_eq!(count, 26);
    }
}
