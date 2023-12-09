use std::{iter::zip, str::FromStr};

#[derive(Debug)]
struct Sequence {
    values: Vec<i32>,
}

impl Sequence {
    fn derivative(&self) -> Sequence {
        let vals = zip(self.values.iter(), self.values.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        Sequence { values: vals }
    }

    fn next_value(&self) -> i32 {
        if self.values.iter().all(|x| *x == 0) {
            return 0;
        }

        let derivative_next = self.derivative().next_value();

        self.values.iter().last().unwrap() + derivative_next
    }

    fn prev_value(&self) -> i32 {
        if self.values.iter().all(|x| *x == 0) {
            return 0;
        }

        let derivative_prev = self.derivative().prev_value();

        self.values.iter().nth(0).unwrap() - derivative_prev
    }
}

impl FromStr for Sequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Sequence, ()> {
        let values = s
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();

        Ok(Sequence { values })
    }
}

fn part1() {
    let result: i32 = include_str!("../inputs/part1.txt")
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }

            Some(l.parse::<Sequence>().expect("Expected a seq").next_value())
        })
        .sum();

    println!("Part 1 Result: {:?}", result);
}

fn part2() {
    let result: i32 = include_str!("../inputs/part2.txt")
        .split("\n")
        .filter_map(|l| {
            if l.is_empty() {
                return None;
            }

            Some(l.parse::<Sequence>().expect("Expected a seq").prev_value())
        })
        .sum();

    println!("Part 2 Result: {:?}", result);
}

fn main() {
    part1();
    part2();
}
