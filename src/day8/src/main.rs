use utils::read_input;
use std::fmt;
use std::str::Split;
use binary_tree::{Node, BinaryTree, WalkAction};
use std::collections::HashMap;

// #[derive(Clone, Debug)]
// struct DesertNode {
//     val: String,
//     left: Box<Option<Self>>,
//     right: Box<Option<Self>>,
// }
// 
// impl Node for DesertNode {
//     type Value = String;
//     fn left(&self) -> Option<&Self> { self.left.as_ref().as_ref().clone() }
//     fn right(&self) -> Option<&Self> { self.right.as_ref().as_ref().clone() }
//     fn value(&self) -> &String { &self.val }
//     fn walk<'static, F>(&'static self, step_in: F) where F: FnMut(&'static Self) -> WalkAction {
//     }
// }
// 
// struct Desert {
//     root: Option<DesertNode>
// }
// 
// impl BinaryTree for Desert {
//     type Node = DesertNode;
//     fn root(&self) -> Option<&Self::Node> { self.root.as_ref() }
// }

struct ShortestPath {
    grid: Vec<Vec<Option<u64>>>,
    instructions: Vec<char>,
    nodes: Vec<String>,
}

impl ShortestPath {
    fn new(instructions: &str, nodes: Split<'_, &str>) -> Self {
        let mut inner: Vec<Option<u64>> = Vec::new();
		for _ in 0..instructions.len() { inner.push(None); }
		let mut n = Vec::<String>::new();
		let n: Vec<String> = nodes.map(|s| s.to_string()).collect();

        Self {
            grid: vec![inner; n.len()],
            instructions: instructions.to_owned().chars().collect(),
            nodes: n,
        }
    }

    fn memoize(&mut self, i_idx: usize, n_idx: usize, val: u64) {
        self.grid[n_idx][i_idx] = Some(val);
    }

    fn fill_paths(&mut self) {
        for (i, n) in self.nodes.clone().iter().enumerate() {
            if n.ends_with('Z') {
                for j in 0..self.instructions.len() { self.memoize(j, i, 0); }
            }
        }
    }
}

impl fmt::Debug for ShortestPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		println!("{}", self.grid[0].len());
		let mut output = String::new();
		output.push_str("      ");
		self.instructions.iter().for_each(|x| output.push_str(&format!("{} ", x)[..]));
		output.push_str("\n");
		for i in 0..self.nodes.len() {
			output.push_str(&self.nodes[i][..]);
			for j in 0..self.instructions.len() {
				match self.grid[j][i] {
					Some(x) => output.push_str(&format!("{} ", x)),
					None => output.push_str(&format!("N ")),
				}
			}
			output.push_str("\n");
		}
		write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
struct DesertNode {
    name: String,
    left: String,
    right: String,
}

impl DesertNode {
    fn new(name: String, left: String, right: String) -> Self {
        Self { name, left, right }
    }
}

fn build_desert(nodes: Split<'_, &str>) -> HashMap<String, DesertNode> {
    let mut map: HashMap<String, DesertNode> = HashMap::new();
    let mut first: Option<DesertNode> = None;
    nodes.for_each(|x| {
        let mut split = x.split(" = ");
        let name = split.next().unwrap().trim().to_owned();
        let lr = split.next().unwrap().trim();
        let lr = lr.replace("(", "");
        let lr = lr.replace(")", "");
        let mut lr = lr.split(", ");
        let l = lr.next().unwrap().to_owned();
        let r = lr.next().unwrap().to_owned();
        let node = DesertNode::new(name.clone(), l, r);
        if first.is_none() { first = Some(node.clone()) }
        map.insert(name, node);

    });
    map
}

fn build_desert_for_ghosts(nodes: Split<'_, &str>) ->
                           (HashMap<String, DesertNode>, Vec<String>) {
    let map = build_desert(nodes);
    let mut starts = Vec::<String>::new();
    for (k, _) in map.iter() { if k.ends_with('A') { starts.push(k.clone()) } }
    (map, starts)
}

fn traverse(choices: &str, nodes: Split<'_, &str>) -> u64 {
    let map = build_desert(nodes);
    let mut curr = map.get(&"AAA".to_owned()).expect("Couldn't find node AAA");
    let mut i = 0usize;
    let mut counter = 0u64;
    loop {
        // println!("starting with node {curr:?}");
        match choices.chars().nth(i % choices.len()).unwrap() {
            'L' => {
                let next = &curr.left;
                // println!("moving left to node {next}");
                curr = map.get(next).expect("Couldn't find node {next}");
                // println!("Got node {curr:?}");
            },
            'R' => {
                let next = &curr.right;
                // println!("moving right to node {next}");
                curr = map.get(next).expect("Couldn't find node {next}");
                // println!("Got node {curr:?}");
            },
            _ => panic!("Error, cannot process choice"),
        }
        i += 1;
        counter += 1;
        if curr.name == "ZZZ" { break; }
    }
    counter
}

fn traverse_ghosts(choices: &str, nodes: Split<'_, &str>) -> u64 {
    let (map, mut starts) = build_desert_for_ghosts(nodes.clone());
    let mut i = 0usize;
    let mut counter = 0u64;
    let mut solved: bool = false;
	let solver = ShortestPath::new(choices, nodes);
	println!("{solver:?}");
//     loop {
//         solved = true;
//         // println!("starting with vec {starts:?}");
//         let mut next_nodes = Vec::<String>::new();
//         starts.iter().for_each(|x| {
//             let curr = map.get(x).expect("Couldn't find starting node");
//             // println!("starting with node {curr:?}");
//             match choices.chars().nth(i % choices.len()).unwrap() {
//                 'L' => {
//                     let next = &curr.left;
//                     // println!("moving left to node {next}");
//                     let next_node = map.get(next).expect("Couldn't find node {next}");
//                     next_nodes.push(next_node.name.clone());
//                     if !next_node.name.ends_with('Z') { solved = false; }
//                     // else { println!("got node that ends in 'Z'! {next_node:?}"); }
//                     // println!("Got node {next_node:?}");
//                 },
//                 'R' => {
//                     let next = &curr.right;
//                     // println!("moving left to node {next}");
//                     let next_node = map.get(next).expect("Couldn't find node {next}");
//                     next_nodes.push(next_node.name.clone());
//                     if !next_node.name.ends_with('Z') { solved = false; }
//                     // else { println!("got node that ends in 'Z'! {next_node:?}"); }
//                     // println!("Got node {next_node:?}");
//                 },
//                 _ => panic!("Error, cannot process choice"),
//             }
//         });
//         i += 1;
//         counter += 1;
//         if solved == true { break; }
//         starts = next_nodes;
//     }
    counter
}

fn main() {
	let day: String = env!("CARGO_PKG_NAME").to_owned();
	let mut path: String = "src/".to_owned();
    path.push_str(&day);
	path.push_str("/short_input.txt");
    let lines: String = read_input(&path[..]).expect("Failed to parse input");
    let mut split = lines.split("\n\n");
    let choices = split.next().unwrap().trim();
    let nodes = split.next().unwrap().trim().split("\n");

	// Part 1
    let sum = traverse(&choices, nodes.clone());
    // assert_eq!(sum, 16343);
	println!("{day}, part 1");
    println!("  the number of steps is {sum}");

	// Part 2
    let sum = traverse_ghosts(&choices, nodes);
    // // assert_eq!(sum, 69929);
	println!("{day}, part 2");
    println!("  the sum of winning hands is {sum}");
}
