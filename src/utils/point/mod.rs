use std::fmt;
use std::ops;
use std::path::Display;
use std::str::FromStr;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Vec2<T>
{
	x: T,
	y: T,
}

impl<T> Vec2<T> {
	fn new(x: T, y: T) -> Vec2<T> {
		Vec2 {x, y}
	}

}

#[derive(Debug, PartialEq, Eq)]
struct ParseVec2Error;

impl<T> FromStr for Vec2<T> 
where
	T: FromStr	
{
	type Err = ParseVec2Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (x, y) = s.split_once(',')
		.ok_or(ParseVec2Error)?;

		let x_fromstr = x.parse::<T>().map_err(|_| ParseVec2Error)?;	
		let y_fromstr = y.parse::<T>().map_err(|_| ParseVec2Error)?;

		Ok(Vec2::new(x_fromstr, y_fromstr))
	}
}

impl<T> fmt::Display for Vec2<T>
where
	T: fmt::Display
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}, {}", self.x, self.y)
	}
}

impl<T> ops::Mul<T> for Vec2<T> 
where
	T: ops::Mul<Output = T> + Clone
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		Self {
			x: self.x * rhs.clone(),
			y: self.y * rhs,
		}
	}
}

impl<T> ops::Add<Self> for Vec2<T>
where
	T: ops::Add<Output = T>
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> ops::AddAssign<Self> for Vec2<T>
where
	T: ops::AddAssign
{
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> ops::Sub<Self> for Vec2<T>
where
	T: ops::Sub<Output = T>
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> ops::SubAssign<Self> for Vec2<T>
where
	T: ops::SubAssign
{
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}
