use std::cell::RefCell;

use itertools::EitherOrBoth::{Both, Right};
use itertools::{repeat_n, Itertools};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct BrokenGroup {
    index: usize,
    width: usize,
}

impl BrokenGroup {
    fn new(index: usize, width: usize) -> BrokenGroup {
        BrokenGroup { index, width }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct BrokenGroupCombo<'a> {
    groups: RefCell<Vec<BrokenGroup>>,
    n_variations: RefCell<usize>,
    next_groups: RefCell<Vec<&'a BrokenGroupCombo<'a>>>,
}

impl BrokenGroupCombo<'_> {
    fn new(groups: Vec<BrokenGroup>, n_variations: usize) -> BrokenGroupCombo<'static> {
        BrokenGroupCombo {
            groups: RefCell::new(groups),
            n_variations: RefCell::new(n_variations),
            next_groups: RefCell::new(vec![]),
        }
    }
    fn num_insertion_points(&self) -> usize {
        self.groups.borrow().len() + 1
    }
    fn n_variable_insertions(&self, record_part: &Box<RecordPart>) -> usize {
        record_part.records.borrow().len()
            - (self.groups.borrow().iter().map(|g| g.width).sum::<usize>()
                + (self.groups.borrow().len() - 1))
    }
    fn calculate_variations(&self, record_part: &Box<RecordPart>) -> usize {
        let new_variations = (0..self.num_insertion_points())
            .combinations_with_replacement(self.n_variable_insertions(record_part))
            .map(|v| v.into_iter().counts())
            .map(|insertions| {
                (0..self.num_insertion_points())
                    .enumerate()
                    .map(|(i, ip)| {
                        let base: usize = if i == 0 || i == self.num_insertion_points() - 1 {
                            0
                        } else {
                            1
                        };

                        match insertions.get(&ip) {
                            Some(v) => *v,
                            None => base,
                        }
                    })
                    .collect::<Vec<usize>>()
            })
            .map(|ips| {
                self.groups
                    .borrow()
                    .iter()
                    .zip_longest(ips.iter())
                    .map(|val| match val {
                        Both(g, n) => repeat_n(RecordType::Operational, *n)
                            .chain(repeat_n(RecordType::Broken, g.width))
                            .collect::<Vec<RecordType>>(),
                        Right(last) => {
                            repeat_n(RecordType::Operational, *last).collect::<Vec<RecordType>>()
                        }
                        _ => panic!("Shouldnt have more in left than right"),
                    })
                    .flatten()
                    .collect::<Vec<RecordType>>()
            })
            .filter(|variant| {
                // println!("Checking: {:?}", variant);
                let valid = record_part.is_valid_recordset(variant);
                // if valid {
                //     println!("Valid: {:?}", variant);
                // }
                valid
            })
            .count();
        // println!("Variations: {}, {:?}", new_variations, self.groups.borrow());
        if *self.n_variations.borrow() > 0 {
            *self.n_variations.borrow_mut() *= new_variations;
        } else {
            *self.n_variations.borrow_mut() = new_variations;
        }

        *self.n_variations.borrow()
    }
}

fn partition_records(records: Vec<RecordType>) -> Vec<Box<RecordPart<'static>>> {
    records
        .into_iter()
        .group_by(|r| *r != RecordType::Operational)
        .into_iter()
        .map(|(_, g)| g.collect())
        .filter(|v: &Vec<RecordType>| v.iter().all(|r| *r != RecordType::Operational))
        .map(|rec| RecordPart::new(rec))
        .collect::<Vec<Box<RecordPart>>>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RecordPart<'a> {
    records: RefCell<Vec<RecordType>>,
    valid_group_combos: RefCell<Vec<BrokenGroupCombo<'a>>>,
    next_part: RefCell<Option<&'a Box<RecordPart<'a>>>>,
    prev_part: RefCell<Option<&'a RecordPart<'a>>>,
}

impl RecordPart<'_> {
    fn new(records: Vec<RecordType>) -> Box<RecordPart<'static>> {
        Box::new(RecordPart {
            records: RefCell::new(records),
            valid_group_combos: RefCell::new(vec![]),
            next_part: RefCell::new(None),
            prev_part: RefCell::new(None),
        })
    }

    fn is_valid_recordset(&self, records: &Vec<RecordType>) -> bool {
        self.records
            .borrow()
            .iter()
            .zip(records.iter())
            .all(|(actual, variant)| match actual {
                RecordType::Broken => variant == actual,
                _ => true,
            })
    }

    fn groups_fit(&self, groups: Vec<BrokenGroup>) -> bool {
        groups.iter().map(|g| g.width).sum::<usize>() + (groups.len() - 1)
            <= self.records.borrow().len()
    }

    fn fitting_groups(
        &self,
        prev_variations: usize,
        available_groups: Vec<BrokenGroup>,
    ) -> Vec<BrokenGroupCombo<'static>> {
        let mut fit_groups: Vec<BrokenGroup> = vec![];
        for group in available_groups {
            // println!("Doesst fit: {:?} in {}", group, self.records.borrow().len());
            let groups = fit_groups
                .clone()
                .into_iter()
                .chain(vec![group].into_iter())
                .collect();
            if self.groups_fit(groups) {
                fit_groups.push(group.clone());
            } else {
                break;
            }
        }
        (1..fit_groups.len() + 1)
            .map(|n| {
                BrokenGroupCombo::new(
                    fit_groups.clone().into_iter().take(n).collect(),
                    prev_variations,
                )
            })
            .collect::<Vec<BrokenGroupCombo>>()
    }
}

fn part1() {
    let input = include_str!("../inputs/part1.txt");

    let result: usize = input
        .lines()
        // .skip(5)
        // .take(1)
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            (
                partition_records(
                    parts
                        .first()
                        .unwrap()
                        .trim()
                        .chars()
                        .map(|c| RecordType::from(c))
                        .collect::<Vec<RecordType>>(),
                ),
                parts[1]
                    .trim()
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .enumerate()
                    .map(|(i, g)| BrokenGroup::new(i, g))
                    .collect::<Vec<BrokenGroup>>(),
            )
        })
        .map(|(records, groupings)| {
            records
                .iter()
                .enumerate()
                .fold(vec![BrokenGroupCombo::new(vec![], 0)], |acc, (i, r)| {
                    let part_res: Vec<BrokenGroupCombo> = acc
                        .iter()
                        .map(|gc| {
                            let start_index = match gc.groups.borrow().last() {
                                Some(g) => g.index + 1,
                                _ => 0,
                            };
                            // let start_index = if i == 0 {
                            //     0
                            // } else {
                            //     gc.groups.borrow().last().unwrap().index + 1
                            // };
                            // let start_index = match *gc.n_variations.borrow() {
                            //     0 => 0,
                            //     _ => gc.groups.borrow().last().unwrap().index + 1,
                            // };
                            let start_variations = if start_index == 0 {
                                1
                            } else {
                                *gc.n_variations.borrow()
                            };
                            // println!(start_index, start_variations);
                            let available_groups =
                                groupings.clone().into_iter().skip(start_index).collect();

                            r.fitting_groups(start_variations, available_groups)
                                .into_iter()
                                .filter_map(|gc| match gc.calculate_variations(r) {
                                    // 0 => None,
                                    _ => Some(gc),
                                })
                                // .chain(vec![BrokenGroupCombo::new(vec![], 1)].into_iter())
                                .collect::<Vec<BrokenGroupCombo>>()
                        })
                        .flatten()
                        .map(|v| v)
                        .collect();
                    if part_res.len() > 0 {
                        part_res
                    } else {
                        // vec![BrokenGroupCombo::new(vec![], 1)]
                        acc
                    }
                })
                .iter()
                .filter_map(|gc| match gc.groups.borrow().last() {
                    Some(group) => {
                        if group.index == groupings.iter().last().unwrap().index {
                            Some(*gc.n_variations.borrow())
                        } else {
                            None
                        }
                    }
                    _ => None::<usize>,
                })
                .sum::<usize>()
            // let res = possible_arrangements_multi(&records, groupings);
            // println!("Result: {}", res);
            // res
        })
        .zip(input.lines())
        .map(|(s, l)| {
            println!("{} ==> {}", l, s);
            s
        })
        .sum();

    println!("Part 1 Result {:?}", result);
}
fn main() {
    part1();
}
