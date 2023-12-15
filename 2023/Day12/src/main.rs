use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
enum RecordType {
    Operational,
    Broken,
    Unknown,
}
impl From<char> for RecordType {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Broken,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => panic!("Shouldnt get here"),
        }
    }
}

fn replace_unknowns(records: Vec<RecordType>, others: Vec<RecordType>) -> Vec<RecordType> {
    let mut i = others.into_iter();
    records
        .into_iter()
        .map(|r| match r {
            RecordType::Unknown => i.next().unwrap(),
            _ => r,
        })
        .collect::<Vec<RecordType>>()
}

fn is_valid(variant: &Vec<RecordType>, original: Vec<RecordType>, groups: Vec<usize>) -> bool {
    variant.iter().zip(original).all(|(v, o)| match o {
        RecordType::Unknown => true,
        _ => *v == o,
    }) && variant
        .into_iter()
        .group_by(|v| **v == RecordType::Broken)
        .into_iter()
        .filter(|(k, _)| *k)
        .map(|(_, v)| v.count())
        .zip(groups)
        .all(|(a, b)| a == b)
}

fn num_variants(
    records_orig: Vec<RecordType>,
    broken_groups_orig: Vec<usize>,
    multiplier: usize,
) -> usize {
    let mut records = (0..multiplier)
        .map(|_| records_orig.clone())
        .collect::<Vec<Vec<RecordType>>>()
        .join(&RecordType::Unknown);

    let broken_groups = broken_groups_orig.repeat(multiplier);
    let num_broken: usize = broken_groups.iter().sum();

    let num_unknown_broken = num_broken
        - records
            .iter()
            .filter(|r| match r {
                RecordType::Broken => true,
                _ => false,
            })
            .count();
    let num_unknown = records
        .iter()
        .filter(|r| match r {
            RecordType::Unknown => true,
            _ => false,
        })
        .count();

    let seed = (0..num_unknown).map(|_| RecordType::Operational);
    (0..num_unknown)
        .combinations(num_unknown_broken)
        .map(|perm| {
            perm.iter()
                .fold(seed.clone().collect(), |mut acc: Vec<RecordType>, v| {
                    acc[*v] = RecordType::Broken;
                    acc
                })
        })
        .map(|others| replace_unknowns(records.clone(), others))
        .filter(|candidate| is_valid(&candidate, records.clone(), broken_groups.clone()))
        .count()
}

fn part1() {
    let input = include_str!("../inputs/part1.txt");

    let result: usize = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (
                parts
                    .first()
                    .unwrap()
                    .trim()
                    .chars()
                    .map(|c| RecordType::from(c))
                    .collect::<Vec<RecordType>>(),
                parts[1]
                    .trim()
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(r, g)| num_variants(r, g, 1))
        .sum();

    println!("Part 1 Result {:?}", result);
}
fn part2() {
    let input = include_str!("../inputs/example1.txt");

    let result: usize = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (
                parts
                    .first()
                    .unwrap()
                    .trim()
                    .chars()
                    .map(|c| RecordType::from(c))
                    .collect::<Vec<RecordType>>(),
                parts[1]
                    .trim()
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(r, g)| num_variants(r, g, 5))
        .sum();

    println!("Part 1 Result {:?}", result);
}
fn main() {
    part2();
}
