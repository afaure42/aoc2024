use std::env;
mod solve1;
mod solve2;

fn main() {
	let args: Vec<_> = env::args().collect();
	let input;

	if args.len() > 1
	&& args[1] == "test" {
		input  = include_str!("input_bis.txt");
	} else {
		input = include_str!("input.txt");
	}
	

	println!("RET1: {}", solve1::solve(input));
	println!("RET2: {}", solve2::solve(input));
}
