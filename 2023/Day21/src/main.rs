use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufReader, Read},
    ops::Index,
    str::FromStr,
};

use itertools::Itertools;

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Position(usize, usize);

impl Position {
    fn adjacent(&self, bounds: Position) -> Vec<Position> {
        let deltas: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
        deltas
            .iter()
            .filter_map(|(dr, dc)| match self.0.checked_add_signed(*dr) {
                Some(new_row) => match self.1.checked_add_signed(*dc) {
                    Some(new_column) => {
                        if new_row < bounds.0 && new_column < bounds.1 {
                            Some(Position(new_row, new_column))
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<Position>>()
    }

    fn to_signed(&self) -> SignedPosition {
        SignedPosition(self.0.try_into().unwrap(), self.1.try_into().unwrap())
    }
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct SignedPosition(isize, isize);

impl SignedPosition {
    fn adjacent(&self) -> Vec<SignedPosition> {
        let deltas: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
        deltas
            .iter()
            .map(|(dr, dc)| SignedPosition(self.0 + dr, self.1 + dc))
            .collect::<Vec<SignedPosition>>()
    }

    fn normalized_position(&self, bounds: Position) -> Position {
        let unsigned_row = match self.0 {
            neg_row if neg_row < 0 => (bounds.0) - (((neg_row.abs_diff(0) - 1) % (bounds.0)) + 1),
            pos_row => pos_row.abs_diff(0) % bounds.0,
        };
        let unsigned_col = match self.1 {
            neg_col if neg_col < 0 => (bounds.1) - (((neg_col.abs_diff(0) - 1) % (bounds.1)) + 1),
            pos_col => pos_col.abs_diff(0) % bounds.1,
        };
        // println!(
        //     "R: {} => {}, C: {} => {} | {:?}",
        //     self.0, unsigned_row, self.1, unsigned_col, bounds
        // );

        Position(unsigned_row, unsigned_col)
    }
}

#[derive(Debug)]
enum Entity {
    StartingPosition,
    GardenPlot,
    Rock,
}

impl From<char> for Entity {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::GardenPlot,
            '#' => Self::Rock,
            'S' => Self::StartingPosition,
            _ => panic!("Received invalid entity character!"),
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Entity>>,
}

impl Map {
    fn bounds(&self) -> Position {
        Position(self.grid.len(), self.grid[0].len())
    }

    fn starting_position(&self) -> Position {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter().enumerate().find_map(|(j, e)| match e {
                    Entity::StartingPosition => Some(Position(i, j)),
                    _ => None,
                })
            })
            .unwrap()
    }

    fn valid_steps_signed(&self, from_position: SignedPosition) -> Vec<SignedPosition> {
        from_position
            .adjacent()
            .into_iter()
            .filter_map(|p| match self[p] {
                Entity::StartingPosition | Entity::GardenPlot => Some(p),
                _ => None,
            })
            .collect::<Vec<SignedPosition>>()
    }

    fn valid_steps(&self, from_position: Position) -> Vec<Position> {
        from_position
            .adjacent(self.bounds())
            .into_iter()
            .filter_map(|p| match self[p] {
                Entity::StartingPosition | Entity::GardenPlot => Some(p),
                _ => None,
            })
            .collect::<Vec<Position>>()
    }

    fn count_max_positions_signed(&self, steps_allowed: isize) -> usize {
        let mut seen: HashMap<SignedPosition, isize> = HashMap::new();
        let mut position_queue: VecDeque<(SignedPosition, isize)> = VecDeque::new();

        let matching_remainder = steps_allowed % 2;
        position_queue.push_back((self.starting_position().to_signed(), 0));

        while let Some((position, step_count)) = position_queue.pop_front() {
            if seen.contains_key(&position) {
                continue;
            }

            seen.insert(position, step_count);
            // println!("({}, {}), {}", position.0, position.1, step_count);

            if step_count < steps_allowed {
                position_queue.extend(
                    self.valid_steps_signed(position)
                        .into_iter()
                        .filter(|p| !seen.contains_key(p))
                        .map(|next_p| (next_p, step_count + 1)),
                );
            }
        }

        let step_counts = seen.values().counts();
        let sorted_counts = step_counts.iter().sorted();
        // println!("Step Counts: {:?}", sorted_counts);

        seen.iter()
            .filter(|(_, v)| *v % 2 == matching_remainder)
            .count()
    }

    fn count_max_positions(&self, steps_allowed: usize) -> usize {
        let mut seen: HashMap<Position, usize> = HashMap::new();
        let mut position_queue: VecDeque<(Position, usize)> = VecDeque::new();

        let matching_remainder = steps_allowed % 2;
        position_queue.push_back((self.starting_position(), 0));

        while let Some((position, step_count)) = position_queue.pop_front() {
            if seen.contains_key(&position) {
                continue;
            }

            seen.insert(position, step_count);

            if step_count < steps_allowed {
                position_queue.extend(
                    self.valid_steps(position)
                        .into_iter()
                        .filter(|p| !seen.contains_key(p))
                        .map(|next_p| (next_p, step_count + 1)),
                );
            }
        }

        seen.iter()
            .filter(|(_, v)| *v % 2 == matching_remainder)
            .count()
    }
    fn check_position(&self, p: Position) {
        let val = &self[p];
        println!("Index: {:?} => {:?}", p, val);
    }
}

impl Index<Position> for Map {
    type Output = Entity;
    fn index(&self, index: Position) -> &Self::Output {
        // println!("Finding Position: {:?}", index);
        let l_bounds = self.bounds();
        &self.grid[index.0 % l_bounds.0][index.1 % l_bounds.1]
    }
}
impl Index<SignedPosition> for Map {
    type Output = Entity;
    fn index(&self, index: SignedPosition) -> &Self::Output {
        // println!("Finding Position: {:?}", index);
        let l_bounds = self.bounds();
        let n_index = index.normalized_position(l_bounds);
        &self.grid[n_index.0][n_index.1]
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            grid: s
                .trim()
                .lines()
                .map(|l| l.chars().map(|c| Entity::from(c)).collect::<Vec<Entity>>())
                .collect::<Vec<Vec<Entity>>>(),
        })
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let map = input
        .parse::<Map>()
        .expect("Map should have been parsed successfully!");
    let result = map.count_max_positions(64);

    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("part1.txt");
    let mut map = input
        .parse::<Map>()
        .expect("Map should have been parsed successfully!");
    // println!("Bounds: {:?}", map.bounds());
    // let test_inputs: Vec<(isize, usize)> = vec![
    //     (6, 16),
    //     (10, 50),
    //     (50, 1594),
    //     (100, 6536),
    //     (500, 167004),
    //     // (1000, 668697),
    //     // (5000, 16733044),
    // ];
    let test_inputs = (0..500);
    test_inputs
        // .into_iter()
        .for_each(|input| {
            let answer = map.count_max_positions_signed(input);
            println!("{}, {}", input, answer);
            // println!(
            //     "Got Wrong Answer for: {}. Expected {}, Got: {} | {}",
            //     input,
            //     expected_answer,
            //     answer,
            //     expected_answer == answer
            // );
        });
    // let result = map.count_max_positions(26501365);
    // map.check_position(Position(11, 11));
    // map.check_position(Position(11, 11));
    // map.check_position(Position(11, 11));
    // println!("Part 2 Result: {}", result);
}

fn main() {
    // part1();
    part2();
}
