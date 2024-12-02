// use core::net;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		panic!("Missing filename !");
	}

	let file_path = &args[1];

	let mut left_column : Vec<i64> = Vec::new();
	let mut right_column : Vec<i64> = Vec::new();

	if let Ok(lines) = read_lines(file_path) {
		for line in lines
		{
			match line {
				Ok (line) => {
					let mut iter = line.split_whitespace();
					
					let left_nbr = iter
									.next()
									.unwrap()
									.parse::<i64>()
									.unwrap();
					left_column.push(left_nbr);

					let right_nbr = iter
									.next()
									.unwrap()
									.parse::<i64>()
									.unwrap();
					right_column.push(right_nbr);
				}
				Err (e) => {
					println!("Line error : {e:?}");
				}
			}
		}
		right_column.sort();
		left_column.sort();
		let mut total_dist : i64 = 0;

		let mut chunk_iter = right_column.chunk_by(|a, b| a == b);
		let mut current_chunk = chunk_iter.next();

		let mut left_iter = left_column.iter();
		let mut left_n: Option<&i64> = left_iter.next();

		while left_n.is_some() && current_chunk.is_some()  {
			if *left_n.unwrap() < current_chunk.unwrap()[0] {
				left_n = left_iter.next();
			}
			else if *left_n.unwrap() == current_chunk.unwrap()[0] {
				total_dist += left_n.unwrap() * current_chunk.unwrap().len() as i64;
				left_n = left_iter.next();
			}
			else if *left_n.unwrap() > current_chunk.unwrap()[0] {
				current_chunk = chunk_iter.next();
			}
		}
		println!("TOTAL DIST : {total_dist}");
	}

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
	where P: AsRef<Path>,
{
	let file= File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
