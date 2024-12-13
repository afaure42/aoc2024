use std::fmt;
use std::ops;
use regex::Regex;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Point {
	x: i64,
	y: i64,
}

impl Point {
	fn new(x: i64, y: i64) -> Point {
		Point {x, y}
	}

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
	let machines = input.split("\n\n");
	let mut ret = 0;

	for machine in machines {
		if let Some(res) = solve_machine(machine) {
			ret += i128::try_from(res).expect("up cast failed");
		}
	}
	ret
}

fn parse_pos(line: &str) -> Point {
	let re = Regex::new(r"[XY][+=]\d*").unwrap();
	let mut ret: Point = Point::new(0, 0);

	let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

	ret.x =	matches[0][2..].parse::<i64>().unwrap();
	ret.y =	matches[1][2..].parse::<i64>().unwrap();

	ret
}

fn test_buttons(a: &Point, b: &Point, prize: &Point) -> (i64, i64) {
	let (div, rem) = 

}

fn solve_machine(machine_str: &str) -> Option<i32> {

	let mut iter = machine_str.lines();

	let button_a = parse_pos(iter.next().unwrap());
	let button_b = parse_pos(iter.next().unwrap());
	let prize = parse_pos(iter.next().unwrap());

	println!("{button_a}");
	println!("{button_b}");
	println!("{prize}");


	Some(0)
}
