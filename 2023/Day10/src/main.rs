use std::convert::TryInto;
use std::ops::Add;
use std::result::Result;
use std::result::Result::Err;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn complement(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn delta(&self) -> Coordinates {
        Coordinates {
            x: match self {
                Self::North | Self::South => 0,
                Self::East => 1,
                Self::West => -1,
            },
            y: match self {
                Self::North => -1,
                Self::South => 1,
                Self::East | Self::West => 0,
            },
        }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialOrd)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn contains(&self, other: Coordinates) -> bool {
        self.x >= other.x && self.y >= other.y
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    WithDirections(Direction, Direction),
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::WithDirections(Direction::North, Direction::South),
            '-' => Pipe::WithDirections(Direction::East, Direction::West),
            'L' => Pipe::WithDirections(Direction::North, Direction::East),
            'J' => Pipe::WithDirections(Direction::North, Direction::West),
            '7' => Pipe::WithDirections(Direction::South, Direction::West),
            'F' => Pipe::WithDirections(Direction::South, Direction::East),
            _ => panic!("Received unknown pipe character."),
        }
    }
}

impl Pipe {
    fn connects(&self, direction: Direction) -> bool {
        match self {
            Pipe::WithDirections(dir1, dir2) => *dir1 == direction || *dir2 == direction,
        }
    }

    fn char_representation(&self) -> char {
        match self {
            Pipe::WithDirections(Direction::North, Direction::South) => '│',
            Pipe::WithDirections(Direction::East, Direction::West) => '─',
            Pipe::WithDirections(Direction::North, Direction::East) => '╰',
            Pipe::WithDirections(Direction::North, Direction::West) => '╯',
            Pipe::WithDirections(Direction::South, Direction::West) => '╮',
            Pipe::WithDirections(Direction::South, Direction::East) => '╭',
            _ => 'z',
        }
    }
}

#[derive(Debug, Clone)]
enum Entity {
    Start,
    Pipe(Pipe),
    Blank,
}

impl From<char> for Entity {
    fn from(value: char) -> Self {
        match value {
            'S' => Entity::Start,
            '.' => Entity::Blank,
            _ => Entity::Pipe(Pipe::from(value)),
        }
    }
}

impl Entity {
    fn char_representation(&self, is_mloop: bool, is_enclosed: bool) -> char {
        match self {
            Self::Start => '★',
            Self::Pipe(p) => {
                if is_mloop {
                    p.char_representation()
                } else {
                    if is_enclosed {
                        '■'
                    } else {
                        '□'
                    }
                }
            }
            Self::Blank => {
                if is_enclosed {
                    '■'
                } else {
                    '□'
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    entity: Entity,
    coordinates: Coordinates,
}

impl Tile {
    fn create(c: char, coordinates: Coordinates) -> Tile {
        Tile {
            entity: Entity::from(c),
            coordinates,
        }
    }

    fn neighbor(&self, direction: Direction) -> Coordinates {
        self.coordinates + direction.delta()
    }

    fn connects(&self, direction: Direction) -> bool {
        match self.entity {
            Entity::Pipe(p) => p.connects(direction),
            _ => false,
        }
    }

    fn char_representation(&self, is_mloop: bool, is_enclosed: bool) -> char {
        self.entity.char_representation(is_mloop, is_enclosed)
    }
}
#[derive(Debug, Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    current_coordinate: Option<Coordinates>,
    last_move: Option<Direction>,
    main_loop: Option<Vec<Coordinates>>,
}

impl FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            tiles: s
                .split("\n")
                .enumerate()
                .map(|(y, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(x, cs)| {
                            Tile::create(
                                cs,
                                Coordinates {
                                    x: x.try_into().unwrap(),
                                    y: y.try_into().unwrap(),
                                },
                            )
                        })
                        .collect::<Vec<Tile>>()
                })
                .collect::<Vec<Vec<Tile>>>(),
            current_coordinate: None,
            last_move: None,
            main_loop: None,
        })
    }
}

impl Board {
    fn restart(&mut self) -> Result<(), &'static str> {
        self.current_coordinate = match self.tiles.iter().flatten().find(|t| match t.entity {
            Entity::Start => true,
            _ => false,
        }) {
            Some(tile) => Some(tile.coordinates.clone()),
            _ => None,
        };

        self.last_move = None;

        Ok(())
    }

    fn bounds(&self) -> Coordinates {
        Coordinates {
            x: self
                .tiles
                .get(0)
                .expect("Expected at least 1 row in board")
                .len()
                .try_into()
                .unwrap(),
            y: self.tiles.len().try_into().unwrap(),
        }
    }

    fn get_tile(&self, coordinates: Coordinates) -> Option<&Tile> {
        match self.tiles.get::<usize>(coordinates.y.try_into().unwrap()) {
            Some(r) => r.get::<usize>(coordinates.x.try_into().unwrap()),
            _ => None,
        }
    }

    fn current_tile(&self) -> Option<&Tile> {
        match self.current_coordinate {
            Some(coords) => self.get_tile(coords),
            _ => None,
        }
    }

    fn available_moves(&self) -> Vec<Direction> {
        match self.current_tile() {
            Some(tile) => match tile.entity {
                Entity::Start => vec![
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .clone()
                .iter()
                .filter(|d| match self.get_tile(tile.neighbor(**d)) {
                    Some(neighbor) => neighbor.connects(d.complement()),
                    _ => false,
                })
                .map(|dd| *dd)
                .collect(),
                Entity::Pipe(Pipe::WithDirections(d1, d2)) => match self.last_move {
                    Some(dir) => vec![d1, d2]
                        .iter()
                        .filter(|d| **d != dir.complement())
                        .map(|d| *d)
                        .collect(),
                    _ => vec![],
                },
                _ => vec![],
            },
            _ => vec![],
        }
    }

    fn move_direction(&mut self, direction: Direction) -> Result<(), &'static str> {
        let next_tile_coords = match self.current_tile() {
            Some(tile) => tile.neighbor(direction),
            _ => panic!("No current tile."),
        };

        if !self.bounds().contains(next_tile_coords) {
            return Err("Received a bad direction to move.");
        }

        match self.get_tile(next_tile_coords) {
            Some(tile) => {
                self.current_coordinate = Some(tile.coordinates);
                self.last_move = Some(direction);
                Ok(())
            }
            _ => Err("Failed to move to new tile."),
        }
    }

    fn move_any(&mut self) -> Result<(), &'static str> {
        match self.available_moves().first() {
            Some(dir) => self.move_direction(*dir),
            _ => Err("Failed to move."),
        }
    }

    fn is_at_start(&self) -> bool {
        match self.current_tile() {
            Some(t) => match t.entity {
                Entity::Start => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn main_loop(&mut self) -> Vec<Coordinates> {
        if self.main_loop.is_some() {
            return self.main_loop.clone().unwrap();
        }

        let saved_coordinate = self.current_coordinate;
        let saved_move = self.last_move;

        match self.restart() {
            Ok(()) => (),
            Err(_) => panic!("Failed to restart"),
        }

        let mut loop_coords: Vec<Coordinates> = Vec::new();

        match self.move_any() {
            Err(_) => panic!("Failed to move before while loop in loop_len"),
            _ => loop_coords.push(self.current_coordinate.unwrap()),
        }

        while !self.is_at_start() {
            match self.move_any() {
                Err(_) => panic!("Failed to move in loop_len while loop"),
                _ => loop_coords.push(self.current_coordinate.unwrap()),
            }
        }

        self.current_coordinate = saved_coordinate;
        self.last_move = saved_move;

        loop_coords
    }

    fn loop_len(&mut self) -> usize {
        self.main_loop().len()
    }

    fn is_horizontal(&self, coordinate: Coordinates) -> bool {
        match self.get_tile(coordinate) {
            Some(t) => match t.entity {
                Entity::Pipe(Pipe::WithDirections(d1, d2)) => {
                    vec![d1, d2]
                        .iter()
                        .filter(|d| !vec![Direction::East, Direction::West].contains(d))
                        .count()
                        == 0
                }
                Entity::Start => match self.pipe_for_start() {
                    Pipe::WithDirections(d1, d2) => {
                        vec![d1, d2]
                            .iter()
                            .filter(|d| !vec![Direction::East, Direction::West].contains(d))
                            .count()
                            == 0
                    }
                },
                _ => false,
            },
            _ => false,
        }
    }
    fn is_vertical(&self, coordinate: Coordinates) -> bool {
        match self.get_tile(coordinate) {
            Some(t) => match t.entity {
                Entity::Pipe(Pipe::WithDirections(d1, d2)) => {
                    // d1 == Direction::North || d2 == Direction::North
                    vec![d1, d2]
                        .iter()
                        .filter(|d| !vec![Direction::North, Direction::South].contains(d))
                        .count()
                        == 0
                }
                Entity::Start => match self.pipe_for_start() {
                    Pipe::WithDirections(d1, d2) => {
                        vec![d1, d2]
                            .iter()
                            .filter(|d| !vec![Direction::North, Direction::South].contains(d))
                            .count()
                            == 0
                    }
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn str_rep(&self, mloop: &Vec<Coordinates>, enclosed: &Vec<Coordinates>) -> String {
        self.tiles
            .iter()
            .map(|l| {
                l.iter()
                    .map(|t| {
                        t.char_representation(
                            mloop.contains(&t.coordinates),
                            enclosed.contains(&t.coordinates),
                        )
                        .to_string()
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn pipe_for_start(&self) -> Pipe {
        match self.tiles.iter().flatten().find(|t| match t.entity {
            Entity::Start => true,
            _ => false,
        }) {
            Some(tile) => {
                let dirs: Vec<Direction> = vec![
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .clone()
                .iter()
                .filter(|d| match self.get_tile(tile.neighbor(**d)) {
                    Some(neighbor) => neighbor.connects(d.complement()),
                    _ => false,
                })
                .map(|dd| *dd)
                .collect();
                Pipe::WithDirections(*dirs.first().unwrap(), *dirs.iter().nth(1).unwrap())
            }
            _ => panic!("Should be able to find start."),
        }
    }
}

fn load_board() -> Board {
    include_str!("../inputs/part1.txt")
        .parse::<Board>()
        .expect("Expected to parse a board")
}

fn part1() {
    let mut board = load_board();
    let len = board.loop_len();
    let answer = len / 2;
    println!("Len: {}", len);
    println!("Part 1 Answer: {}", answer);
}

fn print_rep(board: &Board, mloop: &Vec<Coordinates>, enclosed: &Vec<Coordinates>) {
    let rep = board.str_rep(mloop, enclosed);
    println!("{}", rep);
}

fn part2() {
    let mut board = load_board();
    let mut mloop = board.main_loop();
    mloop.sort();

    let enclosed = board
        .tiles
        .iter()
        .flatten()
        .map(|t| t.coordinates)
        .filter(|c| !mloop.contains(c))
        .filter(|coords| {
            let mut partial: Option<Direction> = None;
            mloop
                .iter()
                .filter(|c| c.x > coords.x && c.y == coords.y)
                .map(|c| {
                    if board.is_vertical(*c) {
                        1
                    } else {
                        if !board.is_horizontal(*c) {
                            let part = match board.get_tile(*c).unwrap().entity {
                                Entity::Pipe(Pipe::WithDirections(dir1, _)) => dir1,
                                Entity::Start => match board.pipe_for_start() {
                                    Pipe::WithDirections(dir1, _) => dir1,
                                },
                                _ => panic!("This shouldnt happen"),
                            };
                            match partial {
                                Some(d) => {
                                    let delta = if part == d { 0 } else { 1 };
                                    partial = None;
                                    delta
                                }
                                _ => {
                                    partial = Some(part);
                                    0
                                }
                            }
                        } else {
                            0
                        }
                    }
                })
                .sum::<i32>()
                % 2
                != 0
        })
        .collect::<Vec<Coordinates>>();

    print_rep(&board, &mloop, &enclosed);
    // println!("Board: {}", board);
    let nenclosed = enclosed.len();

    println!("Num Enclosed: {}", nenclosed);
}
fn main() {
    part1();
    part2();
}
