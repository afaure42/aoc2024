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

fn find_intersection(a: &Point, b: &Point, prize: &Point) -> Point {
	let a_slope: f64 = a.y as f64 / a.x as f64;
	let b_slope: f64 = b.y as f64 / b.x as f64;
	if (a_slope - b_slope).abs() < 0.001 {
		panic!("FOUND CO LINEAR IN INPUT");
	}
	
	//calculate y origin for a and b
	//a is starting from 0,0
	//we only know that b is going through the prize at some point
	// so the equation
	// y = slope * x + origin
	//is equal to 
	// prize_y = slope_b * prize_z + origin
	//with some reorganisation with come up with this equation to find origin of b
	// origin = -(b_slope * prize_x) + prize_y
	let y_origin_a: f64 = 0.0; //a is starting from origin
	let y_origin_b = -(b_slope * prize.x as f64) + prize.y as f64;

	let mut intersect= Point::new(0, 0);

	//calculate intersection point of the two lines created by the a and b vector
	//imagine that vector a originates from 0,0
	// the formula used can be found here
	//https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
	intersect.x = ((y_origin_a - y_origin_b) / (b_slope - a_slope)).round() as i64;
	intersect.y = (intersect.x as f64 * a_slope).round() as i64;

	intersect
}

fn solve_machine(machine_str: &str) -> Option<i64> {

	let mut iter = machine_str.lines();

	let button_a = parse_pos(iter.next().unwrap());
	let button_b = parse_pos(iter.next().unwrap());
	let mut prize = parse_pos(iter.next().unwrap());

	prize.x += 10000000000000;
	prize.y += 10000000000000;

	let intersect = find_intersection(&button_a, &button_b, &prize);
	if intersect.x < 0 || intersect.y < 0 {
		return None;
	}

	let intersect_to_prize = prize - intersect;
	
	if intersect.x % button_a.x != 0
		|| intersect_to_prize.x % button_b.x != 0 {
		return None;
	}

	let a_pressed = intersect.x / button_a.x;
	let b_pressed = intersect_to_prize.x / button_b.x;
	Some(a_pressed * 3 + b_pressed)
}
