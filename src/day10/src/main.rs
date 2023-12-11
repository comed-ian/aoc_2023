use utils::read_lines;
use std::fmt;

pub trait RemoveFirst<T> {
    fn remove_first(&mut self) -> Option<T>;
}

impl<T> RemoveFirst<T> for Vec<T> {
    fn remove_first(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.remove(0))
    }
}

enum Direction {
    S,
    N,
    E,
    W
}

#[derive(Debug)]
enum Pipe {
    NS,
    EW,
    SE,
    SW,
    NE,
    NW,
    NULL,
    Start,
    Outside,
    Inside
}

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'F' => Self::SE,
            '7' => Self::SW,
            'L' => Self::NE,
            'J' => Self::NW,
            '.' => Self::NULL,
            'S' => Self::Start,
            _ => panic!("Error parsing char {c}"),
        }
    }
}

impl Pipe {
    fn borders(&self, d: Direction) -> bool {
        match self {
            Self::NS => {
                match d {
                    Direction::S | Direction:: N => true,
                    _ => false,
                }
            },
            Self::EW => {
                match d {
                    Direction::E | Direction:: W => true,
                    _ => false,
                }
            },
            Self::SE => {
                match d {
                    Direction::N | Direction:: W => true,
                    _ => false,
                }
            },
            Self::SW => {
                match d {
                    Direction::N | Direction:: E => true,
                    _ => false,
                }
            },
            Self::NE => {
                match d {
                    Direction::S | Direction:: W => true,
                    _ => false,
                }
            },
            Self::NW => {
                match d {
                    Direction::S | Direction:: E => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    direction: Pipe,
    distance: Option<u64>,
    x: usize,
    y: usize,
    connections: Vec<(usize, usize)>,
}

impl Coordinate {
    fn new(symbol: char, x: usize, y: usize) -> Self {
        Self { direction: Pipe::from(symbol), x, y, distance: None,
               connections: vec![] }
    }
}

struct Grid {
    coordinates: Vec<Vec<Coordinate>>,
}

impl Grid {
    fn new(data: &Vec<String>) -> (Self, (usize, usize)) {
        let mut cols: Vec<Coordinate> = vec![];
        let mut rows: Vec<Vec<Coordinate>> = vec![];
        let mut start: Option<(usize, usize)> = None;
        for (i, x) in data.iter().enumerate() {
            for (j, y) in x.chars().enumerate() {
                cols.push( Coordinate::new(y, j, i));
                if y == 'S' { start = Some((j, i)) }
            }
            rows.push(cols);
            cols = vec![];
        }
        if start.is_none() { panic!("Failed to find start!"); }
        (Self { coordinates: rows }, start.unwrap())
    }
    fn set_distance(&mut self, x: usize, y: usize, distance: u64) {
        let mut c = &mut self.coordinates[y][x];
        c.distance = Some(distance);
    }
    fn populate(&mut self, x: usize, y: usize) -> u64 {
        self.set_distance(x, y, 0);
        let mut v = self.get_borders_with_direction(x, y, 1);
        let mut max = 0u64;
        while let Some((i, j, d)) = v.remove_first() {
            // println!("updating {i},{j}");
            let c = self.coordinates[j][i].distance;
            // println!("{c:?}, {d}");
            if c.is_some() && c.unwrap() < d { continue }
            self.set_distance(i, j, d);
            if d > max { max = d; }
            v.append(&mut self.get_borders_with_direction(i, j, d + 1));
        }
        max
    }
    fn get_borders(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut borders: Vec<(usize, usize)> = vec![];
        let rows = self.coordinates.len();
        let cols = self.coordinates[0].len();
        if x - 1 < x { borders.push((x-1, y)); }
        if y - 1 < y { borders.push((x, y-1)); }
        if x + 1 < cols { borders.push((x+1, y)); }
        if y + 1 < rows { borders.push((x, y+1)); }
        borders

    }
    fn get_borders_with_direction(&self, x: usize, y: usize, d: u64) ->
                   Vec<(usize, usize, u64)> {
        let mut borders: Vec<(usize, usize, u64)> = vec![];
        let rows = self.coordinates.len();
        let cols = self.coordinates[0].len();
        if x - 1 < x {
            if self.coordinates[y][x-1].direction.borders(Direction::W) {
                borders.push((x-1, y, d));
            }
        }
        if y - 1 < y {
            if self.coordinates[y-1][x].direction.borders(Direction::N) {
                borders.push((x, y-1, d));
            }
        }
        if x + 1 < cols {
            if self.coordinates[y][x+1].direction.borders(Direction::E) {
                borders.push((x+1, y, d));
            }
        }
        if y + 1 < rows {
            if self.coordinates[y+1][x].direction.borders(Direction::S) {
                borders.push((x, y+1, d));
            }
        }
        borders
    }
    fn poison_coordinate(&mut self, x: usize, y: usize) -> bool {
        let mut c = &mut self.coordinates[y][x];
        match c.direction {
            Pipe::NULL => {
                match c.distance {
                    Some(_) => false,
                    None => { c.direction = Pipe::Outside; true },
                }
            },
            _ => false,
        }

    }
    fn poison(&mut self) {
        let rows = self.coordinates.len();
        let cols = self.coordinates[0].len();
        let mut start: Option<(usize, usize)> = None;
        for y in 0..cols {
            for x in 0..rows {
                if (x != 0 && x != rows - 1) && (y != 0 && y != cols - 1) {
                    continue
                }
                match self.coordinates[y][x].distance {
                    None => { start = Some((x, y)); break; },
                    _ => (),
                }
            }
            if start.is_some() { break; }
        }
        if start.is_none() { panic!("Error finding start for poison"); }
        println!("Got start {start:?} for poison");
        let start = start.unwrap();
        let mut v = vec![(start.0, start.1)];
        while let Some((x, y)) = v.remove_first() {
            let c = &self.coordinates[y][x];
            if self.poison_coordinate(x, y) {
                v.append(&mut self.get_borders(x, y));
            }
        }
    }

    fn set_outside_inside(&mut self, x: usize, y: usize) {
        let mut e = &mut self.coordinates[y][x];
        match e.direction {
            Pipe::NULL => {
                
            },
            _ => panic!("Error, trying to set outside/inside for {e:?}"),
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.coordinates.iter().for_each(|row| {
            row.iter().for_each(|col| {
                match col.distance {
                    Some(d) => s.push_str(&format!("{:^5}", d)),
                    None => {
                        match col.direction {
                            Pipe::Outside => s.push_str("  o  "),
                            Pipe::Inside => s.push_str("  I  "),
                            _ => s.push_str("  x  "),
                        }
                    },
                }
            });
            s.push_str("\n");
        });
        write!(f, "{}", s)
    }
}

fn solve(v: &Vec<String>) -> (u64, u64) {
    let (mut grid, start) = Grid::new(v);
    println!("Got start {start:?}");
    let max = grid.populate(start.0, start.1);
    grid.poison();
    println!("{grid:?}");
    (max, 0)
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/short_input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let (max, inside) = solve(&lines);
    // assert_eq!(sum, 250347426);
	println!("{day}, part 1");
    println!("  the sum of winning hands is {max}");

	// // // Part 2
    // sum = get_winnings(&lines, true);
    // // assert_eq!(sum, 69929);
	// println!("{day}, part 2");
    // println!("  the sum of winning hands is {sum}");
}
