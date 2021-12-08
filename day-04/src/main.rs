mod cli_parser;
mod io;
mod part1;

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

fn main() {
    let args = cli_parser::parse_args();
    let (drawn_numbers, raw_boards) = io::read_input(&args.file_path);
    let mut boards = create_boards(raw_boards);

    match part1::get_winning_score(&drawn_numbers, &mut boards) {
        Some(score) => println!("BINGO! With score {}", score),
        None => println!("No winners!"),
    };
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
    fn correct_score_from_three_boards() {
        let buf = std::io::BufReader::new(read_from_string(INPUT));
        let (drawn_numbers, raw_boards) = io::parse_lines(buf.lines().by_ref());
        let mut boards = create_boards(raw_boards);

        assert_eq!(
            part1::get_winning_score(&drawn_numbers, &mut boards),
            Some(4512)
        );
    }
}
