type Galaxy = (usize, usize);

struct SpaceTracker {
    rows: Vec<bool>,
    cols: Vec<bool>,
}

impl SpaceTracker {
    fn with_rows_cols(rows: usize, cols: usize) -> Self {
        Self {
            rows: (0..rows).map(|_| true).collect(),
            cols: (0..cols).map(|_| true).collect(),
        }
    }

    fn mark_seen(&mut self, row: usize, col: usize) {
        self.rows[row] = false;
        self.cols[col] = false;
    }

    fn process_seen(&mut self, galaxies: &Vec<Galaxy>) {
        for galaxy in galaxies {
            self.mark_seen(galaxy.0, galaxy.1);
        }
    }

    fn adjusted_location(&self, loc: Galaxy, multiplier: usize) -> Galaxy {
        (
            (self.rows.iter().take(loc.0).filter(|x| **x == true).count() * (multiplier - 1))
                + loc.0,
            (self.cols.iter().take(loc.1).filter(|x| **x == true).count() * (multiplier - 1))
                + loc.1,
        )
    }

    fn adjust_galaxies(&mut self, galaxies: &Vec<Galaxy>, multiplier: usize) -> Vec<Galaxy> {
        galaxies
            .iter()
            .map(|g| self.adjusted_location(*g, multiplier))
            .collect()
    }
}

fn distance_between(galaxy1: Galaxy, galaxy2: Galaxy) -> usize {
    galaxy1.0.abs_diff(galaxy2.0) + galaxy1.1.abs_diff(galaxy2.1)
}

fn main() {
    let input = include_str!("../inputs/part1.txt");

    let nrows = input.lines().count();
    let ncols = input.lines().nth(0).unwrap().len();

    let mut st = SpaceTracker::with_rows_cols(nrows, ncols);

    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter_map(move |(j, c)| match c {
                    '#' => Some((i, j)),
                    '.' => None,
                    _ => panic!("Found unexpected character at {},{}", i, j),
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect::<Vec<Galaxy>>();

    st.process_seen(&galaxies);

    //////// Part 1 Solution /////////////
    let adjusted_galaxies = st.adjust_galaxies(&galaxies, 2);

    let result: usize = adjusted_galaxies
        .iter()
        .enumerate()
        .map(|(i, g1)| {
            adjusted_galaxies
                .iter()
                .skip(i + 1)
                .map(|g2| distance_between(*g1, *g2))
                .sum::<usize>()
        })
        .sum();

    println!("Part 1 Result: {}", result);
    ////////End Part 1 Solution /////////////
    //
    //////// Part 2 Solution /////////////
    let adjusted_galaxies_pt2 = st.adjust_galaxies(&galaxies, 1000000);

    let result_pt2: usize = adjusted_galaxies_pt2
        .iter()
        .enumerate()
        .map(|(i, g1)| {
            adjusted_galaxies_pt2
                .iter()
                .skip(i + 1)
                .map(|g2| distance_between(*g1, *g2))
                .sum::<usize>()
        })
        .sum();

    println!("Part 2 Result: {}", result_pt2);
}
