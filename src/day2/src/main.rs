use utils::read_lines;

#[derive(Debug, Clone, Copy)]
struct Result {
    r: u8,
    b: u8,
    g: u8
}

impl Result {
    fn is_possible(&self, r: u8, b: u8, g: u8) -> bool {
        self.r <= r && self.b <= b && self.g <= g
    }
    fn power(&self) -> u64 {
        self.r as u64 * self.b as u64 * self.g as u64
    }
}

fn parse_cubes(s: &str) -> Result {
    let mut r: u8 = 0;
    let mut b: u8 = 0;
    let mut g: u8 = 0;
    let items = s.split("; ");
    for i in items {
        for ii in i.split(", ") {
            let mut j = ii.split(" ");
            let cnt = j.next().unwrap().parse::<u8>().unwrap();
            match j.next().unwrap() {
                "red" => if cnt > r  { r = cnt },
                "blue" => if cnt > b { b = cnt },
                "green" => if cnt > g { g = cnt },
                _ => panic!("Error trying to match color"),
            }
        }
    }
    Result { r, b, g }
}

fn is_possible(s: &String, r: u8, b: u8, g: u8) -> u64 {
    let mut s = s.split(": ");
    let mut first = s.next().unwrap().split(" ");
    first.next();
    let idx = first.next().unwrap().parse::<u64>().unwrap();
    let second = s.next().unwrap();
    if parse_cubes(second).is_possible(r, b, g) { idx }
    else { 0 }
}

fn get_power(s: &String) -> u64 {
    let mut s = s.split(": ");
    s.next();
    let second = s.next().unwrap();
    parse_cubes(second).power()
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let mut sum: u64 = 0;
    lines.iter().for_each(|x| sum += is_possible(x, 12, 14, 13));
    assert_eq!(sum, 2164);
	println!("{day}, part 1");
    println!("  the sum of possible game numbers is {sum}");

	// // Part 2
	sum = 0;
    lines.iter().for_each(|x| sum += get_power(x));
    assert_eq!(sum, 69929);
	println!("{day}, part 2");
    println!("  the sum of powers is {sum}");
}
