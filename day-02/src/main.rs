mod cli_parser;

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl From<&str> for Direction {
    fn from(direction: &str) -> Direction {
        match direction.as_ref() {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => {
                panic!("Invalid direction string '{}'", direction)
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    magnitude: i32,
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Instruction {
        let parts: Vec<&str> = instruction.split(" ").collect();
        if parts.len() > 2 {
            panic!(
                "More than one space in instruction string '{}'",
                instruction
            );
        }
        if let [direction_str, magnitude_str] = &parts[..] {
            return Instruction {
                direction: Direction::from(direction_str.clone()),
                magnitude: magnitude_str.parse::<i32>().unwrap(),
            };
        }
        panic!("No space found in instruction string '{}'", instruction);
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(Instruction::from(line));
    }

    return instructions;
}

#[derive(Default, Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Default::default()
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Forward => {
                self.horizontal += instruction.magnitude;
                self.depth += self.aim * instruction.magnitude;
            }
            Direction::Up => {
                self.aim -= instruction.magnitude;
            }
            Direction::Down => {
                self.aim += instruction.magnitude;
            }
        }
    }
}

fn accumulate_instructions(instructions: Vec<Instruction>) -> Position {
    let mut pos = Position::new();
    for instruction in instructions {
        pos.apply_instruction(&instruction);
    }
    return pos;
}

fn main() {
    let args = cli_parser::parse_args();

    let file_contents = std::fs::read_to_string(args.file_path).expect("Error reading file.");
    let instructions = parse_instructions(&file_contents);
    let pos = accumulate_instructions(instructions);

    println!("Final position: {:?}", pos);
    println!("depth*horizontal position = {}", pos.depth * pos.horizontal);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! Instruction_from_string_tests {
        ($($name:ident: $value:expr,)*) => {$(
            #[test]
            fn $name() {
                let (input, expected_direction, expected_magnitude) = $value;
                let instruction = Instruction::from(input);

                assert_eq!(instruction.direction, expected_direction);
                assert_eq!(instruction.magnitude, expected_magnitude);
            }
        )*}
    }
    Instruction_from_string_tests! {
        up_4: ("up 4", Direction::Up, 4),
        down_2: ("down 2", Direction::Down, 2),
        forward_negative_10: ("forward -10", Direction::Forward, -10),
    }

    #[test]
    fn parse_instructions_generates_instructions() {
        let instructions_str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

        let instructions = parse_instructions(&instructions_str);

        assert_eq!(instructions.len(), 6);
        assert_eq!(instructions[0].direction, Direction::Forward);
        assert_eq!(instructions[1].direction, Direction::Down);
        assert_eq!(instructions[2].direction, Direction::Forward);
        assert_eq!(instructions[3].direction, Direction::Up);
        assert_eq!(instructions[4].direction, Direction::Down);
        assert_eq!(instructions[5].direction, Direction::Forward);
        assert_eq!(instructions[0].magnitude, 5);
        assert_eq!(instructions[1].magnitude, 5);
        assert_eq!(instructions[2].magnitude, 8);
        assert_eq!(instructions[3].magnitude, 3);
        assert_eq!(instructions[4].magnitude, 8);
        assert_eq!(instructions[5].magnitude, 2);
    }

    #[test]
    fn positions_are_correctly_accumulated() {
        let instructions_str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let instructions = parse_instructions(&instructions_str);

        let pos = accumulate_instructions(instructions);

        assert_eq!(pos.depth, 60);
        assert_eq!(pos.horizontal, 15);
    }
}
