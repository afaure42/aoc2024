use std::{char, collections::{HashMap, HashSet}};
use nalgebra::Vector2;


pub fn solve(input: &str) -> i128 {
	let mut antennas: HashMap<char, Vec<Vector2<i32>>> = HashMap::new();
	let mut y_max = 0;
	let x_max= input.lines().next().unwrap().len() as i32;
	let mut antinodes: HashSet<Vector2<i32>> = HashSet::new();

	for (line_nbr, line) in input.lines().enumerate() {
		for (index, c) in line.chars().enumerate() {
			if c != '.' {
				antennas
					.entry(c)
					.or_insert(Vec::new())
					.push(Vector2::new(index as i32, line_nbr as i32));
			}
		}
		y_max += 1;
	}


	//then do the math
	for pos_vec in antennas.values() {
		for pos_a in pos_vec {
			for pos_b in pos_vec {
				if pos_a == pos_b {continue}
				let dir_a = pos_a - pos_b;
				let dir_b = pos_b - pos_a;
				let mut antinode_a: Vector2<i32> = *pos_a;
				let mut antinode_b: Vector2<i32> = *pos_b;
				
				while valid_pos(antinode_a, y_max, x_max) {
					antinodes.insert(antinode_a);
					antinode_a += dir_a;
				}

				while valid_pos(antinode_b, y_max, x_max) {
					antinodes.insert(antinode_b);
					antinode_b += dir_b;
				}


			}
		}
	}

	// println!("{:?}", antinodes);
	antinodes.len() as i128
}

fn valid_pos (pos: Vector2<i32>, y_max: i32, x_max: i32) -> bool {
	pos.x >= 0 
	&& pos.x < x_max
	&& pos.y < y_max
	&& pos.y >= 0
}
