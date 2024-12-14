use crate::utils::math::ParseVec2Error;

use super::Vec2;

#[test]
fn add() {
	let a = Vec2::<i64>::new(10, 10);
	let b = Vec2::<i64>::new(20, 20);

	assert_eq!(b, a + a)
}

#[test]
fn parse_i32() {
	let a = "30,30".parse::<Vec2<i32>>().unwrap();

	assert_eq!(a, Vec2::<i32>::new(30, 30));
}

#[test]
fn parse_error() {
	assert_eq!("30,".parse::<Vec2<i32>>(), Err(ParseVec2Error));
}
