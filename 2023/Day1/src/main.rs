use regex::Regex;

fn load_input() -> String {
    include_str!("../inputs/example2.txt").to_string()
}

fn part1() {
    let input = load_input();
    let answer: u32 = input
        .trim()
        .split("\n")
        .map(|l| {
            let digits: Vec<char> = l
                .chars()
                .filter(|c| match c {
                    x if x.is_ascii_digit() => true,
                    _ => false,
                })
                .collect();

            vec![
                digits.first().expect("Expected at least 1 digit"),
                digits.last().expect("Expected at least 1 digit."),
            ]
            .iter()
            .cloned()
            .collect::<String>()
            .parse::<u32>()
            .expect("Failed to parse number")
        })
        .sum();
    println!("Part 1 Solution: {}", answer);
}
fn part2() {
    let input = load_input();
    let answer: u64 = input
        .trim()
        .split("\n")
        .map(|l| {
            let pat = Regex::new(
                r#"([0-9])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)|(zero)"#,
            )
            .unwrap();
            let caps = pat
                .captures_iter(l)
                .map(|c| c.extract())
                .map(|(_, [c])| c)
                .collect::<Vec<_>>();

            let first = *caps.first().expect("Should have a match.");
            let last = *caps.last().expect("Should have a match");

            let res: u64 = vec![first, last]
                .iter()
                .map(|p| match p {
                    &"0" => "0",
                    &"1" | &"one" => "1",
                    &"2" | &"two" => "2",
                    &"3" | &"three" => "3",
                    &"4" | &"four" => "4",
                    &"5" | &"five" => "5",
                    &"6" | &"six" => "6",
                    &"7" | &"seven" => "7",
                    &"8" | &"eight" => "8",
                    &"9" | &"nine" => "9",
                    _ => panic!("Got non number digit."),
                })
                .map(|v| {
                    // println!("Joined: {}\n\n", v);
                    v
                })
                .collect::<String>()
                .parse::<u64>()
                .unwrap();

            println!("{}|{} = {}\t\t\t{}", first, last, res, l);
            res
        })
        .sum();
    println!("Part 2 Solution: {}", answer);
}
fn main() {
    // part1();
    part2();
}
