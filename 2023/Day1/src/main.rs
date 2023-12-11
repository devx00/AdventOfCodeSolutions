use std::collections::HashMap;

fn load_input() -> String {
    include_str!("../inputs/part1.txt").to_string()
}

fn part1() {
    let input = load_input();
    let answer: u32 = input
        .trim()
        .split("\n")
        .map(|l| {
            let digits: Vec<char> = l
                .chars()
                .filter(|c| match c {
                    x if x.is_ascii_digit() => true,
                    _ => false,
                })
                .collect();

            vec![
                digits.first().expect("Expected at least 1 digit"),
                digits.last().expect("Expected at least 1 digit."),
            ]
            .iter()
            .cloned()
            .collect::<String>()
            .parse::<u32>()
            .expect("Failed to parse number")
        })
        .sum();
    println!("Part 1 Solution: {}", answer);
}
fn part2() {
    let input = load_input();
    let answer: u64 = input
        .trim()
        .split("\n")
        .map(|l| {
            let mut i = 0;
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            while i < l.len() {
                let subs = &l[i..];
                i += 1;
                let mut c = subs.chars().next().unwrap();
                if !c.is_ascii_digit() {
                    let num_hash = HashMap::from([
                        ("zero", '0'),
                        ("one", '1'),
                        ("two", '2'),
                        ("three", '3'),
                        ("four", '4'),
                        ("five", '5'),
                        ("six", '6'),
                        ("seven", '7'),
                        ("eight", '8'),
                        ("nine", '9'),
                    ]);

                    let found = num_hash.keys().find(|s| subs.starts_with(&s.to_string()));

                    match found {
                        Some(nc) => c = *num_hash.get(*nc).unwrap(),
                        _ => continue,
                    }
                }

                if first.is_none() {
                    first = Some(c);
                }

                last = Some(c);
            }

            let res: u64 = vec![first, last]
                .iter()
                .map(|v| {
                    // println!("Joined: {}\n\n", v);
                    v.unwrap()
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            res
        })
        .sum();
    println!("Part 2 Solution: {}", answer);
}
fn main() {
    part1();
    part2();
}
