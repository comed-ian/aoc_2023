use std::collections::HashMap;
use utils::read_lines;

#[derive(Debug, Clone)]
struct CardCopy {
    idx: u32,
    points: u32,
    copies: u32,
    overlaps: u32,
}

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<u8>,
    numbers: Vec<u8>,
    idx: u32,
    points: u32,
    overlaps: u32,
}

impl Card {
    fn new(s: &String) -> Self {
        let mut split = s.split(" | ");
        let mut first = split.next().unwrap();
        let mut second = split.next().unwrap().trim();
        let mut first = first.split(": ");
        let mut card = first.next().unwrap().split(" ");
        card.next();
        let mut idx = 0u32;
        loop {
            match card.next().unwrap().trim().parse::<u32>() {
                Ok(x) => { idx = x; break; }
                _ => ()
            }
        }
        let mut first = first.next().unwrap().trim();
        let mut v: Vec<u8> = Vec::new();
        first.split(" ").for_each(|x| {
            match x.trim().parse::<u8>() {
                Ok(y) => v.push(y),
                _ => ()
            }
        });
        let mut vv: Vec<u8> = Vec::new();
        second.split(" ").for_each(|x| {
            match x.trim().parse::<u8>() {
                Ok(y) => vv.push(y),
                _ => ()
            }
        });
        Self { winning_numbers: v, numbers: vv, idx, points: 0, overlaps: 0 }
    }
    fn calculate_points(&mut self) -> u32 {
        let mut v = self.numbers.clone();
        let cnt = v.iter().filter(|x| self.winning_numbers.contains(x))
                           .collect::<Vec<&u8>>().len();
        self.overlaps = cnt as u32;
        if cnt == 0 { return 0 }
        let points = 2u32.pow((cnt - 1) as u32);
        self.points = points;
        points
    }
}

fn get_card_points(s: &String) -> u32 {
    let mut c = Card::new(s);
    c.calculate_points()
}

fn get_multiplying_card_points(v: &Vec<String>) -> u32 {
    let len = v.len() as u32;
    let mut clones: HashMap<u32, CardCopy> = HashMap::new();
    let mut cards: Vec<Card> = Vec::new();
    v.iter().for_each(|x| {
        let mut c = Card::new(x);
        _ = c.calculate_points();
        cards.push(c.clone());
        clones.insert(c.idx, CardCopy {
            idx: c.idx,
            overlaps: c.overlaps,
            points: c.points,
            copies: 1,
        });
    });

    let mut sum = 0u32;
    let mut idx = 1u32;
    loop {
        match clones.remove(&idx) {
            Some(c) => {
                sum += c.copies;
                for i in 1..c.overlaps + 1 {
                    let idx = c.idx + i;
                    match clones.get_mut(&idx) {
                       Some(cc) => cc.copies += c.copies,
                       None => (),
                    }
                }
                idx += 1;
            },
            None => break,
        }
    }
    sum
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let mut sum: u64 = 0;
    lines.iter().for_each(|x| sum += get_card_points(x) as u64);
    // assert_eq!(sum, 18619);
	println!("{day}, part 1");
    println!("  the sum of points is {sum}");

	// Part 2
    sum = get_multiplying_card_points(&lines) as u64;
    assert_eq!(sum, 8063216);
	println!("{day}, part 2");
    println!("  the total number of cards is {sum}");
}
