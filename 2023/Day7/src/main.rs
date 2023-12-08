use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Joker),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from(cards: &[Card; 5]) -> HandType {
        let mut counts = cards
            .iter()
            .filter(|e| **e != Card::Joker)
            .fold(&mut HashMap::new(), |acc, e| {
                let count = acc.entry(e).or_insert(0);
                *count += 1;
                acc
            })
            .values()
            .cloned()
            .collect::<Vec<u32>>();

        counts.sort_unstable_by(|a, b| b.cmp(a));

        let num_non_jokers: u32 = counts.iter().sum();

        if num_non_jokers == 0 {
            return HandType::FiveOfAKind;
        }

        counts[0] += 5 - num_non_jokers;

        match counts.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1] => HandType::Pair,
            [2, 2, 1] => HandType::TwoPair,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [3, 2] => HandType::FullHouse,
            [4, 1] => HandType::FourOfAKind,
            [5] => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn from(cards: [Card; 5], bid: u32) -> Hand {
        let hand_type = HandType::from(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .map(|(cards, bid)| {
                let bid: u32 = bid.parse().unwrap();
                let cards: [Card; 5] = cards
                    .chars()
                    .map(|card| card.to_string().parse().unwrap())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap();

                Hand::from(cards, bid)
            })
            .ok_or(())
    }
}

fn main() {
    let input_file = File::open("inputs/part2.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut hands = reader
        .lines()
        .map(|line| {
            line.expect("Expected line to be present but received none.")
                .parse::<Hand>()
                .unwrap()
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    let winning_total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, e)| acc + (i as u32 + 1) * e.bid);

    println!("Total: {}", winning_total);

    ()
}
