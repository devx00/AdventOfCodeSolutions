use std::cmp::{max, min};
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

struct Instructions {
    sequence: Vec<Direction>,
    idx: usize,
}

impl Iterator for Instructions {
    type Item = Direction;

    fn next(&mut self) -> Option<Direction> {
        if self.idx >= self.sequence.len() {
            self.idx = 0;
        }

        let n = Some(self.sequence[self.idx]);

        self.idx += 1;

        n
    }
}

impl Instructions {
    fn reset(&mut self) {
        self.idx = 0;
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
struct Node {
    identifier: Identifier,
    left: Identifier,
    right: Identifier,
}

impl Node {
    fn direction(&self, d: Direction) -> Identifier {
        match d {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

// Trying to do these challenges without any crates so this is
// my attempt at an LCM function. Its just the first way I thought
// to do it and its likely not ideal lol.
fn naive_lcm(a: usize, b: usize) -> usize {
    let mut high = max(a, b);
    let mut low = min(a, b);

    while high != low {
        low += min(a, b);
        if low > high {
            high += max(a, b);
        }
    }

    high
}

fn count_from(
    location: &Node,
    lmap: &HashMap<Identifier, Node>,
    sequence: Vec<Direction>,
) -> usize {
    let mut instructions = Instructions { sequence, idx: 0 };

    let mut count = 0;
    let mut loc = location;
    while loc.identifier.key[2] != b'Z' {
        let nexti = instructions
            .next()
            .expect("Should always have a next instruction.");
        loc = lmap
            .get(&loc.direction(nexti))
            .expect("Should always have a node.");
        count += 1;
    }

    println!(
        "Finished {:?} with {:?} cycles.",
        location.identifier.key, count
    );
    count
}

// Need to refactor to find the num cycles for each and then find the LCM of those.
fn main() {
    let input = include_str!("../inputs/part2.txt");

    let sequence: Vec<Direction> = input
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| c.to_string().parse::<Direction>().unwrap())
        .collect();

    let locations: HashMap<Identifier, Node> = input
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

            let id = loc.parse::<Identifier>().unwrap();
            acc.insert(
                id,
                Node {
                    identifier: id,
                    left: left.parse::<Identifier>().unwrap(),
                    right: right.parse::<Identifier>().unwrap(),
                },
            );

            acc
        })
        .clone();

    let answer = locations
        .iter()
        .filter(|(k, _)| k.key[2] == b'A')
        .map(|(_, v)| {
            let node = v;

            count_from(&node, &locations, sequence.clone())
        })
        // .collect::<Vec<usize>>()
        .reduce(|acc, v| naive_lcm(acc, v))
        .unwrap();
    //
    println!("Steps: {:?}", answer);

    ()
}
//
