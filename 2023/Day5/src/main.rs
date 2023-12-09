use std::ops::Range as StdRange;
use std::str::FromStr;

#[derive(Debug, Clone, Hash)]
struct Category {
    name: String,
    ranges: Vec<Range>,
    next_category: Option<Box<Category>>,
}

impl Category {
    fn from_line_iter<'a, I>(line_iter: &mut I) -> Option<Category>
    where
        I: Iterator<Item = &'a str>,
    {
        let category_line = line_iter.next();

        if !category_line?.trim().ends_with("map:") {
            println!("Doesnt end with map: {:?}", category_line);
            return None;
        }

        let category_name = category_line?.split("-").next().unwrap();

        let ranges = line_iter
            .take_while(|l| !l.trim().is_empty())
            .filter_map(|l| l.trim().parse::<Range>().ok())
            .collect();

        let next_category = match Category::from_line_iter(line_iter) {
            Some(cat) => Some(Box::new(cat)),
            _ => None,
        };

        Some(Category {
            name: category_name.to_string(),
            ranges,
            next_category,
        })
    }

    fn range_for(&self, val: usize) -> Option<Range> {
        self.ranges.iter().find(|r| r.contains(val)).cloned()
    }

    fn lookup(&self, val: usize) -> usize {
        let rebased_val = match self.range_for(val) {
            Some(range) => range.rebase(val),
            _ => val,
        };

        // println!(
        //     "{} in {} maps to {} in {}",
        //     val,
        //     self.name,
        //     rebased_val,
        //     match &self.next_category {
        //         Some(cat) => &cat.name,
        //         _ => "End",
        //     }
        // );

        match &self.next_category {
            Some(cat) => cat.lookup(rebased_val),
            _ => rebased_val,
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Range {
    start: usize,
    dest_start: usize,
    width: usize,
}

impl Range {
    fn contains(&self, val: usize) -> bool {
        self.start <= val && val < self.start + self.width
    }

    fn rebase(&self, val: usize) -> usize {
        self.dest_start + (val - self.start)
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Range, ()> {
        let range_params = s
            .split(" ")
            .map(|v| v.parse::<usize>().unwrap())
            .take(3)
            .collect::<Vec<usize>>();

        match range_params.as_slice() {
            [dest_start, start, width] => Ok(Range {
                start: *start,
                dest_start: *dest_start,
                width: *width,
            }),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
fn part1() {
    let s = include_str!("../inputs/part1.txt");

    let mut line_iter = s.split("\n").into_iter();

    let seeds: Vec<usize> = line_iter
        .next()
        .expect("Expected at least 1 line...")
        .strip_prefix("seeds: ")
        .expect("Should have started with 'seeds: '")
        .split(" ")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    println!("Seeds: {:?}", seeds);

    let main_category = match line_iter.next() {
        Some("") => match Category::from_line_iter(&mut line_iter) {
            Some(cat) => cat,
            _ => return,
        },
        _ => return,
    };

    let result: usize = seeds
        .iter()
        .map(|s| main_category.lookup(*s))
        .min()
        .expect("Should always get a result.");

    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let s = include_str!("../inputs/part1.txt");

    let mut line_iter = s.split("\n").into_iter();

    let seed_range_params: Vec<usize> = line_iter
        .next()
        .expect("Expected at least 1 line...")
        .strip_prefix("seeds: ")
        .expect("Should have started with 'seeds: '")
        .split(" ")
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let seeds = seed_range_params
        .chunks(2)
        .map(|params| {
            let start = params[0];
            let width = params[1];

            StdRange {
                start,
                end: start + width,
            }
        })
        .flat_map(|r| r.clone());
    //
    let main_category = match line_iter.next() {
        Some("") => match Category::from_line_iter(&mut line_iter) {
            Some(cat) => cat,
            _ => return,
        },
        _ => return,
    };

    let result: usize = seeds
        .map(|s| main_category.lookup(s))
        .min()
        .expect("Should always get a result.");

    println!("Part 2 Result: {:?}", result);
}

fn main() {
    part2();
}
