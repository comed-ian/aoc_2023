use std::cmp::Ordering;
use utils::read_lines;

#[derive(Clone, Copy, Debug)]
enum Card {
	Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
			'1' =>  Card::Joker,
            '2' =>  Card::Two,
            '3' =>  Card::Three,
            '4' =>  Card::Four,
            '5' =>  Card::Five,
            '6' =>  Card::Six,
            '7' =>  Card::Seven,
            '8' =>  Card::Eight,
            '9' =>  Card::Nine,
            'T' =>  Card::Ten,
            'J' =>  Card::Jack,
            'Q' =>  Card::Queen,
            'K' =>  Card::King,
            'A' =>  Card::Ace,
            _ => panic!("Invalid card {c}"),
        }
    }
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        match self {
			Card::Joker => 1,
            Card::Two   => 2,
            Card::Three => 3,
            Card::Four  => 4,
            Card::Five  => 5,
            Card::Six   => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine  => 9,
            Card::Ten   => 10,
            Card::Jack  => 11,
            Card::Queen => 12,
            Card::King  => 13,
            Card::Ace   => 14,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq)]
enum Outcome {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Into<u8> for Outcome {
    fn into(self) -> u8 {
        match self {
            Self::HighCard     => 1,
            Self::Pair         => 2,
            Self::TwoPair      => 3,
            Self::ThreeOfAKind => 4,
            Self::FullHouse    => 5,
            Self::FourOfAKind  => 6,
            Self::FiveOfAKind  => 7,
        }
    }
}

impl Ord for Outcome {
    fn cmp(&self, other: &Self) -> Ordering {
        Into::<u8>::into(*self).cmp(&Into::<u8>::into(*other))
    }
}

impl PartialOrd for Outcome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Outcome {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<(&[Card; 5], bool)> for Outcome {
    fn from(t: (&[Card; 5], bool)) -> Self {
		let cards = t.0;
		let jokers = t.1;
        let mut v = vec![0u8; 14];
        cards.iter().for_each(|x| v[(Into::<u8>::into(*x) - 1) as usize] += 1);
		let mut count_jokers = 0;
		if jokers {
			count_jokers = v[0];
			v[0] = 0;
		}
		let stats: Vec<&u8> = v.iter().filter(|x| **x != 0).collect();
        let max = match stats.iter().max() {
			None => 0,
			Some(x) => **x,

		};
        let min = match stats.iter().min() {
			None => 0,
			Some(x) => **x,
		};
        let cnt = stats.iter().fold(0u8, | c, x | if **x != 0 { c + 1 } else { c });
        let mut outcome = match max {
            5 => Outcome::FiveOfAKind,
            4 => Outcome::FourOfAKind,
            3 => {
                if min == 2 { Outcome::FullHouse }
                else { Outcome::ThreeOfAKind }
            },
            2 => if count_jokers == 0 {
					if cnt == 3 { Outcome::TwoPair } else { Outcome::Pair }
				} else {
					if cnt == 2 { Outcome::TwoPair } else { Outcome::Pair }
			},
            1 => Outcome::HighCard,
			0 => Outcome::HighCard,
            _ => panic!("Error processing outcome of {max}"),
        };
		if !jokers { return outcome }
		for _ in 0..count_jokers {
			outcome = match outcome {
				Outcome::HighCard => 	 Outcome::Pair,
				Outcome::Pair     => 	 Outcome::ThreeOfAKind,
				Outcome::TwoPair  => 	 Outcome::FullHouse,
				Outcome::ThreeOfAKind => Outcome::FourOfAKind,
				Outcome::FullHouse => 	 Outcome::FourOfAKind,
				Outcome::FourOfAKind =>  Outcome::FiveOfAKind,
				Outcome::FiveOfAKind =>  Outcome::FiveOfAKind,
			}
		}
		outcome
    }
}

#[derive(Clone, Copy, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
    outcome: Outcome,
}

impl Hand {
    fn new(s: &str, jokers: bool) -> Self {
        let mut split = s.split(" ");
        let hand = split.next().unwrap();
        let bid = split.next().unwrap().parse::<u64>()
                                       .expect("failed to parse bid");
        let mut cards: [Card; 5] = [Card::Two; 5];
        for (i, c) in hand.chars().enumerate() { cards[i] = Card::from(c) }
        let outcome = Outcome::from((&cards, jokers));
        Self { cards, bid, outcome }
    }
    fn is_stronger(&self, other: &Self) -> bool {
        for i in 0..5 {
            let val: u8 = self.cards[i].into();
            let other_val: u8 = other.cards[i].into();
            if val != other_val { return val > other_val }
        }
        return true
    }
}

fn get_winnings(s: &Vec<String>, jokers: bool) -> u64 {
    let mut hands: Vec<Hand> = Vec::new();
    s.iter().for_each(|x| {
		if jokers {
			let repl = x.trim().replace("J", "1");
			hands.push(Hand::new(&repl, true));
		} else {
			hands.push(Hand::new(x.trim(), false));
		}
	});
    let mut sorted: Vec<Hand> = Vec::new();
    for i in 0..hands.len() {
        let hand = hands[i].clone();
		let mut inserted: bool = false;
        for j in 0..sorted.len() {
            let cmp = sorted[j].clone();
            match hand.outcome.cmp(&cmp.outcome) {
				Ordering::Less => {
					sorted.insert(j, hand);
					inserted = true;
					break;
				},
				Ordering::Equal => if !hand.is_stronger(&cmp) {
					sorted.insert(j, hand);
					inserted = true;
					break;
				},
				Ordering::Greater => (),
			}
        }
		if !inserted { sorted.push(hand); }
    }
	assert_eq!(hands.len(), sorted.len());
	let mut sum = 0u64;
	for (i, h) in sorted.iter().enumerate() {
		sum += h.bid * (i as u64 + 1);
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
    let mut sum: u64;
    sum = get_winnings(&lines, false);
    assert_eq!(sum, 250347426);
	println!("{day}, part 1");
    println!("  the sum of winning hands is {sum}");

	// // Part 2
    sum = get_winnings(&lines, true);
    // assert_eq!(sum, 69929);
	println!("{day}, part 2");
    println!("  the sum of winning hands is {sum}");
}
