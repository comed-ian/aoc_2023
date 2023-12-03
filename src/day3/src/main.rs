use std::fmt;
use utils::read_lines;

#[derive(Debug, Clone, Copy)]
struct PartNumber {
    start: Coordinate,
    end: Coordinate,
    val: u32,
}

#[derive(Debug, Clone, Copy)]
struct Gear {
    coord: Coordinate,
    borders: u8,
    border_val: u32,
}

impl Gear {
    fn get_borders(&mut self, parts: &Vec<PartNumber>) {
        parts.iter().for_each(|pt| {
            let start_col = pt.start.x;
            let end_col = pt.end.x;
            let row = pt.start.y;
            let x = self.coord.x;
            let y = self.coord.y;
            // check left and right
            if (x == start_col - 1 && row == y) ||
                (x == end_col + 1 && row == y) ||
                // check top
                (x >= start_col && x <= end_col && y == row - 1) ||
                // check bottom
                (x >= start_col && x <= end_col && y == row + 1) ||
                // check diagonal
                (x == start_col - 1 && y == row  - 1) ||
                (x == start_col - 1 && y == row  + 1) ||
                (x == end_col + 1 && y == row  + 1) ||
                (x == end_col + 1 && y == row  - 1) {
                self.borders += 1;
                self.border_val *= pt.val;
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn row(&self) -> u32 { self.x }
    fn column(&self) -> u32 { self.y }
}

#[derive(Debug, Clone)]
struct Grid {
    rows: usize,
    columns: usize,
    data: Vec<bool>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.columns {
                write!(f, "{} ", self.abbreviated_val(r, c))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn len(&self) -> usize { self.data.len() }
    fn coordinate(&self, idx: usize) -> Coordinate {
        Coordinate { x: (idx % self.rows) as u32, y: (idx / self.rows) as u32 }
    }
    fn set(&mut self, r: usize, c: usize) { self.data[r * self.rows + c] = true }
    fn val(&self, r: usize, c: usize) -> bool {
        if r > self.rows - 1 || c > self.columns - 1 { return false }
        self.data[r * self.rows + c]
    }
    fn abbreviated_val(&self, r: usize, c: usize) -> char {
        match self.data[r * self.rows + c] {
            true => return 'T',
            false => return 'f',
        }
    }
    fn borders(&self, pt: &PartNumber) -> bool {
        let start_col = pt.start.x as usize;
        let end_col = pt.end.x as usize;
        let row = pt.start.y as usize;
        // check top border
        for y in start_col..end_col + 1 {
            if self.val(row - 1, y) { return true }
        }
        // check left and right borders
        if self.val(row, start_col - 1) { return true }
        if self.val(row, end_col + 1) { return true }
        // check bottom border
        for y in start_col..end_col + 1 {
            if self.val(row + 1, y) { return true }
        }
        // check diagonals
        if self.val(row - 1, start_col - 1) { return true }
        if self.val(row - 1, end_col + 1) { return true }
        if self.val(row + 1, start_col - 1) { return true }
        if self.val(row + 1, end_col + 1) { return true }
        false
    }
}

fn _get_symbols(s: &String, x: u32) -> Vec<Coordinate> {
    let mut v: Vec<Coordinate> = Vec::new();
    for (y, c) in s.chars().enumerate() {
        match c as u8 {
            n if (n < 0x30 || n > 0x39) && n != 0x2e => {
                v.push(Coordinate { x, y: y.try_into().unwrap() });
            },
            _ => (),
        }
    }
    v
}

fn _get_symbols_stars(s: &String, x: u32) -> Vec<Coordinate> {
    let mut v: Vec<Coordinate> = Vec::new();
    for (y, c) in s.chars().enumerate() {
        match c as u8 {
            0x2a => {
                v.push(Coordinate { x, y: y.try_into().unwrap() });
            },
            _ => (),
        }
    }
    v
}

fn get_part_numbers(s: &String, row: u32) -> Vec<PartNumber> {
    let mut v: Vec<PartNumber> = Vec::new();
    let len = s.len() as u32;
    let mut val = 0;
    let mut end = 0u32;
    let mut pow = 0;
    for (i, c) in s.chars().rev().enumerate() {
        match c.to_digit(10) {
            Some(d) => {
                val += d * (10u32.pow(pow));
                if end == 0 { end = len - (i as u32) - 1};
                pow+= 1
            },
            None => {
                if (val != 0) {
                    v.push(PartNumber {
                             start: Coordinate { x: len - i as u32, y: row },
                             end:   Coordinate { x: end as u32, y: row },
                             val
                    });
                    val = 0; end = 0; pow = 0;
                }
            },
        }
    }
    // handle edge case
    if val != 0 {
        v.push(PartNumber {
                 start: Coordinate { x: 0, y: row },
                 end:   Coordinate { x: end as u32, y: row },
                 val
        });
    }
    v
}

fn get_symbols(data: &Vec<String>, stars_only: bool) -> Grid {
    let rows = data.len();
    let columns = data[0].len();
    let mut grid = Grid { rows, columns, data: vec![false; rows * columns] };
    for (i, s) in data.iter().enumerate() {
        if stars_only {
            for s in _get_symbols_stars(s, i as u32) {
                grid.set(s.x as usize, s.y as usize);
            }
        }
        else {
            for s in _get_symbols(s, i as u32) {
                grid.set(s.x as usize, s.y as usize);
            }
        }
    }
    grid
}

fn find_adjacent_parts(data: &Vec<String>) -> u32 {
    let grid = get_symbols(data, false);
    let mut sum = 0u32;
    for (i, x) in data.iter().enumerate() {
        let v: Vec<PartNumber> = get_part_numbers(x, i as u32)
                                    .into_iter()
                                    .filter(|pt| grid.borders(&pt))
                                    .collect();
        sum += v.iter().fold(0u32, |sum, x| sum + x.val);
    }
    sum
}

fn find_adjacent_gears(data: &Vec<String>) -> u32 {
    let grid = get_symbols(data, true);
    let mut gears: Vec<Gear> = Vec::new();
    for x in 0..grid.len() {
        if grid.data[x] { gears.push(Gear {
                                coord: grid.coordinate(x),
                                borders: 0,
                                border_val: 1,
                          });
        }
    }
    let mut v: Vec<PartNumber> = Vec::new();
    for (i, x) in data.iter().enumerate() {
        v.extend(get_part_numbers(x, i as u32));
    }
    gears.iter_mut().for_each(|x| x.get_borders(&v));
    let mut sum = 0u32;
    sum += gears.iter().fold(0u32, |sum, x| {
        if x.borders == 2 { sum + x.border_val }
        else { sum }
    });
    sum
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let mut sum: u32 = find_adjacent_parts(&lines);
	println!("{day}, part 1");
    println!("  the sum of part numbers is {}", sum);
    assert_eq!(sum, 531561);

	// Part 2
    sum = find_adjacent_gears(&lines);
    assert_eq!(sum, 83279367);
	println!("{day}, part 2");
    println!("  the sum of gear powers is {sum}");
}
