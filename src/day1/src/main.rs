use std::cmp::Ordering;
use std::convert::TryFrom;
use utils::read_lines;

#[derive(Debug, Clone, Copy)]
enum Word {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<String> for Word {
    type Error = ();
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match &s[..] {
            "zero"  => Ok(Self::Zero),
            "one"   => Ok(Self::One),
            "two"   => Ok(Self::Two),
            "three" => Ok(Self::Three),
            "four"  => Ok(Self::Four),
            "five"  => Ok(Self::Five),
            "six"   => Ok(Self::Six),
            "seven" => Ok(Self::Seven),
            "eight" => Ok(Self::Eight),
            "nine"  => Ok(Self::Nine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Word {
    fn into(self) -> u32 {
        match self {
           Self::Zero => 0u32,
           Self::One  => 1u32,
           Self::Two  => 2u32,
           Self::Three=> 3u32,
           Self::Four => 4u32,
           Self::Five => 5u32,
           Self::Six  => 6u32,
           Self::Seven=> 7u32,
           Self::Eight=> 8u32,
           Self::Nine => 9u32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    idx: usize,
    val: u32,
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

fn is_digit(v: u32) -> bool {
    v >= 0x30 && v < 0x40
}

fn find_first(s: &String) -> Coordinate {
    for (i, c) in s.chars().enumerate() {
        let v: u32 = c.into();
        if is_digit(v) { return Coordinate { idx: i, val: v - 0x30 } }
    }
    panic!("Could not find the first digit in {s}");
}

fn find_last(s: &String) -> Coordinate {
    for (i, c) in s.chars().rev().enumerate() {
        let v: u32 = c.into();
        if is_digit(v) { return Coordinate {
									idx: s.len() - i - 1,
									val: v - 0x30 }
		}
    }
    panic!("Could not find the last digit in {s}");
}

fn find_word_indices(s: &String) -> Option<(Coordinate, Coordinate)> {
    fn do_match(i: usize, c: char, len: usize, dup: &Vec<char>) ->
                Option<Coordinate> {
        match c {
            'o' | 'z' | 't' | 'f' | 's' | 'e' | 'n' => {
                for l in 3..6 {
                    if i + l <= len {
                        let res: Result<Word, ()> = dup[i..i+l]
                                                        .iter()
                                                        .collect::<String>()
                                                        .try_into();
                        match res {
                            Ok(m) => {
                                let val: u32 = m.into();
                                return Some(Coordinate { idx: i, val });
                            },
                            Err(_) => (),
                        }
                    }
                };
            },
            _ => ()
        }
        None
    }
    let dup: Vec<char> = s.chars().collect();
    let len = dup.len();
    let mut first: Option<Coordinate> = None;
    let mut second: Option<Coordinate> = None;
    for (i, c) in s.chars().enumerate() {
        match do_match(i, c, len, &dup) {
            Some(c) => { first = Some(c); break; }
            _ => (),
        }
    }
    for (i, c) in s.chars().rev().enumerate() {
        match do_match(len - i - 1, c, len, &dup) {
            Some(c) => { second = Some(c); break; }
            _ => (),
        }
    }
    if first.is_some() || second.is_some() {
        Some((first.unwrap(), second.unwrap()))
    }
    else { None }
}

fn get_calibration_value_1(s: &String) -> u32 {
    find_first(s).val * 10 + find_last(s).val
}

fn get_calibration_value_2(s: &String) -> u32 {
    let mut first = find_first(s);
    let mut last = find_last(s);
    match find_word_indices(s) {
        Some(t) => {
            if t.0 < first { first = t.0 }
			if t.1 > last { last = t.1 }
        },
		None => (),
    }
    first.val * 10 + last.val
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let mut sum: u64 = 0;
    lines.iter().for_each(|x| sum += get_calibration_value_1(x) as u64);
    assert_eq!(sum, 54953);
	println!("{day}, part 1");
    println!("  the sum of calibration values is {sum}");

	// Part 2
	sum = 0;
    lines.iter().for_each(|x| sum += get_calibration_value_2(x) as u64);
    assert_eq!(sum, 53868);
	println!("{day}, part 2");
    println!("  the sum of calibration values is {sum}");
}
