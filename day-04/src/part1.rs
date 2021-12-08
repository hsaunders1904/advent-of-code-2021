use ndarray::Array2;

pub struct Board {
    pub values: Array2<u32>,
    pub dobbed: Array2<bool>,
}

pub fn update_board(board: &mut Board, value: u32) {
    for (idx, board_value) in board.values.indexed_iter() {
        if *board_value == value {
            board.dobbed[idx] = true;
        }
    }
}

fn sum_where_false(values: &Array2<u32>, mask: &Array2<bool>) -> u32 {
    let mut sum = 0;
    for (idx, value) in values.indexed_iter() {
        if !mask[idx] {
            sum += value;
        }
    }
    sum
}

pub fn board_has_bingo(board: &Board) -> Option<u32> {
    let dobbed = &board.dobbed;

    for row_idx in 0..5 {
        let row = dobbed.slice(ndarray::s![row_idx, ..]);
        let num_dobbed = row.fold(0, |x, y| x + *y as u32);
        if num_dobbed == 5 {
            return Some(sum_where_false(&board.values, dobbed));
        }
    }

    for col_idx in 0..5 {
        let col = dobbed.slice(ndarray::s![.., col_idx]);
        let num_dobbed = col.fold(0, |x, y| x + *y as u32);
        if num_dobbed == 5 {
            return Some(sum_where_false(&board.values, dobbed));
        }
    }
    None
}

pub fn get_winning_score(drawn_numbers: &Vec<u32>, boards: &mut Vec<Board>) -> Option<u32> {
    for draw_idx in 0..4 {
        for board in boards.iter_mut() {
            update_board(board, drawn_numbers[draw_idx]);
        }
    }

    for draw_idx in 4..drawn_numbers.len() {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            update_board(board, drawn_numbers[draw_idx]);
            match board_has_bingo(board) {
                Some(sum) => {
                    println!("Last number  : {}", drawn_numbers[draw_idx]);
                    println!("Remaining sum: {}", sum);
                    println!("Board number : {}", board_idx + 1);
                    return Some(sum * drawn_numbers[draw_idx]);
                }
                None => {}
            };
        }
    }
    None
}
