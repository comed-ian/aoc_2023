use utils::read_lines;

struct Galaxy {
    x: isize,
    y: isize,
    expansion_x: isize,
    expansion_y: isize,
}

impl Galaxy {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y, expansion_x: 0, expansion_y: 0 }
    }
    fn expand(&mut self, factor: isize) {
        let expansion: isize = self.expansion_x * (factor - 1);
        assert!(expansion >= 0isize);
        self.x += expansion;
        let expansion: isize = self.expansion_y * (factor - 1);
        assert!(expansion >= 0isize);
        self.y += expansion;
    }

    fn shortest_path(&self, other: &Self) -> isize {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        delta_x.abs() + delta_y.abs()
    }
}

fn sum_paths(g: &Vec<Galaxy>) -> isize {
    let mut sum = 0isize;

    for (i, gg) in g.iter().enumerate() {
        for j in i+1..g.len() {
            match g.get(j) {
                Some(other) => { sum += gg.shortest_path(&other); },
                None => (),
            }
        }
    }
    sum
}

fn get_galaxies(v: &Vec<String>) -> Vec<Galaxy> {
    let mut g: Vec<Galaxy> = vec![];
    let mut rows: Vec<bool> = vec![true; v.len()];
    let mut cols: Vec<bool> = vec![true; v[0].len()];
    for (j, x) in v.iter().enumerate() {
        for (i, c) in x.chars().enumerate() {
            if c == '#' {
                g.push(Galaxy::new(i as isize, j as isize));
                rows[j] = false;
                cols[i] = false;
            }
        }
    }

    for (c, b) in cols.iter().enumerate() {
        g.iter_mut().for_each(|x| if *b && x.x > c as isize { x.expansion_x += 1 });
    }
    for (r, b) in rows.iter().enumerate() {
        g.iter_mut().for_each(|x| if *b && x.y > r as isize { x.expansion_y += 1 });
    }
    g
}

fn shortest_path(v: &Vec<String>, factor: isize) -> isize {
    let mut g = get_galaxies(v);
    g.iter_mut().for_each(|mut x| x.expand(factor));
    sum_paths(&g)
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/input.txt");

	// Part 1
    let lines: Vec<String> = read_lines(&path[..]);
    let sum = shortest_path(&lines, 2);
    assert_eq!(sum, 9565386);
	println!("{day}, part 1");
    println!("  the sum of shortest_paths is {sum}");

	// Part 2
    let sum = shortest_path(&lines, 1_000_000);
    assert_eq!(sum, 857986849428);
	println!("{day}, part 2");
    println!("  the sum of shortest paths is {sum}");
}
