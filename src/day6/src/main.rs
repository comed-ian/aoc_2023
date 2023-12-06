use utils::read_lines;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn count_winning_options(&self) -> u64 {
        let mut speed = 0u64;
        let mut sum = 0u64;
        for i in 0..self.time {
            if (self.time - i) * speed > self.distance { sum += 1 }
            speed += 1;
        }
        sum
    }
}

fn get_vec(vals: &String) -> Vec<u64> {
    let mut split = vals.split(": ");
    split.next();
    let vals = split.next().unwrap().split(" ");
    let mut v: Vec<u64> = Vec::new();
    vals.for_each(|x| {
       if x != "" {
           v.push(x.trim().parse::<u64>().expect("failed to parse val"));
       }
    });
    v
}

fn get_vec_folded(vals: &String) -> Vec<u64> {
    let mut split = vals.split(": ");
    split.next();
    let vals = split.next().unwrap().split(" ");
    let mut v: Vec<&str> = Vec::new();
    let mut val = String::new();
    vals.for_each(|x| if x != "" { v.push(x); });
    v.iter().rev().for_each(|x| {
        let mut vvv = x.to_string();
        vvv.push_str(&val);
        val = vvv.to_string();
    });
    vec![val.parse::<u64>().expect("failed to parse val")]
}

fn get_data(s: &Vec<String>, folded: bool) -> Vec<Race> {
    let mut v: Vec<Race> = Vec::new();
    let times: Vec<u64>;
    let distances: Vec<u64>;
    if !folded {
        times = get_vec(&s[0]);
        distances = get_vec(&s[1]);
    } else {
        times = get_vec_folded(&s[0]);
        distances = get_vec_folded(&s[1]);
    }
    assert_eq!(times.len(), distances.len());
    for i in 0..times.len() {
        v.push(Race { time: times[i], distance: distances[i] });
    }
    v
}

fn find_winners(v: &Vec<String>, folded: bool) -> u64 {
    let races = get_data(&v, folded);
    races.iter().fold(1u64, |sum, x| sum * x.count_winning_options())
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let sum = find_winners(&lines, false);
    assert_eq!(sum, 303600);
	println!("{day}, part 1");
    println!("  the sum of points is {sum}");

	// Part 2
    let sum = find_winners(&lines, true);
    assert_eq!(sum, 23654842);
	println!("{day}, part 2");
    println!("  the total number of cards is {sum}");
}
