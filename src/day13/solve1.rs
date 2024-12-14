use std::fmt;
use std::ops;
use regex::Regex;
use std::cmp;

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

fn test_buttons(a: &Point, b: &Point, prize: &Point) -> Option<(i64, i64)> {
	//this is the number of times we can push button a without overshooting the prize
	//Imagine a prize at 11, 11
	//and a button that does 2, 3
	// you get 11 / 2 = 5, 11 / 3 = 3
	//you can only push the button three times without overshooting
	//you'll be at 6, 9
	let mut b_in_prize = cmp::max(prize.x / b.x, prize.y / b.y);
	let mut a_in_prize = 0;
	while b_in_prize >= 0 && a_in_prize <= 100{
		let test = *a * a_in_prize + *b * b_in_prize;

		if test == *prize {
			return Some((a_in_prize, b_in_prize));
		}

		if test.x < prize.x && test.y < prize.y {
			a_in_prize += 1;
		}
		else if test.x >= prize.x || test.y >= prize.y {
			b_in_prize -= 1;
		}
	}

	None
}

fn solve_machine(machine_str: &str) -> Option<i64> {

	let mut iter = machine_str.lines();

	let button_a = parse_pos(iter.next().unwrap());
	let button_b = parse_pos(iter.next().unwrap());
	let prize = parse_pos(iter.next().unwrap());

	// println!("{button_a}");
	// println!("{button_b}");
	// println!("{prize}");

	if let Some(ab) = test_buttons(&button_a, &button_b, &prize){
		// println!("Found :{:?}", ab);
		return Some(ab.0 * 3 + ab.1);
	} else {
		// println!("NO SOLUTION");
	}
	None
}
