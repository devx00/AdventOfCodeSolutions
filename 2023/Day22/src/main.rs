use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd)]
struct Point3d {
    x: u32,
    y: u32,
    z: u32,
}

impl Ord for Point3d {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x
            .cmp(&other.x)
            .then(self.y.cmp(&other.y).then(self.z.cmp(&other.z)))
    }
}

impl PartialEq for Point3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point3d {
    fn sub_z(&self, diff_z: u32) -> Point3d {
        Point3d {
            x: self.x,
            y: self.y,
            z: self.z - diff_z,
        }
    }
}

impl FromStr for Point3d {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split(",")
            .into_iter()
            .map(|v| {
                v.parse::<u32>()
                    .expect("Expected to be able to parse point as a u32")
            })
            .collect::<Vec<u32>>()
            .as_slice()
        {
            [x, y, z] => Ok(Point3d {
                x: *x,
                y: *y,
                z: *z,
            }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Brick {
    bottom_corner: Point3d,
    top_corner: Point3d,
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        self.bottom_corner == other.bottom_corner && self.top_corner == other.top_corner
    }
}

impl FromStr for Brick {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split("~")
            .into_iter()
            .map(|p| {
                p.parse::<Point3d>()
                    .expect("Expected to be able to parse brick as 2 points.")
            })
            .collect::<Vec<Point3d>>()
            .as_slice()
        {
            [p1, p2] => Ok(Brick {
                bottom_corner: *p1,
                top_corner: *p2,
            }),
            _ => Err(()),
        }
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.top_corner
            .cmp(&other.top_corner)
            .then(self.bottom_corner.cmp(&other.bottom_corner))
    }
}

impl Brick {
    fn xy_points(&self) -> Vec<(u32, u32)> {
        (self.min_x()..=self.max_x())
            .map(|x| {
                (self.min_y()..=self.max_y())
                    .map(|y| (x, y))
                    .collect::<Vec<(u32, u32)>>()
            })
            .flatten()
            .collect::<Vec<(u32, u32)>>()
    }
    fn min_z(&self) -> u32 {
        self.bottom_corner.z
    }

    fn max_z(&self) -> u32 {
        self.top_corner.z
    }
    fn min_y(&self) -> u32 {
        self.bottom_corner.y.min(self.top_corner.y)
    }

    fn max_y(&self) -> u32 {
        self.bottom_corner.y.max(self.top_corner.y)
    }
    fn min_x(&self) -> u32 {
        self.bottom_corner.x.min(self.top_corner.x)
    }

    fn max_x(&self) -> u32 {
        self.bottom_corner.x.max(self.top_corner.x)
    }

    fn rebased(&self, z: u32) -> Brick {
        let diff = self.bottom_corner.z - z;
        Brick {
            top_corner: self.top_corner.sub_z(diff),
            bottom_corner: self.bottom_corner.sub_z(diff),
        }
    }
    fn intersects_x(&self, other: &Brick) -> bool {
        let x_range = self.min_x()..=self.max_x();

        x_range.contains(&other.min_x()) || x_range.contains(&other.max_x())
    }
    fn intersects_y(&self, other: &Brick) -> bool {
        let y_range = self.min_y()..=self.max_y();

        y_range.contains(&other.min_y()) || y_range.contains(&other.max_y())
    }

    fn intersects(&self, other: &Brick) -> bool {
        self.intersects_x(other) && self.intersects_y(other)
            || other.intersects_x(&self) && other.intersects_y(&self)
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{} -> {},{},{}",
            self.bottom_corner.x,
            self.bottom_corner.y,
            self.bottom_corner.z,
            self.top_corner.x,
            self.top_corner.y,
            self.top_corner.z,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BrickBoard {
    bricks: Vec<Brick>,
    positions: HashMap<(u32, u32, u32), Brick>,
}

impl FromStr for BrickBoard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bricks: Vec<Brick> = s
            .lines()
            .map(|l| {
                l.parse::<Brick>()
                    .expect("Expected to be able to parse a brick from the line.")
            })
            .collect();
        Ok(BrickBoard::new(&mut bricks))
    }
}

fn print_tops(tops: &HashMap<(u32, u32), u32>) {
    let out = (0..=10)
        .map(|y| {
            (0..=10)
                .map(|x| tops.get(&(x, y)).unwrap_or(&0))
                .map(|height| format!("{:^5}", height))
                .collect::<Vec<String>>()
                .join("|")
            // .collect()
        })
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", out);
    // .collect::<String>();
}

impl BrickBoard {
    fn new(bricks: &mut Vec<Brick>) -> Self {
        // let mut brickset: BTreeSet<Brick> = BTreeSet::new();
        let mut brickset: Vec<Brick> = vec![];
        println!("Bricks: {:?}", bricks);
        bricks.sort_by(|a, b| a.max_z().cmp(&b.max_z()));
        let mut tops: HashMap<(u32, u32), u32> = HashMap::new();
        let mut positions: HashMap<(u32, u32, u32), Brick> = HashMap::new();
        for brick in bricks.iter_mut() {
            let xy_footprint = brick.xy_points();
            let z_base = xy_footprint
                .iter()
                .map(|pos| tops.entry(*pos).or_insert(0).clone())
                .max()
                .expect("Should always have a val here.");

            // println!("Brick: {:?}", brick);
            // let intersected = brickset
            //     .iter()
            //     .filter(|br| {
            //         let int = br.intersects(&brick);
            //         let brlessbrick = br.max_z() < brick.min_z();
            //         // println!(
            //         //     "intersects {}  && {:?} max_z < min_z: {}",
            //         //     int, br, brlessbrick
            //         // );
            //         int && brlessbrick
            //     })
            //     .map(|br| br.max_z())
            //     .reduce(|acc, z| acc.max(z));
            let new_brick = brick.rebased(z_base + 1u32);
            println!("Brick: {} -> New Brick: {}", brick, new_brick);
            let new_min_z = new_brick.min_z().clone();
            let new_max_z = new_brick.max_z().clone();

            brickset.push(new_brick);
            xy_footprint.iter().for_each(|pos| {
                tops.entry(*pos).and_modify(|v| *v = new_max_z);
                (new_min_z..=new_max_z).for_each(|z| {
                    positions.insert((pos.0, pos.1, z), new_brick.clone());
                });
            });
            println!("Tops: {:?}", tops);
            print_tops(&tops);
            // // println!("Brick: {:?}\n", new_brick);
        }
        // println!("New BrickBoard: {:?}", brickset);
        BrickBoard {
            bricks: brickset,
            positions,
        }
    }

    fn what_supports(&self, brick: Brick) -> Vec<Brick> {
        brick
            .xy_points()
            .into_iter()
            .map(|(x, y)| self.positions[&(x, y, brick.min_z() - 1)])
            .collect::<Vec<Brick>>()
    }

    fn this_supports_what(&self, brick: Brick) -> Vec<Brick> {
        brick
            .xy_points()
            .into_iter()
            .map(|(x, y)| self.positions[&(x, y, brick.max_z() + 1)])
            .collect::<Vec<Brick>>()
    }

    fn supporting_brick(&self, brick: &Brick) -> Option<Brick> {
        let supports = self.bricks_at_z(brick.min_z() - 1, |br| br.intersects(brick));

        // let supports = self
        //     .bricks
        //     .iter()
        //     .filter(|br| br.max_z() == brick.min_z() - 1 && br.intersects(brick))
        //     .collect::<Vec<&Brick>>();

        for b in supports.iter() {
            println!("{} supports {}", b, brick);
        }

        match supports.as_slice() {
            [s] => {
                println!("Sole Support: {} for {}", s, brick);
                Some(**s)
            }
            _ => None,
        }
    }

    fn count_removable(&self) -> usize {
        self.bricks.iter().filter(|brick| {
            self.this_supports_what(brick).into_iter().map(|brick_s| {
                self.what_supports(brick_s)
                    .iter()
                    .all(|brick_s_s| *brick_s_s == brick)
            })
        })
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    // let input = load_input("part1.txt");
    let board = input
        .parse::<BrickBoard>()
        .expect("Expected to parse all the bricks...");
    // let bricks_dup = board.bricks.clone();
    // board.settle_bricks();
    // board
    //     .bricks
    //     .iter()
    //     .zip(bricks_dup.iter())
    //     .for_each(|(b1, b2)| println!("{} {}", b1, b2));

    let result = board.count_removable();
    println!("Part 1 Result: {:?}", result);
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
