use advent_of_code::cli;
use std::io::BufRead;

fn main() {
    let args = cli::parse_args();
    let lines = read_lines(&args.file_path);
    let max_val = get_maximum(&lines);
    let mut grid = build_grid(max_val + 1, max_val + 1);
    for line in lines {
        draw_line(&mut grid, &line);
    }
    println!("Found {} values greater than 1", count_gt_value(&grid, 1));
}

fn count_gt_value(grid: &Vec<Vec<i32>>, value: i32) -> u32 {
    let mut count: u32 = 0;
    for row in grid {
        for v in row {
            if v > &value {
                count += 1;
            }
        }
    }
    count
}

fn draw_line(grid: &mut Vec<Vec<i32>>, line: &LineSegment) {
    for coord in line.coordinates() {
        grid[coord.1 as usize][coord.0 as usize] += 1;
    }
}

fn build_grid(x_size: usize, y_size: usize) -> Vec<Vec<i32>> {
    vec![vec![0; y_size]; x_size]
}

fn get_maximum(lines: &Vec<LineSegment>) -> usize {
    let mut max: usize = 0;
    for line in lines {
        let max_val = line.y.0.max(line.y.1).max(line.x.0.max(line.x.1));
        max = max.max(max.max(max_val as usize) as usize);
    }
    max
}

#[derive(Debug, PartialEq)]
struct LineSegment {
    x: (i32, i32),
    y: (i32, i32),
}

impl LineSegment {
    fn coordinates(&self) -> Vec<(i32, i32)> {
        if self.x.0 == self.x.1 {
            return self.coordinates_vertical();
        } else if self.y.0 == self.y.1 {
            return self.coordinates_horizontal();
        } else {
            return self.coordinates_diagonal();
        }
    }

    fn coordinates_vertical(&self) -> Vec<(i32, i32)> {
        let mut coords = Vec::new();
        for y in self.y.0.min(self.y.1)..(self.y.0.max(self.y.1) + 1) {
            coords.push((self.x.0, y));
        }
        coords
    }

    fn coordinates_horizontal(&self) -> Vec<(i32, i32)> {
        let mut coords = Vec::new();
        for x in self.x.0.min(self.x.1)..(self.x.0.max(self.x.1) + 1) {
            coords.push((x, self.y.0));
        }
        coords
    }

    fn coordinates_diagonal(&self) -> Vec<(i32, i32)> {
        let mut coords = Vec::new();
        let dx = if self.x.1 > self.x.0 { 1 } else { -1 };
        let dy = if self.y.1 > self.y.0 { 1 } else { -1 };
        let mut x = self.x.0;
        let mut y = self.y.0;
        while x != self.x.1 + dx {
            coords.push((x, y));
            x += dx;
            y += dy;
        }
        coords
    }
}

impl std::fmt::Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{} -> {},{}", self.x.0, self.y.0, self.x.1, self.y.1)
    }
}

fn parse_line(line: &String) -> LineSegment {
    let point_strs = line.split(" -> ").collect::<Vec<&str>>();
    let mut points = Vec::new();
    for point_str in point_strs {
        for num in point_str.split(",") {
            points.push(num.parse().expect(""));
        }
    }
    LineSegment {
        x: (points[0], points[2]),
        y: (points[1], points[3]),
    }
}

fn read_lines(file_path: &String) -> Vec<LineSegment> {
    let file = match std::fs::File::open(file_path) {
        Err(why) => panic!(
            "Could not open file '{}': {}",
            file_path,
            std::io::Error::to_string(&why)
        ),
        Ok(file) => file,
    };
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|x| parse_line(&x.unwrap())).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_creates_line_from_valid_string() {
        let line_str = String::from("0,9 -> 5,9");
        let expected_line = LineSegment {
            x: (0, 5),
            y: (9, 9),
        };

        assert_eq!(parse_line(&line_str), expected_line);
    }
}
