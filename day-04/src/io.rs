use ndarray::Array2;
use std::io::BufRead;

#[derive(Debug)]
pub struct Bingo {
    pub boards: Vec<Array2<u8>>,
    pub drawn_numbers: Vec<u8>,
}

fn read_drawn_numbers<T>(lines: &mut std::io::Lines<T>) -> Vec<u8>
where
    T: std::io::BufRead,
{
    let drawn_numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    return drawn_numbers;
}

fn read_board<T>(lines: &mut std::io::Lines<T>) -> Result<Array2<u8>, Array2<u8>>
where
    T: std::io::BufRead,
{
    let mut board = Array2::<u8>::zeros((5, 5));
    for (row_idx, line_res) in lines.enumerate() {
        let line = line_res.unwrap();
        if line.trim().is_empty() {
            return Ok(board);
        }
        for (col_idx, value) in line.split_whitespace().enumerate() {
            board[[row_idx, col_idx]] = value.parse::<u8>().unwrap();
        }
    }
    return Err(board);
}

fn open_file(file_path: &str) -> std::io::BufReader<std::fs::File> {
    let file = match std::fs::File::open(file_path) {
        Err(why) => panic!(
            "Could not open file '{}': {}",
            file_path,
            std::io::Error::to_string(&why)
        ),
        Ok(file) => file,
    };
    std::io::BufReader::new(file)
}

pub fn parse_lines<T>(lines: &mut std::io::Lines<T>) -> Bingo
where
    T: std::io::BufRead,
{
    let drawn_numbers = read_drawn_numbers(lines.by_ref());
    lines.next();
    let mut boards: Vec<Array2<u8>> = Vec::new();
    loop {
        match read_board(lines.by_ref()) {
            Ok(board) => boards.push(board),
            Err(board) => {
                boards.push(board);
                break;
            }
        }
    }

    Bingo {
        boards: boards,
        drawn_numbers: drawn_numbers,
    }
}

pub fn read_input(file_path: &str) -> Bingo {
    let reader = open_file(file_path);
    let mut lines = reader.lines();
    parse_lines(lines.by_ref())
}
