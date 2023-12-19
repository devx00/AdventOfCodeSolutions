use std::{
    fs::File,
    io::{BufReader, Read},
    ops::{Deref, DerefMut},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Clone, Hash)]
enum Operation {
    Store(String, u8),
    Remove(String),
}

fn hash_func(initial_state: u32, val: String) -> u32 {
    val.to_string()
        .as_bytes()
        .iter()
        .fold(initial_state, |acc, c| {
            let sum = acc + *c as u32;
            let product = sum * 17;
            let moded = product % 256;
            moded
        })
}

impl Operation {
    fn hash(&self, initial_state: u32) -> u32 {
        hash_func(initial_state, self.to_string())
    }
}

impl ToString for Operation {
    fn to_string(&self) -> String {
        match self {
            Self::Store(label, focal_len) => format!("{}={}", label, focal_len),
            Self::Remove(label) => format!("{}-", label),
        }
    }
}

impl FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.ends_with("-") {
            true => Ok(Self::Remove(s.strip_suffix("-").unwrap().to_string())),
            false => match s.split_once("=") {
                Some((label, focal_len_str)) => match focal_len_str.parse::<u8>() {
                    Ok(focal_len) => Ok(Self::Store(label.to_string(), focal_len)),
                    _ => Err(()),
                },
                _ => Err(()),
            },
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Lens {
    label: String,
    focal_len: u8,
}
impl Lens {
    fn new(label: String, focal_len: u8) -> Lens {
        Lens { label, focal_len }
    }
}
#[derive(Debug, Clone, Hash)]
struct LensBox(Vec<Lens>);

impl LensBox {
    fn new() -> LensBox {
        LensBox(Vec::new())
    }

    fn add_or_update_lens(&mut self, label: String, focal_len: u8) {
        match self.iter_mut().find(|lens| lens.label == label) {
            Some(lens) => lens.focal_len = focal_len,
            None => self.push(Lens::new(label, focal_len)),
        }
    }

    fn remove_lens(&mut self, label: String) {
        match self
            .iter()
            .enumerate()
            .find_map(|(i, lens)| if lens.label == label { Some(i) } else { None })
        {
            Some(index) => _ = self.remove(index),
            None => (),
        }
    }
}

impl Deref for LensBox {
    type Target = Vec<Lens>;
    fn deref(&self) -> &Vec<Lens> {
        &self.0
    }
}

impl DerefMut for LensBox {
    fn deref_mut(&mut self) -> &mut Vec<Lens> {
        &mut self.0
    }
}

#[derive(Debug, Clone, Hash)]
struct LensStore {
    boxes: [LensBox; 256],
}

impl LensStore {
    fn new() -> LensStore {
        LensStore {
            boxes: (0..256)
                .map(|_| LensBox::new())
                .into_iter()
                .collect::<Vec<LensBox>>()
                .try_into()
                .expect("Should be able to create array here..."),
        }
    }

    fn index_for(label: String) -> usize {
        hash_func(0, label)
            .try_into()
            .expect("Should always be able within usize range")
    }

    fn perform(&mut self, operation: Operation) {
        match operation {
            Operation::Store(label, focal_len) => self
                .boxes
                .get_mut(Self::index_for(label.clone()))
                .unwrap()
                .add_or_update_lens(label, focal_len),
            Operation::Remove(label) => self
                .boxes
                .get_mut(Self::index_for(label.clone()))
                .unwrap()
                .remove_lens(label),
        }
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .filter(|(_, lb)| lb.len() > 0)
            .fold(0, |acc, (i, lb)| {
                lb.iter().enumerate().fold(acc, |lens_acc, (j, lens)| {
                    lens_acc + ((i + 1) * (j + 1) * lens.focal_len as usize)
                })
            })
    }
}

struct StorageState {
    operations: Vec<Operation>,
    store: LensStore,
}

impl FromStr for StorageState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StorageState {
            operations: s
                .trim()
                .split(",")
                .map(|s| {
                    s.parse::<Operation>()
                        .expect("Should be able to parse the operation.")
                })
                .collect::<Vec<Operation>>(),
            store: LensStore::new(),
        })
    }
}

impl StorageState {
    fn hash(&self) -> u32 {
        self.operations
            .iter()
            .fold(0u32, |step_acc, op| op.hash(step_acc))
    }

    fn run(&mut self) -> usize {
        self.operations
            .iter()
            .for_each(|op| self.store.perform(op.clone()));
        self.store.focusing_power()
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let result = input
        .parse::<StorageState>()
        .expect("Should have been able to parse input")
        .hash();

    println!("Part 1 Result: {}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("part1.txt");
    let result = input
        .parse::<StorageState>()
        .expect("Should have been able to parse input")
        .run();
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
