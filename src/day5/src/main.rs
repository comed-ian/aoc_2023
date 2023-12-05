use bio::data_structures::interval_tree::IntervalTree;
use std::cmp::Ordering;
use std::ops::Range;
use std::str::Split;
use utils::read_input;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Sub(u64),
}

#[derive(Debug)]
struct Map(IntervalTree<u64, Operation>);

impl Map {
    fn new() -> Self {
        Self { 0: IntervalTree::<u64, Operation>::new() }
    }
    fn get_output(&self, input: u64) -> u64 {
        let mut i = self.0.find(input..input+1);
        let res: u64 = match i.next() {
            Some(entry) => match entry.data() {
                Operation::Add(v) => input + v,
                Operation::Sub(v) => input - v,
            },
            None => input,
        };
        assert!(i.next().is_none());
        res
    }
    fn get_output_and_limit(&self, start: u64, range: u64) ->
                           (u64, Option<u64>) {
        let end = start + range;
        let mut i = self.0.find(start..start+1);
        let mut limit: Option<u64> = None;
        let res: u64 = match i.next() {
            Some(entry) => {
                let i_end = entry.interval().end;
                if i_end < end { limit = Some(i_end - start); }
                match entry.data() {
                    Operation::Add(v) => start + v,
                    Operation::Sub(v) => start - v,
                }
            },
            None => start,
        };
        assert!(i.next().is_none());
        (res, limit)
    }
}

fn get_map(s: &str) -> Map {
    let mut map: Map = Map::new();
    let mut i = s.split("\n");
    i.next();
    for ii in i {
        let mut iii = ii.trim().split(" ");
        let dest = iii.next().unwrap().parse::<u64>()
                      .expect("failed to parse dest");
        let src  = iii.next().unwrap().parse::<u64>()
                      .expect("failed to parse src");
        let range = iii.next().unwrap().parse::<u64>()
                      .expect("failed to parse range");
        match src.cmp(&dest) {
            Ordering::Less => {
                map.0.insert(src..src+range, Operation::Add(dest-src));
            },
            _ => { map.0.insert(src..src+range, Operation::Sub(src - dest)); }
        }
    }
    map
}

fn get_maps(mut split: Split<'_, &str>) -> Vec<Map> {
    let seed_to_soil = split.next().unwrap();
    let soil_to_fertilizer = split.next().unwrap();
    let fertilizer_to_water = split.next().unwrap();
    let water_to_light = split.next().unwrap();
    let light_to_temperature = split.next().unwrap();
    let temperature_to_humidity = split.next().unwrap();
    let humidity_to_location = split.next().unwrap();
    let mut maps: Vec<Map> = Vec::new();
    maps.push(get_map(seed_to_soil));
    maps.push(get_map(soil_to_fertilizer));
    maps.push(get_map(fertilizer_to_water));
    maps.push(get_map(water_to_light));
    maps.push(get_map(light_to_temperature));
    maps.push(get_map(temperature_to_humidity));
    maps.push(get_map(humidity_to_location));
    maps
}

fn get_seeds1(s: &str) -> Vec<u64> {
    let mut seed_nums: Vec<u64> = Vec::new();
    s.trim().split(" ").for_each(|x| {
        seed_nums.push(x.parse::<u64>().expect("Could not parse seed number"));
    });
    seed_nums
}

fn get_seeds2(s: &str) -> Vec<Range<u64>> {
    let mut seed_nums: Vec<Range<u64>> = Vec::new();
    let mut i = s.trim().split(" ");
    loop {
        let start = i.next();
        match start {
            Some(s) => {
                let begin = s.parse::<u64>()
                             .expect("Could not parse seed number");
                let range = i.next().unwrap().parse::<u64>()
                             .expect("Could not parse seed number");
                seed_nums.push(begin..begin+range);
            },
            None => break,
        }
    }
    seed_nums
}

fn part1(s: &String) -> u64 {
    let mut split = s.trim().split("\n\n");
    let seeds = split.next().unwrap();

    let mut i = seeds.split(": ");
    i.next();
    let seed_nums = get_seeds1(i.next().unwrap());
    let maps = get_maps(split);
    let mut min = 0xffff_ffff_ffff_ffffu64;
    for i in &seed_nums {
        let mut output: u64 = *i;
        for m in &maps {
            output = m.get_output(output)
        }
        if output < min { min = output; }
    }
    min
}

fn part2(s: &String) -> u64 {
    let mut split = s.trim().split("\n\n");
    let seeds = split.next().unwrap();

    let mut i = seeds.split(": ");
    i.next();
    let seed_nums = get_seeds2(i.next().unwrap());
    let maps = get_maps(split);

    let mut min = 0xffff_ffff_ffff_ffffu64;
    let mut to_check: Vec<u64> = Vec::new();
    for i in &seed_nums {
        let start = i.start;
        let end = i.end;
        to_check.push(start);
        loop {
            match to_check.pop() {
                Some(v) => {
                    let mut output = v;
                    let mut range = end - v;
                    for m in &maps {
                        let new_r: Option<u64>;
                        (output, new_r) = m.get_output_and_limit(output, range);
                        match new_r {
                            Some(s) => if s < range {
                                range = s;
                                to_check.push(v + range);
                            }
                            None => (),
                        }
                    }
                    if output < min { min = output; }
                },
                None => break,
            }
        }
    }
    min
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");
    let data: String = read_input(&path[..])
        .expect("Failed to read input");

	// Part 1
    let min = part1(&data);
    assert_eq!(min, 1181555926);
	println!("{day}, part 1");
    println!("  the lowest location number is {min}");

	// // Part 2
    let min = part2(&data) as u64;
    assert_eq!(min, 37806486);
	println!("{day}, part 2");
    println!("  the lowest location number is {min}");
}
