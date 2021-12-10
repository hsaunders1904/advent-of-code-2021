mod io;
mod part1;

use advent_of_code::cli;
use ndarray::Array2;
use part1::Board;

fn create_boards(raw_boards: Vec<Array2<u32>>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    for board in raw_boards {
        boards.push(Board {
            values: board,
            dobbed: Array2::default((5, 5)),
        });
    }
    boards
}

fn get_last_to_win(drawn_numbers: &Vec<u32>, boards: &mut Vec<Board>) -> Option<u32> {
    for draw_idx in 0..4 {
        for board in boards.iter_mut() {
            part1::update_board(board, drawn_numbers[draw_idx]);
        }
    }

    let mut board_has_bingo: Vec<u32> = vec![0; boards.len()];
    for draw_idx in 4..drawn_numbers.len() {
        for (board_idx, board) in boards.iter_mut().enumerate() {
            part1::update_board(board, drawn_numbers[draw_idx]);
            match part1::board_has_bingo(board) {
                Some(sum) => {
                    board_has_bingo[board_idx] = 1;
                    if board_has_bingo.iter().sum::<u32>() as usize == board_has_bingo.len() {
                        println!("Last number  : {}", drawn_numbers[draw_idx]);
                        println!("Remaining sum: {}", sum);
                        println!("Board number : {}", board_idx + 1);
                        return Some(sum * drawn_numbers[draw_idx]);
                    }
                }
                None => {}
            };
        }
    }
    None
}

fn main() {
    let args = cli::parse_args();
    let (drawn_numbers, raw_boards) = io::read_input(&args.file_path);
    let mut boards = create_boards(raw_boards);

    let now = std::time::Instant::now();
    match part1::get_winning_score(&drawn_numbers, &mut boards) {
        Some(score) => println!("First BINGO! With score {}", score),
        None => println!("No winners!"),
    };
    println!("Took {} ms\n", now.elapsed().as_millis());

    let now = std::time::Instant::now();
    match get_last_to_win(&drawn_numbers, &mut boards) {
        Some(score) => println!("Final BINGO! With score {}", score),
        None => println!("No winners!"),
    }
    println!("Took {} ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    fn read_from_string(s: &str) -> &[u8] {
        s.as_bytes()
    }

    const INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn correct_first_bingo_score_from_three_boards() {
        let buf = std::io::BufReader::new(read_from_string(INPUT));
        let (drawn_numbers, raw_boards) = io::parse_lines(buf.lines().by_ref());
        let mut boards = create_boards(raw_boards);

        assert_eq!(
            part1::get_winning_score(&drawn_numbers, &mut boards),
            Some(4512)
        );
    }

    #[test]
    fn correct_final_bingo_score_from_three_boards() {
        let buf = std::io::BufReader::new(read_from_string(INPUT));
        let (drawn_numbers, raw_boards) = io::parse_lines(buf.lines().by_ref());
        let mut boards = create_boards(raw_boards);

        assert_eq!(get_last_to_win(&drawn_numbers, &mut boards), Some(1924));
    }
}
