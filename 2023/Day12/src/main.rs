use itertools::Itertools;
use log::{debug, info, trace, LevelFilter};
use std::cell::RefCell;
use std::collections::HashMap;

use env_logger::Env;

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
impl ToString for RecordType {
    fn to_string(&self) -> String {
        match self {
            Self::Broken => String::from("#"),
            Self::Operational => String::from("."),
            Self::Unknown => String::from("?"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct EvaluationPosition {
    record_index: usize,
    group_index: usize,
}

impl EvaluationPosition {
    fn new(record_index: usize, group_index: usize) -> EvaluationPosition {
        EvaluationPosition {
            record_index,
            group_index,
        }
    }

    fn start() -> EvaluationPosition {
        EvaluationPosition {
            record_index: 0,
            group_index: 0,
        }
    }

    fn get_records_string(&self, input: &InputData) -> String {
        input
            .records
            .iter()
            .skip(self.record_index)
            .map(|r| r.to_string())
            .join("")
    }
}

struct BrokenGroup {
    start_index: usize,
    length: usize,
}

impl BrokenGroup {
    fn new(start_index: usize, length: usize) -> BrokenGroup {
        BrokenGroup {
            start_index,
            length,
        }
    }
}

struct InputData {
    records: Vec<RecordType>,
    groups: Vec<usize>,
    position_cache: RefCell<HashMap<EvaluationPosition, usize>>,
    arrangement_cache: RefCell<HashMap<EvaluationPosition, Vec<Vec<EvaluationPosition>>>>,
}
impl InputData {
    fn new(records: Vec<RecordType>, groups: Vec<usize>) -> InputData {
        InputData {
            records,
            groups,
            position_cache: RefCell::new(HashMap::new()),
            arrangement_cache: RefCell::new(HashMap::new()),
        }
    }

    fn last_possible_index(&self, group_index: usize) -> usize {
        // Implement some logic here to narrow this down. For now just returning the last index -
        // group len.
        self.records.len() - self.groups[group_index]
    }

    fn last_index(&self, from_position: EvaluationPosition) -> usize {
        match self.next_known_broken_group(from_position) {
            Some(group) => {
                if group.length <= self.groups[from_position.group_index] {
                    group.start_index
                } else {
                    group.start_index - self.groups[from_position.group_index] - 1
                }
            }
            _ => self.last_possible_index(from_position.group_index),
        }
    }

    fn known_broken_groups(&self) -> Vec<BrokenGroup> {
        self.records
            .iter()
            .enumerate()
            .group_by(|(_i, r)| **r == RecordType::Broken)
            .into_iter()
            .filter_map(|(is_match, group)| {
                if is_match == false {
                    return None;
                }
                let group_vec: Vec<(usize, &RecordType)> = group.collect();
                match group_vec.first() {
                    Some((i, _)) => Some(BrokenGroup::new(*i, group_vec.len())),
                    _ => None,
                }
            })
            .collect_vec()
    }

    fn next_known_broken_group(&self, from_position: EvaluationPosition) -> Option<BrokenGroup> {
        self.known_broken_groups()
            .into_iter()
            .find(|g| g.start_index >= from_position.record_index)
    }

    fn possible_positions(
        &self,
        from_position: EvaluationPosition,
    ) -> impl Iterator<Item = EvaluationPosition> + '_ + Clone {
        let end_index = self.last_index(from_position);
        debug!("{:?} ends @ {}", from_position, end_index);
        (from_position.record_index..end_index + 1).filter_map(move |i| {
            if self
                .records
                .iter()
                .skip(i)
                .take(self.groups[from_position.group_index])
                .all(|r| match r {
                    RecordType::Unknown | RecordType::Broken => true,
                    _ => false,
                })
                && match self
                    .records
                    .iter()
                    .skip(i + self.groups[from_position.group_index])
                    .next()
                {
                    Some(RecordType::Broken) => false,
                    _ => true,
                }
            {
                debug!(
                    "Broken Group of Len: {} valid at index: {}",
                    self.groups[from_position.group_index], i
                );
                Some(EvaluationPosition::new(
                    i + self.groups[from_position.group_index] + 1,
                    from_position.group_index + 1,
                ))
            } else {
                debug!(
                    "Doesnt fit: {}, {:?}",
                    i,
                    self.records.iter().skip(i).next()
                );
                None
            }
        })
    }

    fn possible_arrangements(
        &self,
        start_position: EvaluationPosition,
    ) -> Vec<Vec<EvaluationPosition>> {
        if let Some(cache_hit) = self.arrangement_cache.borrow().get(&start_position) {
            trace!("Cache Hit!: {:?}", start_position);
            return cache_hit.clone();
        }

        let possible_positions = self.possible_positions(start_position);
        let possible_count = possible_positions.clone().count();
        let value: Vec<Vec<EvaluationPosition>> =
            if start_position.group_index == self.groups.len() - 1 {
                possible_positions.map(|p| vec![p]).collect()
            } else {
                possible_positions
                    .map(|pos| {
                        debug!(
                        "Calculating num arrangements for position: {:?} from position: {:?} '{}'",
                        pos,
                        start_position,
                        start_position.get_records_string(&self)
                    );
                        let mut arrangements = self.possible_arrangements(pos);
                        arrangements
                            .iter_mut()
                            .for_each(|arr| arr.insert(0, pos.clone()));
                        arrangements
                    })
                    .concat()
                    .into_iter()
                    .dedup()
                    .collect()
            };

        if value.len() == 0 {
            debug!(
                "Found 0/{} possible positions for group: [{}] {} evaluated from location: {} '{}'",
                possible_count,
                start_position.group_index,
                self.groups[start_position.group_index],
                start_position.record_index,
                start_position.get_records_string(&self)
            );
        }
        self.arrangement_cache
            .borrow_mut()
            .insert(start_position, value.clone());
        value
    }

    fn num_arrangements(&self, start_position: EvaluationPosition) -> usize {
        if let Some(cache_hit) = self.position_cache.borrow().get(&start_position) {
            trace!("Cache Hit!: {:?}", start_position);
            return cache_hit.clone();
        }

        let possible_positions = self.possible_positions(start_position);
        let possible_count = possible_positions.clone().count();
        let value = if start_position.group_index == self.groups.len() - 1 {
            possible_count
        } else {
            possible_positions
                .map(|pos| {
                    debug!(
                        "Calculating num arrangements for position: {:?} from position: {:?} '{}'",
                        pos,
                        start_position,
                        start_position.get_records_string(&self)
                    );
                    self.num_arrangements(pos)
                })
                .sum()
        };

        if value == 0 {
            debug!(
                "Found 0/{} possible positions for group: [{}] {} evaluated from location: {} '{}'",
                possible_count,
                start_position.group_index,
                self.groups[start_position.group_index],
                start_position.record_index,
                start_position.get_records_string(&self)
            );
        }
        self.position_cache
            .borrow_mut()
            .insert(start_position, value.clone());
        value
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
    let records = (0..multiplier)
        .map(|_| records_orig.clone())
        .collect::<Vec<Vec<RecordType>>>()
        .join(&RecordType::Unknown);

    let broken_groups = broken_groups_orig.repeat(multiplier);
    debug!("broken_groups {:?}", broken_groups);
    debug!("records {:?}", records);
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
    let input = include_str!("../inputs/debug2.txt");

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
        .map(|(r, g)| {
            let nvariants = num_variants(r.clone(), g.clone(), 1);

            println!(
                "{}, {:?} ==> {}",
                r.iter().map(|r| r.to_string()).collect::<String>(),
                g,
                nvariants
            );

            nvariants
        })
        .sum();

    info!("Part 1 Result {:?}", result);
}
fn part2() {
    let input = include_str!("../inputs/debug2.txt");

    let result: usize = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            InputData::new(
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
        .map(|rec| {
            let variants = rec.possible_arrangements(EvaluationPosition::start());
            println!(
                "{}, {:?} ==> {}",
                EvaluationPosition::start().get_records_string(&rec),
                rec.groups,
                variants.len()
            );

            variants.iter().for_each(|v| {
                info!(
                    "Variant: {:?}",
                    v.iter().map(|pos| pos.record_index).collect::<Vec<_>>()
                )
            });

            variants.len()
        })
        .sum();

    info!("Part 2 Result {:?}", result);
}
fn main() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");

    env_logger::init_from_env(env);
    log::set_max_level(LevelFilter::Debug);
    part1();
    part2();
}
