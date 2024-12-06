use std::collections::{HashSet};
use std::ops;
use std::fmt;
// use std::io::{stdin};

// https://adventofcode.com/2024/day/4
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
	x: i32,
	y: i32,
}

#[derive(Clone, Copy)]
struct Node {
	content: char,
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

fn main() {
	let input = include_str!("../input.txt");


	println!("RET1: {}", solve1(input));
	println!("RET2: {}", solve2(input));
}

fn solve1 (input : &str) -> i128
{
	let dirs: Vec<Point> =vec![
		Point {x: 0, y: -1}, // up
		Point {x: 1, y: 0}, //right
		Point {x: 0, y: 1}, //down
		Point {x: -1, y: 0} //left
		 ];
	
	let mut current_dir = 0;
	let mut pos = Point {x: 0, y: 0};
	let mut visited: HashSet<Point> = HashSet::new(); //starting at one because start pos already visited
	
	let mut grid: Vec<Vec<Node>> = Vec::new();

	//parsing grid
	for (line_nbr, line) in input.lines().enumerate() {
		let temp: Vec<Node> = line.chars().enumerate().map(|(char_nbr, c)| {
			if c == '^' {
				pos.x = char_nbr as i32;
				pos.y = line_nbr as i32;
				
				Node {content: '.'}
			} else {
				Node {content: c}
			}
		}).collect();

		grid.push(temp);
	}

	println!("STARTING : {pos}");
	// graph traversal
	loop {
		visited.insert(pos);

		let next_pos = advance(&grid, pos, & mut current_dir, &dirs);
		if next_pos.is_none() {	break;	}
		pos = next_pos.unwrap();
	}

	return visited.len() as i128;
}

fn solve2 (input : &str) -> i128
{
	let dirs: Vec<Point> =vec![
		Point {x: 0, y: -1}, // up
		Point {x: 1, y: 0}, //right
		Point {x: 0, y: 1}, //down
		Point {x: -1, y: 0} //left
		 ];
	
	let mut pos = Point {x: 0, y: 0};
	let mut valid_obstacles: i128 = 0;
	
	let mut grid: Vec<Vec<Node>> = Vec::new();

	//parsing grid
	for (line_nbr, line) in input.lines().enumerate() {
		let temp: Vec<Node> = line.chars().enumerate().map(|(char_nbr, c)| {
			if c == '^' {
				pos.x = char_nbr as i32;
				pos.y = line_nbr as i32;
				
				Node {content: '.'}
			} else {
				Node {content: c}
			}
		}).collect();

		grid.push(temp);
	}
	let mut grid_clone = grid.clone();
	// graph traversal
	let mut total_iters = 0;
	println!("size :{}", grid.len() * grid[0].len());
	for (y, row) in grid.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if cell.content == '.'  && ! (x as i32 == pos.x && y as i32== pos.y ){
				total_iters += 1;
				*grid_clone.get_mut(y).unwrap().get_mut(x).unwrap() = Node {content: '#'};
				if grid_clone[y][x].content != '#' { panic!("WTF")}
				if is_loop(&grid_clone, pos , 0, &dirs) {
					valid_obstacles += 1;
				}
				*grid_clone.get_mut(y).unwrap().get_mut(x).unwrap() = Node {content: '.'};
				if grid_clone[y][x].content == '#' { panic!("WTF")}
			}
		}
	}
	println!("{total_iters}");

	valid_obstacles
}

fn advance (grid : & Vec<Vec<Node>>, pos: Point, current_dir: & mut usize, dirs: & Vec<Point>) -> Option<Point> {

	let mut next_pos = pos + dirs[*current_dir];

	let next_node = grid.get(next_pos.y as usize)?.get(next_pos.x as usize)?;

	//if direction change is needed
	if next_node.content == '#' {
		*current_dir += 1;
		if *current_dir >= dirs.len() { *current_dir = 0;}

		next_pos = pos + dirs[*current_dir];
		if next_pos.x < 0 || next_pos.y < 0 {
			return None
		}

		if next_pos.y as usize >= grid.len() || next_pos.x as usize >= grid[0].len() {
			return None
		}
		//check bounds
		// grid.get(next_pos.y as usize)?.get(next_pos.x as usize)?;
	}

	Some(next_pos)
}

fn is_loop(grid: &Vec<Vec<Node>>, start: Point, start_dir: usize, dirs: & Vec<Point>) -> bool {
	
	// let mut dir_map: HashMap<Point, u8> = HashMap::new();
	let mut dir= start_dir;
	let mut pos = start;
	for _n in 1..1000000 {


		// visit and check that we didnt visit here with this direction before
		// if dir_map.contains_key(&pos) {
		// 	if (dir_map[&pos] & ( 1 << dir)) != 0 {return true;}
		// 	*dir_map.get_mut(&pos).unwrap() |= 1 << dir;
		// } else {
		// 	dir_map.insert(pos, 1 << dir);
		// }

		let next_pos = advance(&grid, pos, & mut dir, &dirs);
		if next_pos.is_none() {	return false;	}
		pos = next_pos.unwrap();		
	}

	true
}
