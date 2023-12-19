use std::{
    fs::File,
    io::{BufReader, Read},
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("example1.txt");
    let result = input;

    println!("Part 1 Result: {}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("example1.txt");
    let result = input;
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    // part2();
}
