use std::cmp::max;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl FromStr for CubeSet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CubeSet {
            red: match s.split(", ").find(|e| e.ends_with("red")) {
                Some(red) => red
                    .strip_suffix(" red")
                    .ok_or(())?
                    .parse::<i32>()
                    .expect("Received a malformed Red value."),
                _ => 0,
            },
            green: match s.split(", ").find(|e| e.ends_with("green")) {
                Some(green) => green
                    .strip_suffix(" green")
                    .ok_or(())?
                    .parse::<i32>()
                    .expect("Received a malformed Green value."),
                _ => 0,
            },
            blue: match s.split(", ").find(|e| e.ends_with("blue")) {
                Some(blue) => blue
                    .strip_suffix(" blue")
                    .ok_or(())?
                    .parse::<i32>()
                    .expect("Received a malformed Blue value."),
                _ => 0,
            },
        })
    }
}

impl CubeSet {
    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }

    fn update_minimums(&mut self, other: &CubeSet) {
        self.red = max(self.red, other.red);
        self.green = max(self.green, other.green);
        self.blue = max(self.blue, other.blue);
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    revelations: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Game, ()> {
        match s.split_once(":") {
            Some((game, rest)) => {
                let id = game
                    .strip_prefix("Game ")
                    .expect("Trying to parse a malformed game")
                    .parse::<i32>()
                    .expect("Couldnt parse malformed game id");
                let revelations: Vec<CubeSet> = rest
                    .trim()
                    .split(";")
                    .map(|cs| match cs.trim().parse::<CubeSet>() {
                        Ok(cubeset) => cubeset,
                        Err(_) => {
                            println!("Failed to parse cubeset: {:?}", cs);
                            panic!("Failed to parse cubeset");
                        }
                    })
                    .collect();

                Ok(Game { id, revelations })
            }
            _ => panic!("Tried to parse a malformed game."),
        }
    }
}

impl Game {
    fn possible_with(&self, cubes: &CubeSet) -> bool {
        self.revelations
            .iter()
            .all(|cs| cubes.red >= cs.red && cubes.green >= cs.green && cubes.blue >= cs.blue)
    }

    fn min_possible(&self) -> CubeSet {
        let mut min_set = CubeSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        self.revelations
            .iter()
            .for_each(|cs| min_set.update_minimums(cs));

        min_set
    }
}

fn load_games() -> Vec<Game> {
    let input = include_str!("../inputs/part1.txt");
    input
        .trim()
        .split("\n")
        .map(|l| l.parse::<Game>())
        .filter_map(|g| g.ok())
        .collect()
}

fn part1() {
    let answer: i32 = load_games()
        .iter()
        .filter(|g| {
            g.possible_with(&CubeSet {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|g| g.id)
        .sum();

    println!("Part 1 Answer: {}", answer);
}

fn part2() {
    let answer: i32 = load_games().iter().map(|g| g.min_possible().power()).sum();

    println!("Part 2 Answer: {}", answer);
}

fn main() {
    part1();
    part2();
}
