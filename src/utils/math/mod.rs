use std::fmt;
use std::ops;
use std::str::FromStr;
use std::convert::From;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Debug)]
pub struct Vec2<T>
{
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	pub fn new(x: T, y: T) -> Vec2<T> {
		Vec2 {x, y}
	}

}

#[derive(Debug)]
pub struct ConvertVec2Error;

pub trait Convert<T>: Sized {
	fn convert(value:T) -> Result<Self, ConvertVec2Error>;
}


impl<T, U> Convert<Vec2<T>> for Vec2<U>
where
	U: TryFrom<T>
 {
	fn convert(value:Vec2<T>) -> Result<Self, ConvertVec2Error> {
		Ok(Vec2 {
			x: U::try_from(value.x).map_err(|_| ConvertVec2Error)?,
			y: U::try_from(value.y).map_err(|_| ConvertVec2Error)?
		})
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVec2Error;

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
	T: ops::Mul<Output = T> + Copy
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		Self {
			x: self.x * rhs,
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
