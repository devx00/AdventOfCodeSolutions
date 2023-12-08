fn num_ways_to_win(time: u32, record: u32) -> u32 {
    (0..time).filter(|t| (time - t) * t > record).count() as u32
}

fn main() {
    let input = include_str!("../inputs/part1.txt");
    let times_line = input.lines().nth(0).unwrap();
    let distance_line = input.lines().nth(1).unwrap();
    let times: Vec<u32> = times_line
        .strip_prefix("Time:")
        .expect("Times line not found.")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<u32> = distance_line
        .strip_prefix("Distance:")
        .expect("Distances line not found.")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let answer = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| num_ways_to_win(*t, *d))
        .fold(1, |acc, x| acc * x);

    println!("Times: {:?}", times);
    println!("Distances: {:?}", distances);
    println!("Answer: {}", answer);

    ()
}
