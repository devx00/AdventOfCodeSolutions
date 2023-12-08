use std::convert::TryInto;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Identifier {
    key: [u8; 3],
}

impl FromStr for Identifier {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key: [u8; 3] = s.as_bytes().try_into().unwrap();
        Ok(Identifier { key })
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Hash, Copy, Clone)]
struct Location {
    left: Identifier,
    right: Identifier,
}

impl Location {
    fn direction(&self, d: Direction) -> Identifier {
        match d {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

#[derive(Debug, Clone)]
struct LocationMap {
    locations: HashMap<Identifier, Location>,
}

impl LocationMap {
    fn walk(&self, sequence: Vec<Direction>, start: Identifier, target: Identifier) -> u64 {
        let mut steps = 0;
        let mut next = start;
        let mut seq = sequence.iter().clone();
        while next != target {
            steps += 1;
            let nextd = seq
                .next()
                .or_else(|| {
                    seq = sequence.iter().clone();

                    seq.next()
                })
                .unwrap();
            next = self.locations.get(&next).expect("F").direction(*nextd);
        }

        steps
    }
}

fn main() {
    let input = include_str!("../inputs/part1.txt");

    let sequence: Vec<Direction> = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<Direction>().unwrap())
        .collect();

    let instructions: HashMap<Identifier, Location> = input
        .lines()
        .skip(2)
        .fold(&mut HashMap::new(), |acc, line| {
            let (loc, nexts) = line.split_once(" = ").unwrap();
            let (left, right) = nexts
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();

            acc.insert(
                loc.parse::<Identifier>().unwrap(),
                Location {
                    left: left.parse::<Identifier>().unwrap(),
                    right: right.parse::<Identifier>().unwrap(),
                },
            );

            acc
        })
        .clone();

    let locmap = LocationMap {
        locations: instructions,
    };

    let steps = locmap.walk(sequence, "AAA".parse().unwrap(), "ZZZ".parse().unwrap());

    println!("Steps: {}", steps);

    ()
}
