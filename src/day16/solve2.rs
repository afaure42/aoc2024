use lib::utils::math::Vec2;
use std::{collections::{BinaryHeap, HashMap, HashSet, VecDeque}, hash::Hash, io::stdin, usize};

pub fn solve(input: &str) -> i128 {
	let mut grid: Vec<Vec<u8>> = Vec::new();
	let mut start: Vec2<usize> = Vec2::new(0, 0);
	let mut end: Vec2<usize> = Vec2::new(0, 0);

	for (y, line) in input.lines().enumerate() {
		grid.push(line.as_bytes().to_vec());
		for (x, cell) in line.chars().enumerate() {
			if cell == 'S' {
				start = Vec2::new(x, y);
			} else if cell == 'E' {
				end = Vec2::new(x, y);				
			}
		}
	}


	bfs(&grid, start, end) as i128
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
	pos: Vec2<i32>,
	dir: i32,
	cost: usize,
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.cost.cmp(&self.cost)
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn clamp(n: i32) -> usize {
	if n < 0 {
		return 3;
	} else if n > 3 {
		return 0;
	}
	n as usize
}

fn bfs(grid: &Vec<Vec<u8>>, start: Vec2<usize>, end: Vec2<usize>) -> i32 {
	let mut dists: Vec<Vec<_>> = (0..grid.len())
		.map(|_|
		(0..grid.len()).map(|_| 
			vec![usize::MAX - 1001; 4]).collect())
		.collect();
	
	let mut end_dirs = HashSet::new();
	let mut visited: HashSet<Vec2<i32>> = HashSet::new();
	let mut heap: BinaryHeap<Node> = BinaryHeap::new();

	let end_i32 = Vec2::new(end.x as i32, end.y as i32);
	let start_i32 = Vec2::new(start.x as i32, start.y as i32);

	let dirs = [
		Vec2::new(-1, 0), //LEFT
		Vec2::new(0, 1), //DOWN
		Vec2::new(1, 0), //RIGHT
		Vec2::new(0, -1) //UP
	];

	dists[start.y][start.x][2] = 0;
	heap.push(Node {pos: start_i32, dir: 2, cost: 0});

	println!("Inserting end in map");
	while let Some(node) = heap.pop() {
		
		let current_dist: usize;
		current_dist = dists[node.pos.y as usize][node.pos.x as usize][node.dir as usize];
		
		//if shorter way already found or we are in a wall
		if node.cost > current_dist
		|| grid[node.pos.y as usize][node.pos.x as usize] == '#' as u8 {
			continue;
		}
	
		if node.pos == end_i32 {
			// return node.cost as i32;
			//setting dist for all directions since we dont care about the dir to count
			//as if we reached the end
			//so another one trying to reach the end from another dir will get denied if they come from further
			dists[end.y][end.x].iter_mut()
			.for_each(|cost| *cost = node.cost);

			//need to register end dirs to backtrack later

			end_dirs.insert(node.dir);
			continue;
		}

		//test neighbors
		let front_dir = node.dir;
		let left_dir = clamp(node.dir + 1);
		let right_dir = clamp(node.dir - 1 );
		
		//test front
		let front_pos = node.pos + dirs[node.dir as usize];
		let left_pos = node.pos;
		let right_pos = node.pos;

		let front_pos_cost = node.cost + 1;
		let left_pos_cost = node.cost + 1000;
		let right_pos_cost = node.cost + 1000;

		if	dists[left_pos.y as usize][left_pos.x as usize][left_dir as usize] >= left_pos_cost {
			heap.push(Node {pos:left_pos, dir: left_dir as i32, cost: left_pos_cost});
			dists[left_pos.y as usize][left_pos.x as usize][left_dir as usize] = left_pos_cost;
		}
		if dists[right_pos.y as usize][right_pos.x as usize][right_dir as usize] >= right_pos_cost {
			heap.push(Node {pos:right_pos, dir: right_dir as i32, cost: right_pos_cost});
			dists[right_pos.y as usize][right_pos.x as usize][right_dir as usize] = right_pos_cost;
		}
		if grid[front_pos.y as usize][front_pos.x as usize] != '#' as u8
		&& dists[front_pos.y as usize][front_pos.x as usize][front_dir as usize] >= front_pos_cost {
			heap.push(Node {pos:front_pos, dir: front_dir, cost: front_pos_cost});
			dists[front_pos.y as usize][front_pos.x as usize][front_dir as usize] = front_pos_cost;
		}
	}

	println!("Finished first bfs");

	//another bfs using the dist array to stay on the optimal pathS
	let mut queue = VecDeque::new();
	
	for n in end_dirs {
		queue.push_back((end_i32, n));
	}
	//the objective here is to go from the end and go trhough every cell that makes us go closer to the start
	//so any turn or move that reduces the dist
	while let Some((pos, dir)) = queue.pop_front() {
		let dist = dists[pos.y as usize][pos.x as usize][dir as usize];
		visited.insert(pos);
		if pos == start_i32 {
			continue;
		}
		//test for turns
		for (index, test_dist) in dists[pos.y as usize][pos.x as usize].iter().enumerate() {
			if *test_dist + 1000 == dist {
				queue.push_back((pos, index as i32));
			}
		}

		//test by cancelling move
		//cancelling by removing dir
		let old_pos = pos - dirs[dir as usize];
		if dists[old_pos.y as usize][old_pos.x as usize][dir as usize] + 1 == dist {
			queue.push_back((old_pos, dir));
		}
	}

	visited.len() as i32
}

fn print_grid (grid: &Vec<Vec<u8>>, except: &Vec2<i32>) {
	for (y, line) in grid.iter().enumerate() {
		for (x, c) in line.iter().enumerate() {
			if y as i32 == except.y && x as i32 == except.x {
				print!("X")
			} else {
				print!("{}", *c as char);
			}
		}
		print!("\n");
	}
}
