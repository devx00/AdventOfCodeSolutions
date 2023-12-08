fn num_ways_to_win(time: u64, record: u64) -> u64 {
    (0..time).filter(|t| (time - t) * t > record).count() as u64
}

fn main() {
    let input = include_str!("../inputs/part2.txt");
    let times_line = input.lines().nth(0).unwrap();
    let distance_line = input.lines().nth(1).unwrap();
    let time = times_line
        .strip_prefix("Time:")
        .expect("Times line not found.")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let distance = distance_line
        .strip_prefix("Distance:")
        .expect("Distances line not found.")
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let answer = num_ways_to_win(time, distance);

    println!("Times: {:?}", time);
    println!("Distances: {:?}", distance);
    println!("Answer: {}", answer);

    ()
}
