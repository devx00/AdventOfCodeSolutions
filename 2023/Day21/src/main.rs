use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufReader, Read},
    ops::Index,
    str::FromStr,
};

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
    bounds_override: Option<Position>,
}

impl Map {
    fn literal_bounds(&self) -> Position {
        Position(self.grid.len(), self.grid[0].len())
    }
    fn bounds(&self) -> Position {
        if let Some(override_bounds) = self.bounds_override {
            return override_bounds;
        }
        self.literal_bounds()
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
        let l_bounds = self.literal_bounds();
        &self.grid[index.0 % l_bounds.0][index.1 % l_bounds.1]
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
            bounds_override: None,
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
    let input = load_input("example1.txt");
    let mut map = input
        .parse::<Map>()
        .expect("Map should have been parsed successfully!");
    map.bounds_override = Some(Position(usize::MAX, usize::MAX));
    // let test_inputs: Vec<(usize, usize)> = vec![
    //     (6, 16),
    //     (10, 50),
    //     (50, 1594),
    //     (180, 6536),
    //     (500, 167004),
    //     (1000, 668697),
    //     (5000, 16733044),
    // ];
    // test_inputs
    //     .into_iter()
    //     .for_each(|(input, expected_answer)| {
    //         let answer = map.count_max_positions(input);
    //         if answer != expected_answer {
    //             println!(
    //                 "Got Wrong Answer for: {}. Expected {}, Got: {}",
    //                 input, expected_answer, answer
    //             );
    //         }
    //     });
    // let result = map.count_max_positions(26501365);
    // map.check_position(Position(11, 11));
    // map.check_position(Position(11, 11));
    // map.check_position(Position(11, 11));
    // println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    // part2();
}
