use itertools::Itertools;
use std::{
    cell::RefCell,
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
}

impl Platform {
    fn num_columns(&self) -> usize {
        self.items[0].len()
    }
    fn num_rows(&self) -> usize {
        self.items.len()
    }
    fn column(&self, column: usize) -> impl Iterator<Item = &RefCell<Rock>> + '_ {
        self.items.iter().map(move |row| &row[column])
    }

    fn row(&self, row: usize) -> impl Iterator<Item = &RefCell<Rock>> + '_ {
        self.items[row].iter()
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &RefCell<Rock>>> {
        self.items.iter().map(|r| r.iter())
    }

    fn sort_direction(&self, cells: Vec<&RefCell<Rock>>, direction: CardinalDirection) {
        let items = cells.iter().map(|c| *c.borrow()).collect::<Vec<Rock>>();
        // let item_iter = items.iter();
        // let item_iter = self.items.iter().map(|r| r[index].borrow().clone());
        let mut sorted_parts: Vec<Rock> = vec![];
        let mut sortable = vec![];
        let mut rest = items.clone();
        // let (mut squares, mut others): (Vec<&Rock>, Vec<&Rock>) =
        //     item_iter.clone().partition(|item| **item == Rock::Square);
        while sortable.len() > 0 || rest.len() > 0 {
            // let mut sortable: Vec<&Rock> = item_iter
            //     .clone()
            //     .take_while(|item| **item == Rock::Square)
            //     .collect::<Vec<&Rock>>();
            //
            let (non_squares, rest_others): (Vec<Rock>, Vec<Rock>) = rest
                .clone()
                .into_iter()
                .partition(|item| *item != Rock::Square);
            sortable.extend_from_slice(&non_squares);
            println!("Sortable: {:?}", sortable);
            println!("Rest: {:?}", rest_others);
            sortable.sort_by(|a: &Rock, b: &Rock| match direction {
                CardinalDirection::North | CardinalDirection::West => b.cmp(a),
                CardinalDirection::South | CardinalDirection::East => a.cmp(b),
            });

            sorted_parts.extend_from_slice(&sortable);
            let (squares, non_squares_others): (Vec<Rock>, Vec<Rock>) = rest_others
                .clone()
                .into_iter()
                .partition(|item| *item == Rock::Square);
            rest = non_squares_others;
            println!("Squares: {:?}", squares);
            sorted_parts.extend_from_slice(&squares);
            println!("sorted_parts: {:?}", sorted_parts);
        }

        cells
            .iter()
            .zip(sorted_parts.iter())
            .for_each(|(rc, r)| *rc.borrow_mut() = *r);
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
    fn print(&self) {
        self.items.iter().for_each(|r| {
            println!(
                "{}",
                r.iter()
                    .map(|i| (*i.borrow()).to_string())
                    .collect::<String>()
            );
        })
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
    let input = load_input("example1.txt");
    let platform = input.parse::<Platform>().unwrap();
    platform.print();
    platform.tilt(CardinalDirection::North);
    platform.print();
    let result = platform.items.iter().map(|r| r.len()).count();

    println!("Part 1 Result: {}", result);
}

fn part2() {
    ()
}

fn main() {
    part1();
    part2();
}
