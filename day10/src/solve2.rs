use std::collections::VecDeque;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
	x: i32,
	y: i32,
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}", self.x, self.y)
	}
}

impl ops::Add<Point> for Point {
	type Output = Self;

	fn add(self, rhs: Point) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl ops::AddAssign<Point> for Point {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl ops::Sub<Point> for Point {
	type Output = Self;

	fn sub(self, rhs: Point) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl ops::SubAssign<Point> for Point {
	fn sub_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

pub fn solve(input: &str) -> i128 {
	let mut grid: Vec<Vec<i8>> = Vec::new();
	let mut ret = 0;

	for line in input.lines() {
		grid.push(Vec::new());
		for c in line.chars() {
			let n = i8::try_from(c.to_digit(10)
				.expect("only digits in input file"))
				.unwrap();
			grid.last_mut().unwrap().push(n);
		}
	}

	for (y, row) in grid.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == 0 {
				ret += bfs(&grid, Point {y:y as i32, x:x as i32}) as i128;
			}
		}
	}
	ret
}

fn bfs(grid: &Vec<Vec<i8>>, start: Point) -> u32 {
	let mut to_visit: VecDeque<Point> = VecDeque::new();

	let dirs = vec![
		Point {x: -1, y: 0},
		Point {x: 1, y: 0},
		Point {x: 0, y: -1},
		Point {x: 0, y: 1}];
	let mut ret = 0;

	to_visit.push_back(start);

	while !to_visit.is_empty() {

		//visit
		let cell = to_visit.pop_front().unwrap();
		let current_value = grid[cell.y as usize][cell.x as usize];

		if current_value == 9 {
			ret += 1;
			continue;
		}

		//add neighbors
		for dir in dirs.iter() {
			let current_neighbor = cell + *dir;

			if current_neighbor.y < 0
				|| current_neighbor.x < 0
				|| current_neighbor.y as usize >= grid.len()
				|| current_neighbor.x as usize >= grid[0].len() {
					continue;
				}
			let neighbor_value = grid[current_neighbor.y as usize][current_neighbor.x as usize];

			if neighbor_value == current_value + 1 {
				to_visit.push_back(current_neighbor);
			}
		}
	}
	ret
}
