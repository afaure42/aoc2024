use core::str;
use std::collections::HashSet;
use core::array;
use std::io::stdin;
use std::fmt;
use std::ops;


#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Point {
	x: i64,
	y: i64,
}

impl Point {
	fn new(x: i64, y: i64) -> Point {
		Point {x, y}
	}

	fn parse(string: &str) -> Option<Point> {
		let mut iter = string.split(',');
		let mut ret = Point::new(0, 0);
		
		if let Ok(x) = iter.next()?.parse::<i64>() {
			ret.x = x;
		} else {
			return None;
		}
		if let Ok(y) = iter.next()?.parse::<i64>() {
			ret.y = y;
		} else {
			return None;
		}
		
		Some(ret)
	}
}

impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}", self.x, self.y)
	}
}

impl ops::Mul<i64> for Point {
	type Output = Self;

	fn mul(self, rhs: i64) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
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
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl ops::Rem<Point> for Point {
	type Output = Self;

	fn rem(self, rhs: Point) -> Self {
		Self {
			x: self.x % rhs.x,
			y: self.y % rhs.y 
		}
	}
}

impl ops::Div<Point> for Point {
	type Output = Self;

	fn div(self, rhs: Point) -> Self {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y 
		}
	}
}

pub fn solve(input: &str) -> i128 {
	let mut bots: Vec<(Point, Point)> = Vec::new();
	let mut quadrants: [HashSet<usize>; 4] = array::from_fn(|_i| HashSet::new());
	let size = Point::new(101, 103);

	for line in input.lines() {
		let mut iter = line.split_whitespace();

		let pos = Point::parse(iter.next().unwrap().strip_prefix("p=").unwrap());
		let vel = Point::parse(iter.next().unwrap().strip_prefix("v=").unwrap());

		bots.push((pos.unwrap(), vel.unwrap()));
	}
	let mut buf: String = String::new();	
	for n in 1..10000 {

		// stdin().read_line(& mut buf).unwrap();
		move_bots(& mut bots, &size, n == 100, & mut quadrants);
		println!("ITERATION : {n}");
		print_bots(&bots, &size);
	}
	
	let mut ret = 1;
	quadrants.iter().for_each(|h| ret *= h.len());

	i128::try_from(ret).expect("woopsie")
}

fn print_bots(bots: &[(Point, Point)], size: &Point) {
	let mut out: Vec<Vec<u8>> = vec![vec!['.' as u8; size.x as usize]; size.y as usize];

	for (pos, _vel) in bots {
		out[pos.y as usize][pos.x as usize] = 'X' as u8;
	}

	for line in &out {
		let line_str = str::from_utf8(line).unwrap();
		println!("{line_str}");
	}
}

fn move_bots(bots: & mut [(Point, Point)], size: &Point, save_to_quadrant: bool, quadrants: & mut[HashSet<usize>]) {
	for (index, (pos, vel)) in bots.iter_mut().enumerate() {
		*pos += *vel;
		pos.x %= size.x;
		pos.y %= size.y;

		if pos.x < 0 {
			pos.x = size.x + pos.x;
		}
		if pos.y < 0 {
			pos.y = size.y + pos.y;
		}

		if true == save_to_quadrant {
			if let Some(i) = get_quadrant(pos, size) {
				quadrants[i].insert(index);
			}
		}
	}
}

fn get_quadrant (pos: &Point, size: &Point) -> Option<usize> {
	if pos.x < 0 || pos.y < 0 {
		return None;
	}

	if pos.x < size.x / 2 {
		if pos.y < size.y / 2 {
			//TOP LEFT
			return Some(0);			
		} else if pos.y > size.y / 2 {
			//BOT LEFT
			return Some(1);
		}
	}
	else if pos.x > size.x / 2 {
		if pos.y < size.y / 2 {
			//TOP RIGHT
			return Some(2);
			
		} else if pos.y > size.y / 2 {
			//BOT RIGHT
			return Some(3);
		}
	}
	None
}
