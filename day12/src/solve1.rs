use std::fmt;
use std::ops;
use std::collections::{HashSet, VecDeque};


#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
	x: i64,
	y: i64,
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

pub fn solve(input: &str) -> u32 {
	let mut grid: Vec<Vec<u8>> = Vec::new();
	let mut visited: HashSet<Point> = HashSet::new();
	let mut ret = 0;

	for line in input.lines() {
		println!("{}", line.len());
		grid.push(line.as_bytes().to_vec());
	}

	for (y, row) in grid.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			let p = Point {x:x as i64, y:y as i64};
			if !visited.contains(&p) {

				let (area, perimeter) = bfs(&grid, & mut visited, p);
				println!("Found area '{}' of size :{area} and perimeter:{perimeter}", *cell as char);
				ret += area * perimeter;
			}
		}
	}
	
	ret
}


fn bfs(grid: &Vec<Vec<u8>>, visited: & mut HashSet<Point>, start: Point) -> (u32, u32) {
	let mut to_visit: VecDeque<Point> = VecDeque::new();
	let mut area = 0;
	let mut perimeter = 0;

	let dirs = vec![
		Point {x: -1, y: 0},
		Point {x: 1, y: 0},
		Point {x: 0, y: -1},
		Point {x: 0, y: 1}];

	let start_type = grid[start.y as usize][start.x as usize];
	to_visit.push_back(start);
	visited.insert(start);

	while !to_visit.is_empty() {

		//visit
		let cell = to_visit.pop_front().unwrap();
		// let current_value = grid[cell.y as usize][cell.x as usize];

		area += 1;

		//add neighbors
		for dir in dirs.iter() {
			let current_neighbor = cell + *dir;

			if current_neighbor.y < 0
				|| current_neighbor.x < 0
				|| current_neighbor.y as usize >= grid.len()
				|| current_neighbor.x as usize >= grid[0].len() {
					//hit and edge so bump perimeter
					perimeter += 1;
					continue;
			}
			let neighbor_value = grid[current_neighbor.y as usize][current_neighbor.x as usize];

			if neighbor_value == start_type {
				if !visited.contains(&current_neighbor) {
					visited.insert(current_neighbor);
					to_visit.push_back(current_neighbor);
				}
			} else {
				//on a side of current zone
				perimeter += 1;
			}
		}
	}
	(area, perimeter)
}
