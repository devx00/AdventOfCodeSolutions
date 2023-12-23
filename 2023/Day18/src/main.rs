use colored::{ColoredString, Colorize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    ops::{Index, IndexMut},
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
struct Edge {
    direction: Direction,
    size: isize,
    color: Color,
}

impl FromStr for Edge {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        Ok(Self {
            direction: parts[0]
                .parse::<Direction>()
                .expect("Should have been a direction!"),
            size: isize::from_str(parts[1]).expect("Should be a number!"),
            color: parts[2]
                .parse::<Color>()
                .expect("Should have been a color!"),
        })
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    Hole(Color),
    Ground,
}

impl Block {
    fn to_printable(&self) -> ColoredString {
        match self {
            Block::Hole(color) => "#".truecolor(color.red, color.green, color.blue),
            Block::Ground => ".".white(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct PlotDimensions(isize, isize);
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Position(isize, isize);

impl Position {
    fn update_max(&mut self, other: Position) {
        self.0 = self.0.max(other.0);
        self.1 = self.1.max(other.1);
    }
    fn update_min(&mut self, other: Position) {
        self.0 = self.0.min(other.0);
        self.1 = self.1.min(other.1);
    }
    fn step(&mut self, direction: Direction, amount: isize) {
        match direction {
            Direction::Up => self.0 -= amount,
            Direction::Down => self.0 += amount,
            Direction::Left => self.1 -= amount,
            Direction::Right => self.1 += amount,
        }
    }

    fn update(&mut self, row: isize, col: isize) {
        self.0 = row;
        self.1 = col;
    }

    fn as_unsigned(&self) -> (usize, usize) {
        (self.0.unsigned_abs(), self.1.unsigned_abs())
    }
}

impl Default for Position {
    fn default() -> Self {
        Position(0, 0)
    }
}

enum EdgeOrientation {
    Horizontal,
    Vertical,
}
#[derive(Debug, Clone)]
struct PlottedEdge {
    start: Position,
    end: Position,
    direction: Direction,
    is_inflection: bool,
}

impl PlottedEdge {
    fn new(start: Position, end: Position, direction: Direction) -> Self {
        let normalized_start = match start.0 < end.0 || start.1 < end.1 {
            true => start,
            false => end,
        };
        let normalized_end = match normalized_start == start {
            true => end,
            false => start,
        };
        Self {
            start: normalized_start,
            end: normalized_end,
            direction,
            is_inflection: match direction {
                Direction::Up | Direction::Down => true,
                _ => false,
            },
        }
    }
    fn orientation(&self) -> EdgeOrientation {
        match self.direction {
            Direction::Up | Direction::Down => EdgeOrientation::Vertical,
            Direction::Left | Direction::Right => EdgeOrientation::Horizontal,
        }
    }
    fn contains_position(&self, position: &Position) -> bool {
        match self.orientation() {
            EdgeOrientation::Vertical => {
                (self.start.0 + 1..self.end.0 + 1).contains(&position.0)
                    && self.start.1 == position.1
            }
            EdgeOrientation::Horizontal => {
                (self.start.1..self.end.1 + 1).contains(&position.1) && self.start.0 == position.0
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Plot {
    terrain: Vec<Vec<Block>>,
    cursor: Position,
    dimensions: PlotDimensions,
    plotted_edges: Vec<PlottedEdge>,
}
impl Index<Position> for Plot {
    type Output = Block;
    fn index(&self, index: Position) -> &Self::Output {
        let position_unsigned = index.as_unsigned();
        &self.terrain[position_unsigned.0][position_unsigned.1]
    }
}
impl IndexMut<Position> for Plot {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let position_unsigned = index.as_unsigned();
        &mut self.terrain[position_unsigned.0][position_unsigned.1]
    }
}

impl Plot {
    fn bounds_needed(edges: &Vec<Edge>) -> (PlotDimensions, Position) {
        let mut cursor = Position::default();
        let mut max_dimensions = Position::default();
        let mut min_dimensions = Position::default();
        for edge in edges {
            cursor.step(edge.direction, edge.size);
            max_dimensions.update_max(cursor);
            min_dimensions.update_min(cursor);
        }

        let start_position = Position(min_dimensions.0.abs(), min_dimensions.1.abs());
        let dimensions = PlotDimensions(
            max_dimensions.0 + min_dimensions.0.abs(),
            max_dimensions.1 + min_dimensions.1.abs(),
        );

        (dimensions, start_position)
    }
    fn new(edges: Vec<Edge>) -> Self {
        let (dimensions, cursor) = Plot::bounds_needed(&edges);
        let terrain = (0..dimensions.0 + 1)
            .map(|_| {
                (0..dimensions.1 + 1)
                    .map(|_| Block::Ground)
                    .collect::<Vec<Block>>()
            })
            .collect::<Vec<Vec<Block>>>();

        let mut plot = Plot {
            terrain,
            cursor,
            dimensions,
            plotted_edges: vec![],
        };

        plot.draw_edges(edges);

        plot
    }

    fn current_position(&mut self) -> &mut Block {
        self.index_mut(self.cursor.clone())
    }

    fn dig(&mut self, color: Color) {
        *self.current_position() = Block::Hole(color);
    }

    fn is_start_of_row(&self) -> bool {
        self.cursor.1 == 0
    }

    fn is_start_of_column(&self) -> bool {
        self.cursor.0 == 0
    }

    fn is_at_start(&self) -> bool {
        self.is_start_of_row() && self.is_start_of_column()
    }

    fn is_end_of_row(&self) -> bool {
        self.cursor.1 == self.dimensions.1
    }

    fn is_end_of_column(&self) -> bool {
        self.cursor.0 == self.dimensions.0
    }

    fn is_at_end(&self) -> bool {
        self.is_end_of_row() && self.is_end_of_column()
    }

    fn is_hole(&mut self) -> bool {
        match self.current_position() {
            Block::Ground => false,
            Block::Hole(_) => true,
        }
    }

    fn step_forward(&mut self) {
        if self.is_end_of_row() {
            if self.is_end_of_column() {
                return;
            }
            let next_row = self.cursor.0 + 1;
            self.cursor.update(next_row, 0);
            return;
        }

        self.cursor.step(Direction::Right, 1);
    }

    fn step_backward(&mut self) {
        if self.is_start_of_row() {
            if self.is_start_of_column() {
                return;
            }
            let next_row = self.cursor.0 - 1;
            self.cursor.update(next_row, self.dimensions.1);
            return;
        }

        self.cursor.step(Direction::Left, 1);
    }

    fn is_on_inflection_edge(&self, position: &Position) -> bool {
        self.plotted_edges
            .iter()
            .find(|edge| edge.contains_position(position) && edge.is_inflection)
            // .find(|edge| match edge.orientation() {
            //     EdgeOrientation::Vertical => edge.contains_position(position),
            //     EdgeOrientation::Horizontal => edge.containsedge.is_inflection,
            // })
            .is_some()
    }

    fn is_on_edge(&self, position: &Position) -> bool {
        self.plotted_edges
            .iter()
            .find(|edge| edge.contains_position(position))
            .is_some()
    }

    fn is_currently_on_edge(&self) -> bool {
        self.is_on_edge(&self.cursor)
    }

    fn should_dig(&self) -> bool {
        (self.cursor.1..self.dimensions.1 + 1)
            .map(|col| Position(self.cursor.0, col))
            .fold(vec![false], |mut acc, pos| {
                let is_edge = self.is_on_inflection_edge(&pos);

                if *acc.iter().last().unwrap() != is_edge {
                    acc.push(is_edge);
                }
                acc
            })
            .into_iter()
            .filter(|v| *v)
            .count()
            % 2
            == 1
    }

    fn excavate(&mut self) {
        self.cursor = Position(0, 0);
        let mut should_dig = false;
        let mut was_on_edge = false;

        while !self.is_at_end() {
            if self.is_hole() {
                was_on_edge = true;
                should_dig = false;
            }

            if was_on_edge && !self.is_hole() {
                should_dig = self.should_dig();
                was_on_edge = false;
            }

            if should_dig {
                self.dig(Color::from_str("#ffffffff").unwrap());
            }

            if self.is_end_of_row() {
                should_dig = false;
                was_on_edge = false;
            }
            self.step_forward();
        }
    }

    fn draw_edges(&mut self, edges: Vec<Edge>) {
        let mut last_vert: Option<PlottedEdge> = None;
        let mut last_orientation = EdgeOrientation::Horizontal;
        for edge in edges {
            let edge_start = self.cursor.clone();
            for _ in 0..edge.size {
                self.dig(edge.color);
                self.cursor.step(edge.direction, 1);
            }
            let edge_end = self.cursor.clone();
            let plotted_edge = PlottedEdge::new(edge_end, edge_start, edge.direction);
            match plotted_edge.orientation() {
                EdgeOrientation::Vertical => {
                    last_orientation = EdgeOrientation::Vertical;
                    match self.plotted_edges.iter_mut().last() {
                        Some(last_horiz) => match last_vert {
                            Some(prev_vert) => {
                                last_horiz.is_inflection =
                                    prev_vert.direction == plotted_edge.direction
                            }
                            None => (),
                        },
                        _ => (),
                    }
                    last_vert = Some(plotted_edge.clone());
                }
                EdgeOrientation::Horizontal => last_orientation = EdgeOrientation::Horizontal,
            }
            self.plotted_edges.push(plotted_edge);
        }

        let first_vert = self
            .plotted_edges
            .iter()
            .find(|v| match v.orientation() {
                EdgeOrientation::Vertical => true,
                _ => false,
            })
            .unwrap()
            .clone();
        let remaining = match last_orientation {
            EdgeOrientation::Vertical => self.plotted_edges.iter_mut().nth(0),
            EdgeOrientation::Horizontal => self.plotted_edges.iter_mut().last(),
        };
        match remaining {
            Some(last) => match last.orientation() {
                EdgeOrientation::Horizontal => {
                    last.is_inflection = first_vert.direction == last_vert.unwrap().direction
                }
                _ => panic!("This shouldve been horizontal..."),
            },
            _ => (),
        }
    }

    fn display(&self) {
        self.terrain.iter().for_each(|row| {
            row.iter().for_each(|block| {
                print!("{}", block.to_printable());
            });
            print!("\n");
        })
    }

    fn count_holes(&self) -> usize {
        self.terrain.iter().fold(0, |acc, row| {
            acc + row
                .into_iter()
                .filter(|block| **block != Block::Ground)
                .count()
        })
    }
}

impl FromStr for Plot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s
            .trim()
            .lines()
            .map(|l| l.parse::<Edge>().expect("Shoulda been an edge!"))
            .collect::<Vec<Edge>>();
        Ok(Self::new(edges))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Instruction(Direction, isize);

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

        Ok(Instruction(dir, distance))
    }
}

struct InstructionsState {
    cursor: Position,
    vertical_stack: Vec<(Position, Position)>,
    positive_direction: Direction,
    filled_cubes: isize,
    prev_vert: Direction,
    prev_horiz: Direction,
    saw_vert_inflection: bool,
    saw_horiz_inflection: bool,
    vert_inflections_left: usize,
    horiz_inflections_left: usize,
}

impl InstructionsState {
    fn new() -> InstructionsState {
        InstructionsState {
            cursor: Position::default(),
            vertical_stack: vec![],
            positive_direction: Direction::Down,
            filled_cubes: 0,
            prev_vert: Direction::Down,
            prev_horiz: Direction::Right,
            saw_vert_inflection: false,
            saw_horiz_inflection: false,
            vert_inflections_left: 1,
            horiz_inflections_left: 1,
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        let Instruction(direction, distance) = instruction;
        println!("\n\nProcessing: {:?} {}", direction, distance);
        match direction {
            Direction::Up | Direction::Down => {
                if self.prev_vert != direction {
                    self.saw_vert_inflection = true;
                    self.prev_vert = direction;
                }
            }
            Direction::Left | Direction::Right => {
                if self.prev_horiz != direction {
                    self.saw_horiz_inflection = true;
                    self.prev_horiz = direction;
                }
            }
        }

        match direction {
            Direction::Left | Direction::Right => {
                // self.filled_cubes += distance;
                self.cursor.step(direction, distance);
            }
            direction if direction == self.positive_direction => {
                println!(
                    "Matched Positive Direction: instruction: {:?}, positive direction: {:?}",
                    direction, self.positive_direction
                );
                let start = self.cursor.clone();
                self.cursor.step(direction, distance);
                let end = self.cursor.clone();

                if self.saw_vert_inflection && self.vert_inflections_left > 0 {
                    self.filled_cubes += distance;
                    self.vert_inflections_left -= 1;
                    self.saw_vert_inflection = false;
                }

                self.vertical_stack.push((start, end));
            }
            direction if direction != self.positive_direction => {
                println!(
                    "Matched Non-Positive Direction: instruction: {:?}, positive direction: {:?}",
                    direction, self.positive_direction
                );
                let mut vert_distance_left = distance;
                while let Some((start, end)) = self.vertical_stack.pop() {
                    if vert_distance_left == 0 {
                        break;
                    }
                    let width: isize = start.1.abs_diff(self.cursor.1).try_into().unwrap();
                    let available_vertical = start.0.abs_diff(end.0).try_into().unwrap();
                    let start_fixed = match end.0 == self.cursor.0 {
                        true => start.clone(),
                        false => end.clone(),
                    };

                    let will_rollover = available_vertical > vert_distance_left;

                    let vert_distance = vert_distance_left.min(available_vertical);
                    self.filled_cubes += vert_distance * width;
                    vert_distance_left -= vert_distance;

                    self.cursor.step(direction, vert_distance);

                    let end_fixed = Position(self.cursor.0, start_fixed.1);

                    if will_rollover && start_fixed.0 != end_fixed.0 {
                        self.vertical_stack.push((start_fixed, end_fixed));

                        break;
                    }
                }

                if vert_distance_left != 0 {
                    assert_eq!(
                        self.vertical_stack.len(),
                        0,
                        "Vertical stack should always be empty if we reach here."
                    );

                    println!("Should be switching positive direction now!");

                    self.positive_direction = match self.positive_direction {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                        _ => panic!("Should only be up or down."),
                    };

                    self.process_instruction(Instruction(direction, vert_distance_left));
                }
            }
            _ => panic!("Shouldnt reach here!"),
        }

        println!("-- End Processing --\n\n");
    }
    fn process_instructions(&mut self, instructions: Vec<Instruction>) -> isize {
        let first_vert = instructions
            .iter()
            .map(|i| i.0)
            .find(|d| match d {
                Direction::Up | Direction::Down => true,
                Direction::Right | Direction::Left => false,
            })
            .unwrap();
        let first_horiz = instructions
            .iter()
            .map(|i| i.0)
            .find(|d| match d {
                Direction::Up | Direction::Down => false,
                Direction::Right | Direction::Left => true,
            })
            .unwrap();

        self.positive_direction = first_vert;

        self.prev_vert = first_vert;
        self.prev_horiz = first_horiz;
        for instruction in instructions {
            self.print();
            self.process_instruction(instruction);
            self.print();
        }

        self.filled_cubes
    }
    fn print(&self) {
        println!("Cursor: {:?}", self.cursor);
        println!("Filled Cubes: {}", self.filled_cubes);
        println!("Positive Direction: {:?}", self.positive_direction);
        println!("========== Stack ==========");
        self.vertical_stack.iter().for_each(|(p1, p2)| {
            println!("\t{:?} =({})> {:?}", p1, p1.0.abs_diff(p2.0), p2,);
        });
        println!("===========================");
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let instructions = input
        .trim()
        .lines()
        .map(|l| l.parse::<Edge>().expect("Shoulda been an edge!"))
        .map(|e| Instruction(e.direction, e.size))
        .collect::<Vec<Instruction>>();
    let mut state = InstructionsState::new();
    state.process_instructions(instructions);
    let result = state.filled_cubes;
    // let mut plot = input.parse::<Plot>().unwrap();
    // println!("Edges: {:?}", plot.plotted_edges);
    // plot.display();
    // plot.excavate();
    // plot.display();
    // let result = plot.count_holes();
    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("example1.txt");
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();
    let counts = instructions.iter().fold(HashMap::new(), |mut acc, inst| {
        let Instruction(direction, distance) = inst;
        acc.entry(direction).or_insert_with(isize::default);
        acc.entry(direction).and_modify(|e| *e += distance);
        acc
    });
    println!("Counts: {:?}", counts);
    let mut state = InstructionsState::new();
    state.process_instructions(instructions);
    let result = state.filled_cubes;
    println!("Part 2 Result: {:?}", result);
}

fn main() {
    part1();
    // part2();
}
