use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl FromStr for Color {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stripped = match s.strip_prefix("(") {
            Some(without) => without,
            None => s,
        };
        match stripped.strip_prefix("#") {
            Some(hexstring) => match (
                hexstring.get(0..2),
                hexstring.get(2..4),
                hexstring.get(4..6),
            ) {
                (Some(reds), Some(greens), Some(blues)) => match (
                    u8::from_str_radix(reds, 16),
                    u8::from_str_radix(greens, 16),
                    u8::from_str_radix(blues, 16),
                ) {
                    (Ok(red), Ok(green), Ok(blue)) => Ok(Self { red, green, blue }),
                    _ => Err(()),
                },
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Not a direction!"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position(isize, isize);

impl Position {
    fn row(&self) -> isize {
        self.0
    }
    fn column(&self) -> isize {
        self.1
    }
    fn step(&mut self, direction: Direction, amount: isize) {
        match direction {
            Direction::Up => self.0 -= amount,
            Direction::Down => self.0 += amount,
            Direction::Left => self.1 -= amount,
            Direction::Right => self.1 += amount,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Instruction {
    direction: Direction,
    distance: isize,
}
impl Instruction {
    fn new(direction: Direction, distance: isize) -> Self {
        Self {
            direction,
            distance,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hex_iter = s.split("#").skip(1).nth(0).unwrap().chars();
        let distance_hex = String::from_iter(hex_iter.clone().take(5));
        let dir: Direction = match hex_iter.nth(5) {
            Some('0') => Direction::Right,
            Some('1') => Direction::Down,
            Some('2') => Direction::Left,
            Some('3') => Direction::Up,
            _ => panic!("Invalid direction hex"),
        };

        let distance = isize::from_str_radix(&distance_hex, 16).unwrap();

        Ok(Instruction::new(dir, distance))
    }
}

struct InstructionsState {
    cursor: Position,
}
fn adjustment_for(direction1: &Direction, direction2: &Direction) -> (isize, isize) {
    match (direction1, direction2) {
        (Direction::Right, other) | (other, Direction::Right) => match other {
            Direction::Up => (0, 0),
            Direction::Down => (0, 1),
            _ => panic!("Direction combination is invalid!"),
        },
        (Direction::Left, other) | (other, Direction::Left) => match other {
            Direction::Up => (1, 0),
            Direction::Down => (1, 1),
            _ => panic!("Direction combination is invalid!"),
        },
        _ => panic!("Direction combination is invalid!"),
    }
}

impl InstructionsState {
    fn new() -> InstructionsState {
        InstructionsState {
            cursor: Position::default(),
        }
    }

    fn get_area(&self, coordinates: Vec<Position>) -> isize {
        let mut prev: Position = *coordinates.last().unwrap();
        let mut running_dividend = 0;
        for coord in coordinates {
            running_dividend += (coord.row() * prev.column()) - (prev.row() * coord.column());
            prev = coord;
        }

        running_dividend.abs() / 2
    }

    fn get_coordinates(&mut self, instruction: &Instruction) -> Position {
        self.cursor
            .step(instruction.direction, instruction.distance);

        self.cursor.clone()
    }

    fn process_instructions(&mut self, instructions: &Vec<Instruction>) -> isize {
        let coords = instructions.iter().map(|i| self.get_coordinates(i));
        let directions = instructions
            .iter()
            .map(|i| i.direction)
            .collect::<Vec<Direction>>();
        let dir_copy = directions.clone();
        let mut next_directions = dir_copy.iter();
        let first_next_dir = next_directions.next();
        let adjusted = directions
            .iter()
            .zip(next_directions.chain(first_next_dir))
            .map(|(d1, d2)| adjustment_for(d1, d2))
            .zip(coords)
            .map(|(adjustment, position)| {
                Position(position.0 + adjustment.0, position.1 + adjustment.1)
            })
            .collect::<Vec<Position>>();
        self.get_area(adjusted)
    }
}

#[allow(dead_code)]
fn part_1_instructions(fname: &'static str) -> Vec<Instruction> {
    let input = load_input(fname);
    input
        .trim()
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let direction = parts
                .next()
                .unwrap()
                .parse::<Direction>()
                .expect("Should be a valid direction!");
            let distance = parts
                .next()
                .unwrap()
                .parse::<isize>()
                .expect("Should be a valid size!");
            Instruction::new(direction, distance)
        })
        .collect::<Vec<Instruction>>()
}

#[allow(dead_code)]
fn part_2_instructions(fname: &'static str) -> Vec<Instruction> {
    let input = load_input(fname);
    input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>()
}

#[allow(dead_code)]
fn part1() {
    let instructions = part_1_instructions("part1.txt");
    let mut state = InstructionsState::new();
    let result = state.process_instructions(&instructions);
    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let instructions = part_2_instructions("part1.txt");
    let mut state = InstructionsState::new();
    let result = state.process_instructions(&instructions);
    println!("Part 2 Result: {:?}", result);
}

fn main() {
    part1();
    part2();
}
