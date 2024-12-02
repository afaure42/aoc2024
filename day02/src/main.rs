use std::str::Lines;

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

		let mut incr = numbers.chunk_by(|a: &i64, b: &i64| a < b && b - a <= 3);
		let mut decr = numbers.chunk_by(|a: &i64, b: &i64| a > b && a - b <= 3);

		if decr.next().unwrap().len() == numbers.len()
			|| incr.next().unwrap().len() == numbers.len() {
				ret += 1;
			}

	}
	return ret;
}


fn solve2 (lines : Lines) -> i128
{
	let mut ret = 0;
	for line in lines
	{
		let mut numbers: Vec<i64> = Vec::new();

		for n in line.split_whitespace() {
			numbers.push(n.parse::<i64>().expect("There should only be numbers in file"));
		}

		let mut incr = numbers.chunk_by(|a: &i64, b: &i64| a < b && b - a <= 3);
		let mut decr = numbers.chunk_by(|a: &i64, b: &i64| a > b && a - b <= 3);

		if decr.next().unwrap().len() == numbers.len()
		|| incr.next().unwrap().len() == numbers.len() {
			ret += 1;
		}
		else {
			let incr_vec: Vec<&[i64]> = incr.collect();
			let decr_vec: Vec<&[i64]> = decr.collect();

			if potential_valid(&incr_vec) {
				ret += 1;
			} else if potential_valid(&decr_vec) {
				ret += 1;
			}
		}
	}
	return ret;
}

fn potential_valid (numbers: &Vec<&[i64]>) -> bool{

	if numbers.len() == 2 {
		let first_len = numbers[0].len();
		let second_len = numbers[1].len();

		if first_len == 1 || second_len == 1 {
			println!("Valid because first or last is wrong: {numbers:?}");
			return true;
		}
		// now we now we have at least [a, b][c, d]
		//test distance between a and c, b and d
		// to check if b or c could be removed
		let a: i64 = numbers[0][first_len - 2];
		let b: i64 = numbers[0][first_len - 1];
		let c: i64 = numbers[1][second_len - 2];
		let d: i64 = numbers[1][second_len - 1];

		if a.abs_diff(c) <= 3{
			return true;

		}
		if b.abs_diff(d) <= 3{
			return true;
		}
	}
	return false;
}
