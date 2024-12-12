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

				let (area, edges) = bfs(&grid, & mut visited, p);
				let perimeter = calc_sides(&grid, &edges, p);
				println!("Found area '{}' of size :{area} and perimeter:{perimeter}", *cell as char);
				
			}
		}
	}
	
	ret
}

fn in_bounds(grid: &Vec<Vec<u8>> , p: Point) -> bool {
	return 	!(p.y < 0
			|| p.x < 0
			|| p.y as usize >= grid.len()
			|| p.x as usize >= grid[0].len());
}

fn get_value(grid: &Vec<Vec<u8>>, p: &Point) -> u8 {
	grid[p.y as usize][p.x as usize]
}

fn is_side(grid: &Vec<Vec<u8>>, p: &Point, v: u8) -> bool {
	!in_bounds(grid, p) || get_value(grid, p) != v
}

fn calc_sides(grid: &Vec<Vec<u8>>, edges: &HashSet<Point>, start: Point) -> i32 {
	let dirs = vec![
		Point {x: -1, y: 0},
		Point {x: 0, y: -1},
		Point {x: 1, y: 0},
		Point {x: 0, y: 1}];
	let mut visited_sides: HashSet<Point> = HashSet::new();
	let ret = 0;
	let zone = get_value(grid, &start);

	for p in edges {
		for dir in &dirs {
			let neighbor = *p + *dir;
			
			if is_side(grid, &neighbor, zone) && !visited_sides.contains(&neighbor) {
				mark_side(grid, & mut visited_sides, p, dir);
			}
		}
	}

	ret
}

fn mark_side(grid: &Vec<Vec<u8>>, visited: & mut HashSet<Point>, start: &Point, dir: &Point) {

	// find the two perpendicular directions to explore both dir of the current side
}

fn bfs(grid: &Vec<Vec<u8>>, visited: & mut HashSet<Point>, start: Point) -> (i32, HashSet<Point>) {
	let mut to_visit: VecDeque<Point> = VecDeque::new();
	let mut area = 0;
	let mut perimeter = 0;
	let mut edges = HashSet::new();

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

			if !in_bounds(grid, current_neighbor) {
					//hit and edge so bump perimeter
					edges.insert(cell);
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
				edges.insert(cell);
			}
		}
	}

	(area, edges)
}
