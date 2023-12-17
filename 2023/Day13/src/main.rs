use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct Pattern {
    rows: RefCell<Vec<String>>,
    cols: RefCell<Vec<String>>,
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

    fn find_reflected(v: &RefCell<Vec<String>>) -> Option<usize> {
        let mut valmap: HashMap<String, Vec<usize>> = HashMap::new();
        let mut running_matches: Vec<usize> = vec![];

        for (i, v) in v.borrow().iter().enumerate() {
            let match_vec = valmap
                .entry(v.to_string())
                .and_modify(|e| e.push(i))
                .or_insert(vec![i]);

            if i == 0 {
                continue;
            }

            running_matches = running_matches
                .into_iter()
                .chain(vec![i].into_iter())
                .filter(|matched| {
                    let match_distance = (2 * (i - matched)) + 1;
                    println!("{}, {}", i, match_distance);
                    let complement_index = i
                        .checked_sub(match_distance)
                        .expect("Should never go negative");
                    match_vec.contains(&complement_index)
                })
                .collect();

            for matched in running_matches.iter() {
                if i == (2 * matched) - 1 {
                    return Some(*matched);
                }
            }
        }

        running_matches.get(0).copied()
    }

    fn new(input: String) -> Pattern {
        let rows: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        Pattern {
            rows: RefCell::new(rows.clone()),
            cols: Pattern::columns(&rows).into(),
        }
    }

    fn find_reflection_point(&self) -> usize {
        println!(
            "Finding Reflection Point:\n=========Rows=========\n{}\n===========Cols==========\n{}",
            self.rows.borrow().join("\n"),
            self.cols.borrow().join("\n")
        );
        match Pattern::find_reflected(&self.rows) {
            Some(reflected_row) => {
                println!("Reflected Row: {}", reflected_row);
                100 * reflected_row
            }
            None => match Pattern::find_reflected(&self.cols) {
                Some(reflected_col) => {
                    println!("Reflected Column: {}", reflected_col);
                    reflected_col
                }
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
        .map(|p| {
            let ref_val = Pattern::new(p.to_string()).find_reflection_point();
            println!("\nVal for pattern: {}\n{}", ref_val, p);
            ref_val
        })
        .sum();

    println!("Part 1 Result: {}", result);
}

fn part2() {}

fn main() {
    part1();
}
