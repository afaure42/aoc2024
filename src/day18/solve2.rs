
use std::collections::{HashSet, VecDeque};

use lib::utils::math::Vec2;

pub fn solve(input: &str) -> i128 {
	let mut grid: Vec<Vec<u8>> = vec![vec!['.' as u8; 71]; 71];
	let mut bytes: Vec<Vec2<usize>> = Vec::new();

	for (nbr, line) in input.lines().enumerate() {
		let pos = line.parse::<Vec2<usize>>().unwrap();

		bytes.push(pos);
	}

	// for n in 0..1024 {
	// 	let pos = &bytes[n];
	// 	grid[pos.y][pos.x] = '#' as u8;
	// }

	let start = Vec2::new(0, 0);
	let end = Vec2::new(70, 70);

	for n in 0..bytes.len() {
		let pos = &bytes[n];
		grid[pos.y][pos.x] = '#' as u8;

		if bfs(&grid, &start, &end).is_none() {
			println!("{pos}");
			break;
		}
	}

	0

}

fn get_value(grid: &[Vec<u8>], pos: &Vec2<i32>) -> Option<u8> {
	if	pos.y < 0
		|| pos.x < 0
		|| pos.x as usize >= grid.len()
		|| pos.y as usize >= grid.len() {
				return None;
		}
	
	Some(grid[pos.y as usize][pos.x as usize])
}

fn bfs (grid: &[Vec<u8>], start: &Vec2<i32>, end: &Vec2<i32>) -> Option<i32> {
	let dirs = [
		Vec2::new(-1, 0),
		Vec2::new(1, 0),
		Vec2::new(0, -1),
		Vec2::new(0, 1),
	];

	let mut queue = VecDeque::new();
	let mut visited = HashSet::new();

	queue.push_back((*start, 0));
	visited.insert(*start);

	while let Some((pos, dist)) = queue.pop_front() {
		if pos == *end {
			return Some(dist);
		}

		//visit neighbors
		for dir in &dirs {
			let test_pos = pos + *dir;
			if visited.contains(&test_pos) {
				continue;
			}

			if let Some(neighbor_val) = get_value(grid, &test_pos) {
				if neighbor_val != '#' as u8 {
					visited.insert(test_pos);
					queue.push_back((test_pos, dist + 1));
				}
			}
		}
	}

	None
}
