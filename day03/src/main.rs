
use regex::Regex;

// https://adventofcode.com/2024/day/3
fn main() {
	let input = include_str!("../input.txt");


	println!("RET1: {}", solve1(input.to_string()));
	println!("RET2: {}", solve2(input.to_string()));
}

fn solve1 (input: String) -> i128
{
	let mut ret: i128 = 0;
	let re = Regex::new("mul[(][0-9]{1,3},[0-9]{1,3}[)]").unwrap();
	let iter: Vec<_> = re.find_iter(&input).map(|m| m.as_str().to_string()).collect();

	for mut operation in iter {
		//we are getting strings in format "mul(%d,%d)"
		// println!("{operation}");
		operation = operation.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().to_string();
		let iter = operation.split(",");
		let mut total: i128 = 1;

		for n in iter {
			total *= n.parse::<i128>().unwrap()
		}

		ret += total;
	}
	return ret;
}

fn solve2 (input: String) -> i128
{
	let mut ret: i128 = 0;
	let re = Regex::new("mul[(][0-9]{1,3},[0-9]{1,3}[)]|do[(][)]|don't[(][)]").unwrap();
	let iter: Vec<_> = re.find_iter(&input).map(|m| m.as_str()).collect();

	let mut enable: bool = true;
	for operation in iter {
		//we are getting strings in format "mul(%d,%d)"
		// println!("{operation}");
		match operation {
			"do()" => enable = true,
			"don't()" => enable = false,
			_ => {
				if enable {
					ret += do_mul(operation.to_string());
				}
			}
		}
	}
	return ret;
}

fn do_mul (mut mul: String) -> i128 {
	mul = mul.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap().to_string();
	let iter = mul.split(",");
	let mut total: i128 = 1;

	for n in iter {
		total *= n.parse::<i128>().unwrap()
	}

	return total;
}
