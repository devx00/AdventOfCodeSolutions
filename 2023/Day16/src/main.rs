use std::{
    cell::{RefCell, RefMut},
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufReader, Read, Write},
    ops::{Add, Deref, DerefMut, Sub},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

#[derive(Debug)]
enum Optic {
    ForwardMirror,
    BackwardMirror,
    HorizontalSplitter,
    VerticalSplitter,
    Empty,
}

impl From<char> for Optic {
    fn from(value: char) -> Self {
        match value {
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '-' => Self::VerticalSplitter,
            '|' => Self::HorizontalSplitter,
            _ => Self::Empty,
        }
    }
}

impl Optic {
    fn apply_optic(&self, coord_delta: Coordinate) -> Vec<Coordinate> {
        match self {
            Self::Empty => vec![coord_delta],
            Self::ForwardMirror => vec![Coordinate::identity() - coord_delta.swap()],
            Self::BackwardMirror => vec![coord_delta.swap()],
            Self::VerticalSplitter => match coord_delta.as_tuple() {
                (0, _) => vec![coord_delta],
                (_, 0) => vec![Coordinate::new(0, -1), Coordinate::new(0, 1)],
                _ => panic!("This should have matched all possible inputs."),
            },
            Self::HorizontalSplitter => match coord_delta.as_tuple() {
                (_, 0) => vec![coord_delta],
                (0, _) => vec![Coordinate::new(-1, 0), Coordinate::new(1, 0)],
                _ => panic!("This should have matched all possible inputs."),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coordinate {
    row: isize,
    column: isize,
}

impl Coordinate {
    fn identity() -> Self {
        Self { row: 0, column: 0 }
    }
    fn new(row: isize, column: isize) -> Self {
        Self { row, column }
    }

    fn next_straight(self, previous: Coordinate) -> Coordinate {
        let delta = self - previous;
        self + delta
    }

    fn swap(self) -> Coordinate {
        Coordinate::new(self.column, self.row)
    }

    fn as_tuple(self) -> (isize, isize) {
        (self.row, self.column)
    }

    fn as_queued(&self, from: Coordinate) -> QueuedVisit {
        QueuedVisit {
            to_coordinate: self.clone(),
            from_coordinate: from.clone(),
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row - rhs.row,
            column: self.column - rhs.column,
        }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}
#[derive(Debug)]
struct Node {
    optic: Optic,
    index: Coordinate,
    visited_from: HashSet<Coordinate>,
}

impl Node {
    fn new(optic_char: char, index: Coordinate) -> Self {
        let optic = Optic::from(optic_char);
        Node {
            optic,
            index,
            visited_from: HashSet::new(),
        }
    }

    fn visit(&mut self, from_index: Coordinate) -> Vec<QueuedVisit> {
        match self.visited_from.contains(&from_index) {
            true => vec![],
            false => {
                self.visited_from.insert(from_index);
                let delta = self.index - from_index;
                self.optic
                    .apply_optic(delta)
                    .into_iter()
                    .map(|cd| self.index + cd)
                    .map(|cd| cd.as_queued(self.index))
                    .collect::<Vec<QueuedVisit>>()
            }
        }
    }

    fn energized(&self) -> bool {
        self.visited_from.len() > 0
    }

    fn energized_char(&self) -> char {
        match self.energized() {
            true => '#',
            false => '.',
        }
    }

    fn reset(&mut self) {
        self.visited_from.clear();
    }
}
#[derive(Debug)]
struct NodeList(Vec<RefCell<Node>>);
impl Deref for NodeList {
    type Target = Vec<RefCell<Node>>;
    fn deref(&self) -> &Vec<RefCell<Node>> {
        &self.0
    }
}

impl DerefMut for NodeList {
    fn deref_mut(&mut self) -> &mut Vec<RefCell<Node>> {
        &mut self.0
    }
}

impl NodeList {
    fn new(index: usize, row: &str) -> NodeList {
        NodeList(
            row.chars()
                .enumerate()
                .map(|(i, c)| {
                    RefCell::new(Node::new(
                        c,
                        Coordinate::new(index.try_into().unwrap(), i.try_into().unwrap()),
                    ))
                })
                .collect(),
        )
    }

    fn get_column(&self, column: isize) -> Option<&RefCell<Node>> {
        match column.try_into() {
            Ok(ucol) => match self.get::<usize>(ucol) {
                Some(rc_node) => Some(rc_node),
                _ => None,
            },
            Err(_) => None,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct QueuedVisit {
    to_coordinate: Coordinate,
    from_coordinate: Coordinate,
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<NodeList>,
}
impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            nodes: s
                .lines()
                .enumerate()
                .map(|(i, l)| NodeList::new(i, l))
                .collect::<Vec<NodeList>>(),
        })
    }
}

impl Grid {
    fn get_row(&self, row: isize) -> Option<&NodeList> {
        match row.try_into() {
            Ok(urow) => match self.nodes.get::<usize>(urow) {
                Some(nodelist) => Some(nodelist),
                _ => None,
            },
            Err(_) => None,
        }
    }
    fn get_node(&self, coordinate: Coordinate) -> Option<&RefCell<Node>> {
        let nodelist: Option<&NodeList> = self.get_row(coordinate.row);
        match nodelist?.get_column(coordinate.column) {
            Some(rc_node) => Some(rc_node),
            _ => None,
        }
    }

    fn reset(&self) {
        self.nodes
            .iter()
            .for_each(|nl| nl.iter().for_each(|rc_node| rc_node.borrow_mut().reset()));
    }

    fn walk(&self, start_position: Coordinate) {
        let bounds = self.bounds();
        let first_visit = match start_position.as_tuple() {
            (-1, col) => Coordinate::new(0, col),
            (row_bound, col) if row_bound == bounds.row => Coordinate::new(row_bound - 1, col),
            (row, -1) => Coordinate::new(row, 0),
            (row, col_bound) if col_bound == bounds.column => Coordinate::new(row, col_bound - 1),
            _ => {
                println!("{:?}, bounds: {:?}", start_position, bounds.as_tuple());
                panic!("Invalid Start Position")
            }
        };

        let mut queue: VecDeque<QueuedVisit> = VecDeque::new();
        queue.push_back(first_visit.as_queued(start_position));

        while let Some(next_visit) = queue.pop_front() {
            match self.get_node(next_visit.to_coordinate) {
                Some(next_node) => {
                    queue.extend(
                        next_node
                            .borrow_mut()
                            .visit(next_visit.from_coordinate)
                            .into_iter(),
                    );
                }
                _ => (),
            }
        }
    }

    fn bounds(&self) -> Coordinate {
        Coordinate::new(
            self.nodes.len().try_into().unwrap(),
            self.nodes.first().unwrap().len().try_into().unwrap(),
        )
    }

    fn count_energized(&mut self, start_pos: Coordinate) -> usize {
        self.reset();
        self.walk(start_pos);
        self.nodes.iter().fold(0, |acc, nl| {
            acc + nl
                .iter()
                .filter(|rc_node| rc_node.borrow().energized())
                .count()
        })
    }

    fn perimiter_coords(&self) -> Vec<Coordinate> {
        let bounds = self.bounds();
        vec![
            Coordinate::new(0, -1),
            Coordinate::new(-1, 0),
            Coordinate::new(0, bounds.column),
            Coordinate::new(bounds.row, 0),
        ]
        .iter()
        .map(|per_coord| match per_coord.as_tuple() {
            (0, _) => (0..bounds.column)
                .map(|per_row| Coordinate::new(per_row, per_coord.column))
                .collect::<Vec<Coordinate>>(),
            (_, 0) => (0..bounds.row)
                .map(|per_col| Coordinate::new(per_coord.row, per_col))
                .collect::<Vec<Coordinate>>(),
            _ => panic!("Should only match these 2 ones."),
        })
        .flatten()
        .collect()
    }

    fn find_best(&mut self) -> usize {
        self.perimiter_coords()
            .into_iter()
            .map(|start_pos| self.count_energized(start_pos))
            .fold(0, |acc, val| acc.max(val))
    }

    #[allow(dead_code)]
    fn print_energized(&self) {
        let repr = self
            .nodes
            .iter()
            .map(|nl| {
                nl.iter()
                    .map(|rc_node| rc_node.borrow().energized_char())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", repr);
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let mut grid = input
        .parse::<Grid>()
        .expect("Expected to be able to parse the grid...");
    let result = grid.count_energized(Coordinate::new(0, -1));

    println!("Part 1 Result: {:?}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("part1.txt");
    let mut grid = input
        .parse::<Grid>()
        .expect("Expected to be able to parse the grid...");
    let result = grid.find_best();
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
