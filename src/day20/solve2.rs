
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{Hash};
use std::cmp::Reverse;
use std::usize;

use lib::utils::math::Vec2;

pub fn solve(input: &str) -> i128 {
	let mut grid: Vec<Vec<u8>> = Vec::new();
	let mut end: Vec2<i32> = Vec2::new(0, 0);
	let mut start: Vec2<i32> = Vec2::new(0, 0);
	let mut walkable_tiles: Vec<Vec2<i32>> = Vec::new();

	for (y, line) in input.lines().enumerate() {
		for (x, cell) in line.chars().enumerate() {
			if cell != '#'
			{
				let pos = Vec2::new(x as i32, y as i32);
				if cell == 'E' {end = pos}
				else if cell == 'S' {start = pos}
				walkable_tiles.push(pos);
			}
		}
		grid.push(line.as_bytes().to_vec());
	}

	println!("Start: {start}; End: {end}");
	let shortest_dists = bfs(&grid, &start, &end);
	walkable_tiles.sort_by(|a, b| {
		shortest_dists[a].cmp(&shortest_dists[b])
	});


	test_cheats(&shortest_dists, &walkable_tiles, 20, 100) as i128
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

fn bfs (grid: &[Vec<u8>], start: &Vec2<i32>, end: &Vec2<i32>) -> HashMap<Vec2<i32>, i32> {
	let dirs = [
		Vec2::new(-1, 0),
		Vec2::new(1, 0),
		Vec2::new(0, -1),
		Vec2::new(0, 1),
	];

	let mut priority_queue = BinaryHeap::new();
	let mut dists: HashMap<Vec2<i32>, i32> = HashMap::new();
	
	//init hashmap
	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			dists.insert(Vec2::new(x as i32, y as i32), i32::MAX);
		}
	}


	priority_queue.push(Reverse((*start, 0)));
	*dists.get_mut(start).unwrap() = 0;

	while let Some(Reverse((pos, dist))) = priority_queue.pop() {

		if dists[&pos] < dist	{continue}
		if pos == *end			{return dists}

		//visit neighbors
		for dir in &dirs {
			let test_pos = pos + *dir;
			let test_dist = dists[&test_pos];

			if let Some(neighbor_val) = get_value(grid, &test_pos) {
				if neighbor_val != '#' as u8
				&& dist + 1 < test_dist {
					*dists.get_mut(&test_pos).unwrap() = dist + 1;
					priority_queue.push(Reverse((test_pos, dist + 1)));
				}
			}
		}
	}

	dists
}

fn manhatann_dist(a: &Vec2<i32>, b: &Vec2<i32>) -> i32 {
	a.x.abs_diff(b.x) as i32 + a.y.abs_diff(b.y) as i32
}

fn test_cheats(dists: &HashMap<Vec2<i32>, i32>, shortest_path: &Vec<Vec2<i32>>, cheat_len: i32, threshold: i32) -> i32 {
	let mut ret = 0;
	for (n, tile) in shortest_path.iter().enumerate() {
		let tile_to_start = dists[tile];

		for target_tile in shortest_path.iter().skip(n) {

			let cheat_dist = manhatann_dist(tile, target_tile);
			let target_to_start = dists[target_tile];

			let cheat_to_start = tile_to_start + cheat_dist;
			if cheat_dist <= cheat_len
			&& cheat_to_start + threshold <= target_to_start{
				ret += 1;
			}
		}
	}

	ret
}
