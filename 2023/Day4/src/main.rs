use std::convert::TryInto;
use std::str::FromStr;

#[allow(dead_code)]
#[derive(Debug)]
struct Card {
    name: String,
    winning_numbers: Vec<u32>,
    our_numbers: Vec<u32>,
    copies: u32,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Card, ()> {
        let mut parts = s.split(":");

        let name = match parts.next() {
            Some(name) => name,
            _ => return Err(()),
        };

        let number_sets: Vec<Vec<u32>> = parts
            .next()
            .ok_or(())?
            .trim()
            .split("|")
            .map(|nums| {
                nums.trim()
                    .split(" ")
                    .filter_map(|n| n.trim().parse::<u32>().ok())
                    .collect()
            })
            .collect();

        Ok(Card {
            name: name.to_string(),
            winning_numbers: number_sets[0].clone(),
            our_numbers: number_sets[1].clone(),
            copies: 1,
        })
    }
}

impl Card {
    fn num_matching(&self) -> usize {
        self.our_numbers
            .iter()
            .filter(|v| self.winning_numbers.contains(v))
            .count()
    }

    fn value(&self) -> u32 {
        let matching = self.num_matching();
        if matching == 0 {
            return 0;
        }

        (2u32).pow((matching - 1).try_into().unwrap())
    }
}

fn load_input() -> String {
    include_str!("../inputs/part1.txt").to_string()
}

#[allow(dead_code)]
fn part1() {
    let input = load_input();

    let cards: Vec<Card> = input
        .split("\n")
        .filter_map(|l| l.parse::<Card>().ok())
        .collect();

    let value: u32 = cards.iter().map(|c| c.value()).sum();

    println!("Part 1: Value of all cards: {}", value);
}

fn part2() {
    let input = load_input();

    let cards: Vec<Card> = input
        .split("\n")
        .filter_map(|l| l.parse::<Card>().ok())
        .collect();

    let copies = cards.iter().enumerate().fold(
        (0..cards.len()).map(|_| 1).collect::<Vec<usize>>(),
        |mut acc, (i, card)| {
            let val: usize = card.num_matching();
            let card_copies = acc[i];
            acc.iter_mut()
                .skip(i + 1)
                .take(val)
                .for_each(|c| *c += card_copies);

            acc
        },
    );

    let total: usize = copies.iter().sum();

    println!("Part 2: Value of all cards: {}", total);
}

fn main() {
    part1();
    part2();
}
