use std::ops;
use std::fmt;

// https://adventofcode.com/2024/day/4
#[derive(Clone, Copy)]
struct Position {
	x: i32,
	y: i32,
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}", self.x, self.y)
	}
}

impl ops::Add<Position> for Position {
	type Output = Self;

	fn add(self, rhs: Position) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl ops::AddAssign<Position> for Position {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl ops::Sub<Position> for Position {
	type Output = Self;

	fn sub(self, rhs: Position) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl ops::SubAssign<Position> for Position {
	fn sub_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

fn main() {
	let input = include_str!("input.txt");


	println!("RET1: {}", solve1(input));
	println!("RET2: {}", solve2(input));
}

fn solve1 (input: &str) -> i128
{
	let mut matrix: Vec<&str> = Vec::new();
	let mut ret = 0;

	for line in input.lines() {
		matrix.push(line);
	}

	for (line_index, line) in matrix.iter().enumerate() {
		for (index, c) in line.char_indices() {

			if c != 'X' {
				continue;
			}

			let pos = Position {
				y: index as i32,
				x: line_index as i32,
			};
			let word = "XMAS";

			if search_word(&matrix, word, Position {x: -1, y: -1}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: -1, y: 0}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: -1, y: 1}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: 0, y: -1}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: 0, y: 1}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: 1, y: -1}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: 1, y: 0}, pos) {
				ret += 1;
			}
			if search_word(&matrix, word, Position {x: 1, y: 1}, pos) {
				ret += 1;
			}

		}
	}

	ret
}

fn search_word(matrix: &Vec<&str>, word: &str, dir: Position, mut pos: Position) -> bool {

	for c in word.chars() {
		if pos.x < 0 || pos.y < 0 {
			return false;
		}
		let result = get_char(matrix, pos);
		match result {
			Some(current_char) => {
				if current_char != c {
					return false;
				}
			}
			None => return false
		}

		pos = pos + dir;
	}

	true
}


fn solve2 (input: &str) -> i128
{
	let mut matrix: Vec<&str> = Vec::new();
	let mut ret = 0;

	for line in input.lines() {
		matrix.push(line);
	}

	for (line_index, line) in matrix.iter().enumerate() {
		for (index, c) in line.char_indices() {
			let pos = Position {
				y: index as i32,
				x: line_index as i32,
			};

			if c == 'A' && check_cross(&matrix, pos).unwrap_or(false) {
				ret += 1;
			}
		}
	}
	ret
}

fn check_cross(matrix: &Vec<&str>, pos: Position) -> Option<bool> {
	let top_left = get_char(matrix, pos - Position {x: -1, y: -1})?;
	let top_right = get_char(matrix, pos - Position {x: -1, y: 1})?;
	let bot_left = get_char(matrix, pos - Position {x: 1, y: -1})?;
	let bot_right = get_char(matrix, pos - Position {x: 1, y: 1})?;

	Some(check_pair(top_left, bot_right) && check_pair(top_right, bot_left))
}

fn check_pair(a: char, b: char) -> bool {
	(a == 'M' || a == 'S')
	&&
	(b == 'M' || b == 'S')
	&&
	(a != b)
}

fn get_char(matrix: &Vec<&str>, pos: Position) -> Option<char> {
	matrix.get(pos.x as usize)
	?.chars()
	.nth(pos.y as usize)
}
