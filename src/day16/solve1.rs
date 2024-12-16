use lib::utils::math::{Convert, Vec2};
use std::{collections::BinaryHeap, convert, usize};

pub fn solve(input: &str) -> i128 {
	let mut grid: Vec<Vec<u8>> = Vec::new();
	let mut start: Vec2<usize>;
	let mut end: Vec2<usize>;

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




	0
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
	pos: Vec2<i32>,
	dir: i32,
	cost: usize
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		other.cost.cmp(&self.cost)
		.then_with(|| other.dir.cmp(&self.dir))
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
	let mut dists: Vec<Vec<_>> = (0..grid.len()).map(|_| (0..grid.len()).map(|_| usize::MAX).collect()).collect();

	let mut heap: BinaryHeap<Node> = BinaryHeap::new();
	let end_i32 = Vec2::convert(end).unwrap();

	let dirs = [
		Vec2::new(-1, 0), //LEFT
		Vec2::new(0, 1), //DOWN
		Vec2::new(1, 0), //RIGHT
		Vec2::new(0, -1) //UP
	];

	dists[start.y][start.x] = 0;
	heap.push(Node {pos: Vec2::convert(start).unwrap(), dir: 1, cost: 0});

	while let Some(node) = heap.pop() {
		if node.pos == end_i32 {
			return node.cost as i32;
		}

		//if shorter way already found or we are in a wall
		if node.cost > dists[node.pos.y as usize][node.pos.x as usize]
		|| grid[node.pos.y as usize][node.pos.x as usize] == '#' as u8 {
			continue;
		}

		//test neighbors
		
		//test front
		let front_pos = node.pos + dirs[node.dir as usize];
		let left_pos = node.pos + dirs[clamp(node.dir + 1)];
		let right_pos = node.pos + dirs[clamp(node.dir - 1)];

		let front_pos_cost = node.cost + 1;
		let left_pos_cost = node.cost + 1001;
		let right_pos_cost = node.cost + 1001;

		if dists[front_pos.y as usize][front_pos.x as usize] > front_pos_cost {
			heap.push(Node {pos:front_pos, dir: node.dir, cost: front_pos_cost});
			dists[front_pos.y as usize][front_pos.x as usize] = front_pos_cost;
		}
		if dists[left_pos.y as usize][left_pos.x as usize] > left_pos_cost {
			heap.push(Node {pos:left_pos, dir: node.dir + 1, cost: left_pos_cost});
			dists[left_pos.y as usize][left_pos.x as usize] = left_pos_cost;
		}
		if dists[front_pos.y as usize][front_pos.x as usize] > front_pos_cost {
			heap.push(Node {pos:front_pos, dir: node.dir, cost: front_pos_cost});
			dists[front_pos.y as usize][front_pos.x as usize] = front_pos_cost;
		}
	}

	0
}
