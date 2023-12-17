use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone, Hash)]
struct Match {
    index: usize,
    has_seen_smudge: bool,
}

#[derive(Debug)]
struct Pattern {
    rows: RefCell<Vec<String>>,
    cols: RefCell<Vec<String>>,
}

fn distance(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

impl Pattern {
    fn column(rows: &Vec<String>, i: usize) -> String {
        rows.iter()
            .map(move |r| r.chars().nth(i).unwrap())
            .collect()
    }

    fn columns(rows: &Vec<String>) -> Vec<String> {
        (0..rows[0].len())
            .map(|i| Pattern::column(rows, i).clone())
            .collect()
    }

    fn find_reflected(row_or_column: &RefCell<Vec<String>>, smudge_allowed: bool) -> Option<Match> {
        let mut running_matches: Vec<Match> = vec![];
        let rc = row_or_column.borrow();
        for (i, val) in rc.iter().enumerate() {
            if i == 0 {
                continue;
            }

            running_matches = running_matches
                .into_iter()
                .chain(
                    vec![Match {
                        index: i,
                        has_seen_smudge: false,
                    }]
                    .into_iter(),
                )
                .filter_map(|matched| {
                    let match_distance = (2 * (i - matched.index)) + 1;
                    let (complement_index, overflown) = i.overflowing_sub(match_distance);
                    if overflown {
                        return None;
                    }
                    let complement_str = rc.get(complement_index).unwrap();
                    let accepted_distances = match matched.has_seen_smudge || !smudge_allowed {
                        true => vec![0],
                        _ => vec![0, 1],
                    };

                    let dist = distance(val, complement_str);
                    if accepted_distances.contains(&dist) {
                        Some(Match {
                            index: matched.index,
                            has_seen_smudge: matched.has_seen_smudge || dist > 0,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            for matched in running_matches.iter() {
                if i == (2 * matched.index) - 1 && (matched.has_seen_smudge || !smudge_allowed) {
                    return Some(matched.clone());
                }
            }
        }

        running_matches
            .iter()
            .filter(|m| m.has_seen_smudge || !smudge_allowed)
            .nth(0)
            .cloned()
    }

    fn new(input: String) -> Pattern {
        let rows: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        Pattern {
            rows: RefCell::new(rows.clone()),
            cols: Pattern::columns(&rows).into(),
        }
    }

    fn find_reflection_point(&self, smudge_allowed: bool) -> usize {
        match Pattern::find_reflected(&self.rows, smudge_allowed) {
            Some(reflected_row) => 100 * reflected_row.index,
            None => match Pattern::find_reflected(&self.cols, smudge_allowed) {
                Some(reflected_col) => reflected_col.index,
                None => panic!("Should always find either a reflected column or a reflected row."),
            },
        }
    }
}

fn part1() {
    let input = include_str!("../inputs/part1.txt");
    let result: usize = input
        .trim()
        .split("\n\n")
        .map(|p| Pattern::new(p.to_string()).find_reflection_point(false))
        .sum();

    println!("Part 1 Result: {}", result);
}

fn part2() {
    let input = include_str!("../inputs/part1.txt");
    let result: usize = input
        .trim()
        .split("\n\n")
        .map(|p| Pattern::new(p.to_string()).find_reflection_point(true))
        .sum();

    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
