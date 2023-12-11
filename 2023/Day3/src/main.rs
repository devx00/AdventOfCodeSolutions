use std::ops::Range;

#[derive(Debug)]
struct SchematicLocation {
    line: usize,
    column: usize,
    width: usize,
}

impl SchematicLocation {
    fn adjacent_span(&self) -> Range<usize> {
        Range {
            start: self.column.saturating_sub(1),
            end: self.column + self.width + 1,
        }
    }

    fn actual_span(&self) -> Range<usize> {
        Range {
            start: self.column,
            end: self.column + self.width,
        }
    }

    fn is_adjacent(&self, other: &SchematicLocation) -> bool {
        if other.line < self.line.saturating_sub(1) || other.line > self.line + 1 {
            return false;
        }
        self.actual_span()
            .into_iter()
            .any(|i| other.adjacent_span().contains(&i))
    }
}

#[derive(Debug)]
enum SchematicElement {
    PartNumber(u32, SchematicLocation),
    Symbol(char, SchematicLocation),
    Placeholder(SchematicLocation),
}

impl SchematicElement {
    fn location(&self) -> &SchematicLocation {
        match self {
            SchematicElement::Placeholder(location) => location,
            SchematicElement::Symbol(_, location) => location,
            SchematicElement::PartNumber(_, location) => location,
        }
    }
    fn is_adjacent(&self, other: &SchematicElement) -> bool {
        self.location().is_adjacent(other.location())
    }
}

fn parse_line(line: &str, line_index: usize) -> Vec<SchematicElement> {
    let mut char_iter = line.chars().enumerate();
    let mut elements: Vec<SchematicElement> = Vec::new();

    let mut num_chars: Vec<char> = Vec::new();

    while let Some((i, c)) = char_iter.next() {
        match c {
            '.' => elements.push(SchematicElement::Placeholder(SchematicLocation {
                line: line_index,
                column: i,
                width: 1,
            })),
            x if x.is_ascii_digit() => {
                num_chars.push(x);
                continue;
            }
            _ => elements.push(SchematicElement::Symbol(
                c,
                SchematicLocation {
                    line: line_index,
                    column: i,
                    width: 1,
                },
            )),
        }
        if num_chars.len() > 0 {
            elements.push(SchematicElement::PartNumber(
                num_chars
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
                SchematicLocation {
                    line: line_index,
                    column: i - num_chars.len(),
                    width: num_chars.len(),
                },
            ));

            num_chars.clear();
        }
    }
    if num_chars.len() > 0 {
        elements.push(SchematicElement::PartNumber(
            num_chars
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<u32>()
                .unwrap(),
            SchematicLocation {
                line: line_index,
                column: line.len() - num_chars.len(),
                width: num_chars.len(),
            },
        ));

        num_chars.clear();
    }
    elements
}

fn load_elements() -> Vec<Vec<SchematicElement>> {
    let input = include_str!("../inputs/part1.txt").to_string();
    input
        .split("\n")
        .enumerate()
        .map(|(i, l)| parse_line(l, i))
        .collect()
}

fn part1() {
    let elements = load_elements();

    let line_symbols: Vec<Vec<&SchematicElement>> = elements
        .iter()
        .map(|l| {
            l.iter()
                .filter(|x| match x {
                    SchematicElement::Symbol(_, _) => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let result: u32 = elements
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .filter(|element| match element {
                    SchematicElement::PartNumber(_, _) => line_symbols
                        .iter()
                        .skip(i.saturating_sub(1))
                        .take(3)
                        .flatten()
                        .any(|other| other.is_adjacent(element)),
                    _ => false,
                })
                .map(|el| match el {
                    SchematicElement::PartNumber(val, _) => val,
                    _ => panic!("Should never get here"),
                })
                .sum::<u32>()
        })
        .sum();
    println!("Part 1 Result: {}", result);
}

fn part2() {
    let elements = load_elements();

    let gear_ratios: Vec<Vec<&SchematicElement>> = elements
        .iter()
        .map(|l| {
            l.iter()
                .filter(|x| match x {
                    SchematicElement::PartNumber(_, _) => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let result: u32 = elements
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .filter_map(|element| match element {
                    SchematicElement::Symbol('*', _) => Some(
                        gear_ratios
                            .iter()
                            .skip(i.saturating_sub(1))
                            .take(3)
                            .flatten()
                            .filter(|other| other.is_adjacent(element))
                            .filter_map(|el| match el {
                                SchematicElement::PartNumber(val, _) => Some(val),
                                _ => None,
                            })
                            .collect::<Vec<&u32>>(),
                    ),
                    _ => None,
                })
                .filter(|ratios| ratios.len() == 2)
                .map(|ratios| ratios.iter().fold(1, |acc, v| acc * **v))
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
