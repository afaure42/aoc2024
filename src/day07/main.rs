
mod solve2;	
mod solve1;


fn main() {
	let input = include_str!("input.txt");


	println!("RET1: {}", solve1::solve(input));
	println!("RET2: {}", solve2::solve(input));
}
