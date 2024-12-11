use std::collections::VecDeque;
use cached::proc_macro::cached;


#[cached]
fn simulate(num: i64, blinks: i64) -> i64 {
	if blinks == 0 {
		return 1;
	}

	if num == 0 {
		return simulate(1, blinks - 1);
	} 
	let log10 = num.ilog10() + 1;
	if log10 % 2 == 0 {
		let left_split = num / 10_i64.pow(log10 / 2);
		let righ_split = num % 10_i64.pow(log10 / 2);
		return simulate(left_split, blinks - 1)
		+ simulate(righ_split, blinks - 1);
	} else {
		return simulate(num * 2024,  blinks - 1);
	}
}

pub fn solve(input: &str) -> i128 {
	let numbers: VecDeque<i64>;
	let mut ret = 0;
	let blinks = 75;

	numbers = input.lines().next().unwrap()
	.split_whitespace()
	.map(|num| num.parse::<i64>().expect("only numbers in input files"))
	.collect();

	for n in &numbers {
		ret += simulate(*n, blinks) as i128;
	}

	ret
}
