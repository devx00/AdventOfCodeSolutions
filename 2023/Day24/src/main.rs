use itertools::{self, Itertools};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
    ops::{RangeBounds, RangeInclusive},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct HashablePosition {
    x: (u64, u64),
    y: (u64, u64),
    z: (u64, u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}
impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .trim()
            .split(", ")
            .map(|v| v.trim().parse::<f64>().expect("Position component"))
            .into_iter()
            .collect::<Vec<_>>()
            .as_slice()
        {
            [x, y, z] => Ok(Position {
                x: *x,
                y: *y,
                z: *z,
            }),
            _ => Err(()),
        }
    }
}

impl Position {
    fn hashable_position(&self) -> HashablePosition {
        HashablePosition {
            x: (self.x as u64, (self.x.fract() * 1000.0) as u64),
            y: (self.y as u64, (self.y.fract() * 1000.0) as u64),
            z: (self.z as u64, (self.z.fract() * 1000.0) as u64),
        }
    }
    fn offset_2d(&self, delta_x: f64, delta_y: f64) -> Position {
        Position {
            x: self.x + delta_x,
            y: self.y + delta_y,
            z: self.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    delta_x: f64,
    delta_y: f64,
    delta_z: f64,
}

impl FromStr for Velocity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .trim()
            .split(", ")
            .map(|v| v.trim().parse::<f64>().expect("Velocity component"))
            .into_iter()
            .collect::<Vec<_>>()
            .as_slice()
        {
            [dx, dy, dz] => Ok(Velocity {
                delta_x: *dx,
                delta_y: *dy,
                delta_z: *dz,
            }),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Colinear,
    Clockwise,
    Counterclockwise,
}

impl Orientation {
    fn for_points(p1: &Position, p2: &Position, p3: &Position) -> Orientation {
        match ((p2.y - p1.y) * (p3.x - p2.x)) - ((p2.x - p1.x) * (p3.y - p2.y)) {
            val if val > 0.0 => Orientation::Clockwise,
            val if val < 0.0 => Orientation::Counterclockwise,
            _ => Orientation::Colinear,
        }
    }
}

#[derive(Debug, Clone)]
struct PathSegment {
    start: Position,
    end: Position,
}

impl PathSegment {
    fn from(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    fn intersects(&self, other: &PathSegment) -> bool {
        let o1 = Orientation::for_points(&self.start, &self.end, &other.start);
        let o2 = Orientation::for_points(&self.start, &self.end, &other.end);
        let o3 = Orientation::for_points(&other.start, &other.end, &self.start);
        let o4 = Orientation::for_points(&other.start, &other.end, &self.end);

        if o1 != o2 && o3 != o4 {
            return true;
        }
        match (o1, o2, o3, o4) {
            (Orientation::Colinear, _, _, _) => self.contains(&other.start),
            (_, Orientation::Colinear, _, _) => self.contains(&other.end),
            (_, _, Orientation::Colinear, _) => other.contains(&self.start),
            (_, _, _, Orientation::Colinear) => other.contains(&self.end),
            _ => false,
        }
    }

    fn contains(&self, point: &Position) -> bool {
        point.x <= self.start.x.max(self.end.x)
            && point.x >= self.start.x.min(self.end.x)
            && point.y <= self.start.y.max(self.end.y)
            && point.y >= self.start.y.min(self.end.y)
    }
}

#[derive(Debug, Clone)]
struct HailStone {
    position: Position,
    velocity: Velocity,
}

impl HailStone {
    fn time_to_x(&self, x: f64) -> Option<f64> {
        match self.velocity.delta_x {
            dx if dx == 0.0 => match self.position.x {
                current_x if current_x == x => Some(0.0),
                _ => None,
            },
            _ => Some((x - self.position.x) / self.velocity.delta_x),
        }
    }

    fn time_to_y(&self, y: f64) -> Option<f64> {
        match self.velocity.delta_y {
            dy if dy == 0.0 => match self.position.y {
                current_y if current_y == y => Some(0.0),
                _ => None,
            },
            _ => Some((y - self.position.y) / self.velocity.delta_y),
        }
    }

    fn position_at_time(&self, time: f64) -> Position {
        self.position
            .offset_2d(self.velocity.delta_x * time, self.velocity.delta_y * time)
    }

    fn position_when_x(&self, x: f64) -> Option<Position> {
        if self.velocity.delta_x == 0.0 {
            return match x == self.position.x {
                true => Some(self.position),
                false => None
            };
        }

        let m = self.velocity.delta_y / self.velocity.delta_x;
        let b = self.position.y - (m * self.position.x);
        let y = (m * x) + b;

        Some(Position{ x, y, z: self.position.z})
    }

    fn position_when_y(&self, y: f64) -> Option<Position> {
        if self.velocity.delta_x == 0.0 {
            return match self.velocity.delta_y {
                dy if dy == 0.0 => None,
                _ => Some(Position{x: self.position.x, y, z:self.position.z})
            };
        }

        let m = self.velocity.delta_y / self.velocity.delta_x;
        let b = self.position.y - (m * self.position.x);
        if m != 0.0 {
            let x = (y - b) / m;
            Some(Position{ x, y, z: self.position.z})
        } else {
            match y == self.position.y {
                true => Some(self.position),
                false => None
            }
        }
    }

    fn time_to_position(&self, position: &Position) -> Option<f64> {
        match (self.time_to_x(position.x), self.time_to_y(position.y)) {
            (Some(t1), Some(t2)) if t1.trunc() == t2.trunc() => Some(t1),
            (Some(t1), Some(t2)) => {
                println!("Same?: ({} rounded: {}) == ({} rounded: {}), {:?}, {:?}",t1,  (t1 * 100.0).trunc(), t2, (t2 * 100.0).trunc(), self.position_at_time(t1), self.position_at_time(t2));
                None
            }
            _ => None,
        }
    }

    fn path_in_window(&self, window: RangeInclusive<isize>) -> Option<PathSegment> {
        let (start, end) = (*window.start() as f64, *window.end() as f64);
        let window_verts = [
            Position {
                x: start,
                y: start,
                z: self.position.z,
            }, // top left
            Position {
                x: end,
                y: start,
                z: self.position.z,
            }, // top right
            Position {
                x: end,
                y: end,
                z: self.position.z,
            }, // borrom right
            Position {
                x: start,
                y: end,
                z: self.position.z,
            }, // bottom left
        ];
        let top = PathSegment::from(window_verts[0].clone(), window_verts[1].clone());
        let bottom = PathSegment::from(window_verts[3].clone(), window_verts[2].clone());
        let left = PathSegment::from(window_verts[0].clone(), window_verts[3].clone());
        let right = PathSegment::from(window_verts[1].clone(), window_verts[2].clone());

        let possible_intersections = [
            self.position_when_x(start),
            self.position_when_x(end),
            self.position_when_y(start),
            self.position_when_y(end),
        ];
        let intersections = possible_intersections
            .iter()
            .filter_map(|opos| match opos {
                Some(position) => {
                    // println!(
                    //     "Possible Intersections For: {:?} <Velocity: {:?}>: {:?}",
                    //     self.position, self.velocity, position
                    // );
                    if [&top, &bottom, &left, &right]
                        .iter()
                        .find(|bound| bound.contains(position))
                        .is_some()
                    {
                        Some(position)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .unique_by(|pos| pos.hashable_position())
            .collect::<Vec<_>>();

        match intersections.into_iter().collect::<Vec<_>>().as_slice() {
            [p1, p2] => match (self.time_to_position(*p1), self.time_to_position(*p2)) {
                (Some(t1), Some(t2)) if t1 >= 0.0 && t2 >= 0.0 => {
                    Some(PathSegment::from(**p1, **p2))
                }
                (Some(t1), Some(t2)) if t1 >= 0.0 && t2 < 0.0 => {
                    Some(PathSegment::from(self.position.clone(), **p1))
                }
                (Some(t1), Some(t2)) if t1 < 0.0 && t2 >= 0.0 => {
                    Some(PathSegment::from(self.position.clone(), **p2))
                }
                (Some(t1), Some(t2)) if t1 < 0.0 && t2 < 0.0 => None,
                vals => {
                    println!("Hmm... {:?}, ({:?}, {:?})", vals, p1, p2);
                    panic!("Should always fall into one of the previous cases.");
                }
            },
            [p1] => Some(PathSegment::from(**p1, **p1)),
            [] => None,
            other => {
                println!("Other: {:?}", other);
                panic!("Should only have 1, 2, or 0 unique elements here.")
            },
        }
    }
}

impl FromStr for HailStone {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" @ ") {
            Some((pos_str, velo_str)) => Ok(HailStone {
                position: pos_str.parse::<Position>().expect("Position"),
                velocity: velo_str.parse::<Velocity>().expect("Velocity"),
            }),
            _ => Err(()),
        }
    }
}

#[allow(dead_code)]
fn part1() {
    // let (input, window) = (load_input("example1.txt"), 7..=27);
    let (input, window) = (load_input("part1.txt"), 200000000000000..=400000000000000);
    let result = input
        .lines()
        .map(|l| l.parse::<HailStone>().expect("Hailstone"))
        .filter_map(|hs| {
            let path = hs.path_in_window(window.clone());
            println!("Window Path: {:?}", path);
            path
        })
        .combinations(2)
        .filter(|paths| match paths.as_slice() {
            [p1, p2] => p1.intersects(p2),
            _ => panic!("Should always have 2 paths."),
        })
        .count();

    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("example1.txt");
    let result = input
        .lines()
        .map(|l| l.parse::<HailStone>().expect("Hailstone"))
        .collect::<Vec<_>>();
    println!("Part 2 Result: {:?}", result);
}

fn main() {
    part1();
    // part2();
}
