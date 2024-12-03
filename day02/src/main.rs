// use core::num;
use std::str::Lines;
// use std::collections::VecDeque;

fn main() {
	let input = include_str!("../input.txt");


	println!("RET1: {}", solve1(input.lines()));
	println!("RET2: {}", solve2(input.lines()));
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// 	where P: AsRef<Path>,
// {
// 	let file= File::open(filename)?;
// 	Ok(io::BufReader::new(file).lines())
// }

fn solve1 (lines : Lines) -> i128
{
	let mut ret = 0;
	for line in lines
	{
		let mut numbers: Vec<i64> = Vec::new();

		for n in line.split_whitespace() {
			numbers.push(n.parse::<i64>().expect("There should only be numbers in file"));
		}

		if test_vec(&numbers) {
			ret += 1;
		}
		
	}
	return ret;
}


fn solve2 (lines : Lines) -> i128
{
	let mut ret = 0;
	// let mut line_nbr = 1;
	for line in lines
	{
		let mut numbers: Vec<i64> = Vec::new();

		for n in line.split_whitespace() {
			numbers.push(n.parse::<i64>().expect("There should only be numbers in file"));
		}

		if test_vec(& numbers) {
			ret += 1;
			continue;
		}
		for n in 0..numbers.len()
		{
			let mut num_cpy = numbers.clone();
			num_cpy.remove(n);

			if test_vec(& num_cpy) {
				ret += 1;
				break;
			}
		}
		// else {
		// 	let incr_vec: Vec<&[i64]> = incr.collect();
		// 	let decr_vec: Vec<&[i64]> = decr.collect();

		// 	if incr_vec.len() == 2 {
		// 		println!("incr Line : {line_nbr}, {incr_vec:?}");
		// 	}
		// 	if decr_vec.len() == 2 {
		// 		println!("Decr Line : {line_nbr}, {decr_vec:?}");
		// 	}

		// 	if potential_valid(&incr_vec) {
		// 		ret += 1;
		// 	} else if potential_valid(&decr_vec) {
		// 		ret += 1;
		// 	}
		// }
		// line_nbr += 1;
	}
	return ret;
}

fn test_vec (numbers: & Vec<i64>) -> bool {
	let mut incr = numbers.chunk_by(|a: &i64, b: &i64| a < b && b - a <= 3).peekable();
	let mut decr = numbers.chunk_by(|a: &i64, b: &i64| a > b && a - b <= 3).peekable();

	return decr.peek().unwrap().len() == numbers.len()
	|| incr.peek().unwrap().len() == numbers.len()
}
