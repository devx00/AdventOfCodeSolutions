use std::collections::HashMap;

fn load_input() -> String {
    include_str!("../inputs/part1.txt").to_string()
}

fn part1() {
    let input = load_input();
    let vals = input.trim().lines().map(|line| {
        line.split(" ")
            .filter_map(|x| match x.parse::<u32>() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect::<Vec<_>>()
    });
    let mut left_vals = vals.clone().map(|v| v[0]).collect::<Vec<u32>>();
    let mut right_vals = vals.clone().map(|v| v[1]).collect::<Vec<u32>>();

    left_vals.sort();
    right_vals.sort();

    let diff = left_vals
        .iter()
        .zip(right_vals.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .map(|v| v)
        .sum::<u32>();

    println!("Part 1: {}", diff);
}

fn part2() {
    let input = load_input();
    let vals = input.trim().lines().map(|line| {
        line.split(" ")
            .filter_map(|x| match x.parse::<u32>() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect::<Vec<_>>()
    });
    let mut left_vals = vals.clone().map(|v| v[0]).collect::<Vec<u32>>();
    let mut right_vals = vals.clone().map(|v| v[1]).collect::<Vec<u32>>();

    left_vals.sort();
    right_vals.sort();

    let right_counts: HashMap<u32, u32> = right_vals.iter().fold(HashMap::new(), |mut hv, v| {
        hv.entry(*v).and_modify(|v| *v += 1).or_insert(1);
        hv
    });

    let simval: u32 = left_vals
        .iter()
        .map(|v| right_counts.get(&v).unwrap_or(&0) * *v)
        .sum();

    println!("Part 2: {}", simval);
}

fn main() {
    part1();
    part2();
}
