use utils::read_lines;

#[derive(Debug, Clone)]
struct Reading {
    starts: Vec<i64>,
    ends: Vec<i64>,
}

impl Reading {
    fn extrapolate(&self) -> i64 {
        self.ends.iter().fold(0i64, | sum, x | sum + x)
    }
    fn backwards(&self) -> i64 {
        self.starts.iter().rev().fold(0i64, | sum, x | x - sum)
    }
}

fn extrapolate(s: &String) -> (i64, i64) {
    let mut v: Vec<i64> = vec![];
    s.trim().split(" ").for_each(|x|
        v.push(x.parse::<i64>().expect("Failed to parse input val")));
    let mut r = Reading { ends: vec![], starts: vec![] };
    loop {
        let mut done = true;
        let mut new: Vec<i64> = vec![];
        r.starts.push(v[0]);
        for i in 0..v.len() - 1 {
            let diff = v[i+1] - v[i];
            if diff != 0 { done = false; }
            new.push(diff);
        }
        r.ends.push(*v.last().unwrap());
        if done { break; }
        v = new;
    }
    (r.extrapolate(), r.backwards())
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let mut sum = 0i64;
    let mut diff = 0i64;
    lines.iter().for_each(|x| {
        let (first, second) = extrapolate(x);
        sum += first;
        diff += second;
    });

    assert_eq!(sum, 1992273652);
	println!("{day}, part 1");
    println!("  the sum of extrapolations is {sum}");

	// Part 2
    assert_eq!(diff, 1012);
	println!("{day}, part 2");
    println!("  the sum of extrapolations is {diff}");
}
