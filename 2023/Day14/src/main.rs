use itertools::Itertools;
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, Ord, PartialEq, PartialOrd, Eq)]
enum Rock {
    Round,
    Square,
    None,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            'O' => Rock::Round,
            '#' => Rock::Square,
            _ => Rock::None,
        }
    }
}

impl ToString for Rock {
    fn to_string(&self) -> String {
        match self {
            Rock::Round => "O".to_string(),
            Rock::Square => "#".to_string(),
            Rock::None => ".".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Platform {
    items: Vec<Vec<RefCell<Rock>>>,
    cycle_tracker: Option<CycleTracker>,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct PlatformState {
    item_runs: Vec<(usize, Rock)>,
}
#[derive(Debug, Clone)]
struct CycleTracker {
    num_cycles: usize,
    seen_states: HashMap<PlatformState, usize>,
    before_period: usize,
}

impl CycleTracker {
    fn new() -> Self {
        CycleTracker {
            num_cycles: 0,
            seen_states: HashMap::new(),
            before_period: 0,
        }
    }
}

impl Platform {
    fn num_columns(&self) -> usize {
        self.items[0].len()
    }
    fn num_rows(&self) -> usize {
        self.items.len()
    }

    fn sort_direction(&self, cells: Vec<&RefCell<Rock>>, direction: CardinalDirection) {
        cells
            .iter()
            .map(|c| *c.borrow())
            .group_by(|r| *r != Rock::Square)
            .into_iter()
            .map(|(_, vals)| {
                vals.sorted_by(|a, b| match direction {
                    CardinalDirection::North | CardinalDirection::West => a.cmp(b),
                    CardinalDirection::South | CardinalDirection::East => b.cmp(a),
                })
            })
            .flatten()
            .zip(cells.iter())
            .for_each(|(r, rc)| *rc.borrow_mut() = r)
    }

    fn tilt(&self, direction: CardinalDirection) {
        match direction {
            CardinalDirection::North | CardinalDirection::South => {
                (0..self.num_columns()).for_each(|i| {
                    self.sort_direction(
                        self.items.iter().map(|r| &r[i]).into_iter().collect(),
                        direction.clone(),
                    )
                });
            }
            CardinalDirection::East | CardinalDirection::West => {
                (0..self.num_rows()).for_each(|i| {
                    self.sort_direction(self.items[i].iter().collect(), direction.clone())
                });
            }
        }
    }
    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "{}",
            (0..self.items.first().unwrap().len()).map(|_| "-").join("")
        );
        self.items.iter().for_each(|r| {
            println!(
                "{}",
                r.iter()
                    .map(|i| (*i.borrow()).to_string())
                    .collect::<String>()
            );
        });
        println!("Total Load: {}", self.total_load());
    }

    fn total_load(&self) -> usize {
        let difference = self.num_rows();
        self.items.iter().enumerate().fold(0, |acc, (i, row)| {
            let multiplier = difference - i;
            acc + multiplier * row.iter().filter(|rc| *rc.borrow() == Rock::Round).count()
        })
    }

    fn state(&self) -> PlatformState {
        PlatformState {
            item_runs: self
                .items
                .clone()
                .into_iter()
                .flatten()
                .group_by(|rc| *rc.borrow())
                .into_iter()
                .map(|(group, elems)| (elems.count(), group))
                .collect::<Vec<(usize, Rock)>>(),
        }
    }

    fn cycle(&self) {
        [
            CardinalDirection::North,
            CardinalDirection::West,
            CardinalDirection::South,
            CardinalDirection::East,
        ]
        .into_iter()
        .for_each(|direction| self.tilt(direction));

        // self.print();
    }

    fn determine_cycle_period(&mut self, max_cycles: usize) -> (usize, usize) {
        match &self.cycle_tracker {
            Some(tracker) => (tracker.num_cycles, tracker.before_period),
            None => {
                let tracker_cell = RefCell::new(CycleTracker::new());
                while !tracker_cell
                    .borrow()
                    .seen_states
                    .contains_key(&self.state())
                    && tracker_cell.borrow().num_cycles < max_cycles
                {
                    let num_cycles = tracker_cell.borrow().num_cycles;
                    let mut tc = tracker_cell.borrow_mut();
                    tc.seen_states.insert(self.state(), num_cycles);
                    tc.num_cycles += 1;
                    self.cycle();
                }
                let num_cycles = tracker_cell.borrow().num_cycles;

                if num_cycles == max_cycles {
                    return (num_cycles, 0);
                }

                let before_period = tracker_cell
                    .borrow()
                    .seen_states
                    .get(&self.state())
                    .expect("Should exist since we exited the loop.")
                    .clone();
                tracker_cell.borrow_mut().before_period = before_period;

                self.cycle_tracker = Some(tracker_cell.borrow().clone());
                (num_cycles - before_period, before_period)
            }
        }
    }

    fn cycle_n(&mut self, num_cycles: usize) {
        let (cycle_period, before_period) = self.determine_cycle_period(num_cycles);
        println!("Cycle Period: {}", cycle_period);
        let remaining = (num_cycles - before_period) % cycle_period;

        println!("Remaining: {}", remaining);
        (0..remaining).for_each(|_| self.cycle());
    }
}

impl FromStr for Platform {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Platform {
            items: s
                .trim()
                .split("\n")
                .map(|l| {
                    l.chars()
                        .map(|c| RefCell::new(Rock::from(c)))
                        .collect::<Vec<RefCell<Rock>>>()
                })
                .collect(),
            cycle_tracker: None,
        })
    }
}

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

fn part1() {
    let input = load_input("part1.txt");
    let platform = input.parse::<Platform>().unwrap();
    platform.tilt(CardinalDirection::North);
    let result = platform.total_load();

    println!("Part 1 Result: {}", result);
}

fn part2() {
    let input = load_input("part1.txt");
    let mut platform = input.parse::<Platform>().unwrap();
    platform.cycle_n(1000000000);
    let result = platform.total_load();

    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
