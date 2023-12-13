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

// Base case is given a single section and a one or more groups how many ways can it be arranged.
// So given a list of sections and a list of groupings, if we start with the first and see how many
// ways it can match the first or first n groupings then we can iterate over each possibility and
// recursively call that function with the remaining sections and groupings and multiply the resulting
// answer to get our total.
//
fn possible_arrangements(section: &Vec<RecordType>, groupings: &Vec<usize>) -> usize {
    println!("possible_arrangements({:?}, {:?})", section, groupings);
    // If the required number of broken records is more than the length of the section minux
    // the required number of operational records to create the specified number of groupings
    // thien this combination is impossible and we shoudl retgurn early.
    if groupings.len() == 1 && section.len() == groupings[0] {
        // println!("#2 = 1");
        return 1;
    }
    if groupings.iter().sum::<usize>() > (section.len() - (groupings.len() - 1))
        || section.len() == 0
        || groupings.len() == 0
    {
        // println!("#1 = 0");
        return 0;
    }

    if section[0] == RecordType::Broken {
        // Then the first part must be the first grouping. So make sure it fits, and if it does
        // apply it and re-call possible_arrangements with the remaining groupings and records if
        // any.

        if section[groupings[0]] != RecordType::Unknown {
            // println!("#3 = 0");
            return 0;
        }

        if groupings.len() == 1 && section.len() > groupings[0] {
            return possible_arrangements(
                &section.iter().skip(groupings[0] + 1).map(|v| *v).collect(),
                &groupings,
            ) + 1;
        }

        // println!("#4 = call to possible_arrangements");
        return possible_arrangements(
            &section.iter().skip(groupings[0] + 1).map(|v| *v).collect(),
            &groupings.iter().skip(1).map(|v| *v).collect(),
        );
    }
    let partial_section: Vec<RecordType> = section.iter().skip(1).map(|v| *v).collect();
    let mut modified_section = section.clone();
    modified_section[0] = RecordType::Broken;

    let mut partial_res: usize = 0;
    if partial_section.len() > 0 {
        partial_res = possible_arrangements(&partial_section, groupings);
    }

    let res = partial_res + possible_arrangements(&modified_section, groupings);

    // println!("#5 = {}", res);

    res
}

fn possible_arrangements_multi(sections: &Vec<Vec<RecordType>>, groupings: Vec<usize>) -> usize {
    println!(
        "possible_arrangements_multi({:?}, {:?})",
        sections, groupings
    );
    if sections.len() == 1 {
        return possible_arrangements(sections.first().unwrap(), &groupings);
    }

    let mut total = 0;
    let max_num_groupings = groupings.len() - (sections.len() - 1);
    for i in 0..max_num_groupings {
        let n_possibilities = possible_arrangements(
            sections.first().unwrap(),
            &groupings
                .iter()
                .take(i + 1)
                .map(|v| *v)
                .collect::<Vec<usize>>(),
        );
        // println!("=== {}", n_possibilities);
        println!("N: {}", n_possibilities);

        if n_possibilities == 0 {
            break;
        }

        let mut sec_copy = sections.clone();
        let _ = sec_copy.remove(0);
        if groupings.len() > i + 1 {
            total += n_possibilities
                * possible_arrangements_multi(
                    &sec_copy,
                    groupings.clone().iter().skip(i + 1).map(|v| *v).collect(),
                );
        } else {
            total += n_possibilities
        }
    }

    total
}

fn partition_records(records: Vec<RecordType>) -> Vec<Vec<RecordType>> {
    let mut parted: Vec<Vec<RecordType>> = Vec::new();
    parted.push(Vec::new());

    for record in records {
        match record {
            RecordType::Broken | RecordType::Unknown => {
                parted.iter_mut().last().unwrap().push(record);
            }
            _ => {
                if parted.last().unwrap().len() > 0 {
                    parted.push(Vec::new());
                }
            }
        }
    }

    if parted.last().unwrap().len() == 0 {
        parted.pop();
    }

    parted
}

fn part1() {
    let input = include_str!("../inputs/example1.txt");

    let result: usize = input
        .lines()
        .skip(4)
        .take(1)
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
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(records, groupings)| {
            let res = possible_arrangements_multi(&records, groupings);
            println!("Result: {}", res);
            res
        })
        .sum();

    println!("Part 1 Result {:?}", result);
}
fn main() {
    part1();
}
