use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufReader, Read},
    ops::Index,
    str::FromStr,
    usize,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GridIndex(usize, usize);

impl GridIndex {
    fn offset(&self, rows: isize, cols: isize) -> Option<Self> {
        match (
            self.0.checked_add_signed(rows),
            self.1.checked_add_signed(cols),
        ) {
            (Some(new_rows), Some(new_cols)) => Some(Self(new_rows, new_cols)),
            _ => None,
        }
    }
    fn neighbor(&self, direction: Direction) -> Option<GridIndex> {
        match direction {
            Direction::North => self.offset(-1, 0),
            Direction::South => self.offset(1, 0),
            Direction::East => self.offset(0, 1),
            Direction::West => self.offset(0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Node(GridIndex, usize);

// When comparing 2 nodes directly just compare their vals as normal.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.1.cmp(&other.1))
    }
}

// When comparing 2 nodex for a total ordering, reverse order to prioritize Nodes with lower vals.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl Node {
    fn new(row: usize, column: usize, val: usize) -> Self {
        Self(GridIndex(row, column), val)
    }

    fn index(&self) -> GridIndex {
        self.0.clone()
    }
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct DirectionCount(Direction, u8);

impl DirectionCount {
    fn next_steps(&self) -> Vec<DirectionCount> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .filter(|dir| *dir != self.0.opposite())
        .filter_map(|dir| {
            if self.0 != dir {
                Some(DirectionCount(dir, 1))
            } else {
                if self.1 >= 3 {
                    None
                } else {
                    Some(DirectionCount(dir, self.1 + 1))
                }
            }
        })
        .collect::<Vec<DirectionCount>>()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StepState(GridIndex, DirectionCount);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Step {
    state: StepState,
    path: Vec<GridIndex>,
    total_weight: usize,
}

#[derive(Debug, Clone)]
struct Grid {
    size: GridIndex,
    items: Vec<Vec<Node>>,
}

impl Grid {
    fn contains(&self, GridIndex(row, column): &GridIndex) -> bool {
        *row < self.size.0 && *column < self.size.1
    }

    fn get(&self, index: GridIndex) -> Option<&Node> {
        match self.contains(&index) {
            true => Some(&self[index]),
            false => None,
        }
    }

    fn naive_min_path(&self, from: &Node, to: &Node) -> usize {
        let mut step_queue: VecDeque<Step> = VecDeque::new();
        step_queue.push_back(Step {
            state: StepState(from.index(), DirectionCount(Direction::North, 0)),
            path: vec![],
            total_weight: 0,
        });

        let mut seen: HashMap<GridIndex, usize> = HashMap::new();

        let mut min_val = usize::MAX;
        let mut min_path: Vec<GridIndex> = vec![];

        while let Some(Step {
            state,
            path,
            total_weight,
        }) = step_queue.pop_front()
        {
            let current_index = state.0;
            let mut next_path = path.clone();
            next_path.push(current_index);

            if current_index == to.0 {
                if total_weight < min_val {
                    min_val = total_weight;
                    min_path = next_path.clone();
                }
                continue;
            }

            seen.insert(state.0, total_weight);

            let next_steps = state.1.next_steps();
            step_queue.extend(
                next_steps
                    .iter()
                    .filter_map(|dc| match state.0.neighbor(dc.0) {
                        Some(ind) => match self.get(ind) {
                            Some(Node(grid_index, weight)) => {
                                let new_state = StepState(*grid_index, *dc);
                                let next_step = Step {
                                    state: new_state,
                                    path: next_path.clone(),
                                    total_weight: total_weight + weight,
                                };
                                match seen.get(grid_index) {
                                    Some(found_weight) => {
                                        if next_step.total_weight < *found_weight {
                                            Some(next_step)
                                        } else {
                                            None
                                        }
                                    }
                                    None => Some(next_step),
                                }
                            }
                            None => None,
                        },
                        None => None,
                    }),
            );
        }

        println!("New Min Path: {:?}", min_path);
        min_val
    }
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s
            .trim()
            .split("\n")
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| {
                        Node::new(
                            i,
                            j,
                            c.to_digit(10)
                                .expect("Expected char to be digit")
                                .try_into()
                                .unwrap(),
                        )
                    })
                    .collect::<Vec<Node>>()
            })
            .collect::<Vec<Vec<Node>>>();
        Ok(Self {
            size: GridIndex(items.len(), items[0].len()),
            items,
        })
    }
}

impl Index<GridIndex> for Grid {
    type Output = Node;
    fn index(&self, GridIndex(row, column): GridIndex) -> &Self::Output {
        &self.items[row][column]
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let grid = input.parse::<Grid>().unwrap();
    let start = grid
        .get(GridIndex(0, 0))
        .expect("Should always have a first element.");
    let end = grid
        .get(grid.size.offset(-1, -1).expect("Should be possible"))
        .expect("Should be able to find this node");
    let result = grid.naive_min_path(start, end);

    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("example1.txt");
    let result = input;
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    // part2();
}
