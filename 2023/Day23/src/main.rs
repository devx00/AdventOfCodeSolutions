use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
    ops::Index,
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::Slope(Direction::Up),
            '>' => Self::Slope(Direction::Right),
            '<' => Self::Slope(Direction::Left),
            'v' => Self::Slope(Direction::Down),
            _ => panic!("Not a valid map entity!"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct MapIndex(usize, usize);

impl PartialEq for MapIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl MapIndex {
    fn adjacent(&self, direction: Direction) -> Option<MapIndex> {
        match direction {
            Direction::Up => match self.0.checked_sub(1) {
                Some(new_row) => Some(MapIndex(new_row, self.1)),
                _ => None,
            },
            Direction::Down => match self.0.checked_add(1) {
                Some(new_row) => Some(MapIndex(new_row, self.1)),
                _ => None,
            },
            Direction::Left => match self.1.checked_sub(1) {
                Some(new_col) => Some(MapIndex(self.0, new_col)),
                _ => None,
            },
            Direction::Right => match self.1.checked_add(1) {
                Some(new_col) => Some(MapIndex(self.0, new_col)),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Node {
    tile: Tile,
    index: MapIndex,
}

impl Node {
    fn new(tile: Tile, row: usize, column: usize) -> Self {
        Self {
            tile,
            index: MapIndex(row, column),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

#[derive(Debug)]
enum SolutionType {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Node>>,
    solution: SolutionType,
}

impl Index<MapIndex> for Map {
    type Output = Node;
    fn index(&self, index: MapIndex) -> &Self::Output {
        &self.tiles[index.0][index.1]
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            tiles: s
                .lines()
                .enumerate()
                .map(|(row, l)| {
                    l.chars()
                        .enumerate()
                        .map(|(col, c)| Node::new(Tile::from(c), row, col))
                        .collect::<Vec<Node>>()
                })
                .collect::<Vec<Vec<Node>>>(),
            solution: SolutionType::Part1,
        })
    }
}

impl Map {
    fn get_node(&self, index: &MapIndex) -> Option<&Node> {
        match self.tiles.get(index.0) {
            Some(row) => row.get(index.1),
            None => None,
        }
    }

    fn adjacent_path_nodes(&self, index: &MapIndex) -> Vec<MapIndex> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter_map(|dir| match index.adjacent(dir) {
            Some(index) if index.0 < self.tiles.len() && index.1 < self.tiles[0].len() => {
                Some(index)
            }
            _ => None,
        })
        .filter(|index| match self.get_node(index) {
            Some(node) => match node.tile {
                Tile::Path | Tile::Slope(_) => true,
                _ => false,
            },
            _ => false,
        })
        .collect()
    }

    fn possible_steps(&self, index: &MapIndex) -> Vec<MapIndex> {
        match self.get_node(index) {
            Some(node) => match node.tile {
                Tile::Path => self.adjacent_path_nodes(index),
                Tile::Slope(direction) => match self.solution {
                    SolutionType::Part1 => {
                        vec![index
                            .adjacent(direction)
                            .expect("Slope to point to a valid tile.")]
                    }
                    SolutionType::Part2 => self.adjacent_path_nodes(index),
                },
                _ => vec![],
            },
            _ => vec![],
        }
    }

    fn starting_position(&self) -> MapIndex {
        let (start_col, _) = self.tiles[0]
            .iter()
            .enumerate()
            .find(|(_, node)| node.tile == Tile::Path)
            .expect("Starting Tile");

        MapIndex(0, start_col)
    }
    fn ending_position(&self) -> MapIndex {
        let (end_col, _) = self
            .tiles
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .find(|(_, node)| node.tile == Tile::Path)
            .expect("Ending Tile");

        MapIndex(self.tiles.len() - 1, end_col)
    }

    fn print_path(&self, path: &Path) {
        println!(
            "{}",
            self.tiles
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|node| match path.steps.contains(node) {
                            true => 'O',
                            _ => match node.tile {
                                Tile::Path => '.',
                                Tile::Slope(dir) => match dir {
                                    Direction::Right => '>',
                                    Direction::Left => '<',
                                    Direction::Down => 'v',
                                    Direction::Up => '^',
                                },
                                Tile::Forest => '#',
                            },
                        })
                        .collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    fn next_node_candidates(&self, current_path: &Path) -> Vec<MapIndex> {
        self.possible_steps(&current_path.last_node.index)
            .iter()
            .filter_map(|idx| self.get_node(idx))
            .filter(|node| !current_path.steps.contains(node))
            .map(|node| node.index)
            .collect::<Vec<_>>()
    }

    fn farthest_hike<'map>(&self, current_path: &mut Path, end_index: &MapIndex) -> usize {
        let mut next_node_candidates = self.next_node_candidates(&current_path);
        while next_node_candidates.len() <= 1 {
            if next_node_candidates.len() == 0 {
                return 0;
            }

            current_path.add_node(self[*next_node_candidates.first().unwrap()]);

            if next_node_candidates.contains(&end_index) {
                return current_path.len();
            }
            next_node_candidates = self.next_node_candidates(&current_path);
        }
        if next_node_candidates.contains(&end_index) {
            return current_path.len();
        }

        next_node_candidates
            .into_iter()
            .map(|node_idx| {
                let next_node = self[node_idx];
                let mut next_path = current_path.clone();
                next_path.add_node(next_node);
                self.farthest_hike(&mut next_path, end_index)
            })
            .max()
            .unwrap()
    }

    fn find_farthest_hike<'map>(&self) -> usize {
        let start_node = self[self.starting_position()];
        let ending_position = self.ending_position();
        let mut path = Path::new(start_node);
        self.farthest_hike(&mut path, &ending_position)
    }
}

#[derive(Debug, Clone)]
struct Path {
    last_node: Node,
    steps: HashSet<Node>,
}

impl Path {
    fn new(start_node: Node) -> Path {
        Path {
            steps: HashSet::new(),
            last_node: start_node,
        }
    }
    fn len(&self) -> usize {
        self.steps.len()
    }

    fn add_node(&mut self, node: Node) {
        self.steps.insert(node);
        self.last_node = node;
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let map = input.parse::<Map>().unwrap();
    let result = map.find_farthest_hike();

    println!("Part 1 Result: {}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("part1.txt");
    let mut map = input.parse::<Map>().unwrap();
    map.solution = SolutionType::Part2;
    let result = map.find_farthest_hike();
    println!("Longest: {:?}", result);
    println!("Part 2 Result: {}", result);
}

fn main() {
    // part1();
    part2();
}
