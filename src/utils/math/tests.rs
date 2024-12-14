use super::Vec2;

#[test]
fn add() {
	let a = Vec2::<i64>::new(10, 10);
	let b = Vec2::<i64>::new(20, 20);

	assert_eq!(b, a + a)
}
