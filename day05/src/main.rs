use std::{cmp::Ordering, collections::{HashMap, HashSet}};

// https://adventofcode.com/2024/day/5


fn main() {
	let input = include_str!("../input.txt");


	println!("RET1: {}", solve1(input));
	println!("RET2: {}", solve2(input));
}

fn solve1 (input: &str) -> i128
{
	let mut ret: i128 = 0;
	let mut lines = input.lines().peekable();
	let mut before: HashMap<i32, HashSet<i32>> = HashMap::new();
	let mut after: HashMap<i32, HashSet<i32>> = HashMap::new();

	//parse rules
	while lines.peek().unwrap().len() != 0 {
		let line = lines.next().unwrap();

		let mut nums = line.split('|')
			.map(|s| s.parse::<i32>().unwrap());
		
		let num1 = nums.next().unwrap();
		let num2 = nums.next().unwrap();

		before.entry(num2).or_insert(HashSet::new()).insert(num1);
		after.entry(num1).or_insert(HashSet::new()).insert(num2);
	}

	lines.next(); //skipping empty separation line
	//parse lists

	for line in lines {
		let nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

		if is_valid(&before, &after, &nums) {
			ret += nums[nums.len() / 2] as i128;
		}
	}

	ret
}

fn solve2 (input: &str) -> i128
{
	let mut ret: i128 = 0;
	let mut lines = input.lines().peekable();
	let mut before: HashMap<i32, HashSet<i32>> = HashMap::new();
	let mut after: HashMap<i32, HashSet<i32>> = HashMap::new();

	//parse rules
	while lines.peek().unwrap().len() != 0 {
		let line = lines.next().unwrap();

		let mut nums = line.split('|')
			.map(|s| s.parse::<i32>().unwrap());
		
		let num1 = nums.next().unwrap();
		let num2 = nums.next().unwrap();

		before.entry(num2).or_insert(HashSet::new()).insert(num1);
		after.entry(num1).or_insert(HashSet::new()).insert(num2);
	}

	lines.next(); //skipping empty separation line
	//parse lists
	for line in lines {
		let mut nums: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
		
		if !is_valid(&before, &after, &nums) {

			nums.sort_by(|a, b| {
				comp_by_rules(a, b, &before, &after)
			});

			ret += nums[nums.len() / 2] as i128;
		}
	}

	ret
}


fn is_valid(before: & HashMap<i32, HashSet<i32>>, after: & HashMap<i32, HashSet<i32>>, nums: & Vec<i32>) -> bool {
	nums.is_sorted_by( |a, b| comp_by_rules(a, b, before, after) == Ordering::Less)
}

fn comp_by_rules(a: &i32, b: &i32, before: &HashMap<i32, HashSet<i32>>, after: &HashMap<i32, HashSet<i32>>) -> Ordering {
	if
		after[a].contains(b)
	||	before[b].contains(a) {
		return Ordering::Less;
	}
	else if 
		after[b].contains(a)
	||	before[a].contains(b) {
		return Ordering::Greater
	}
	Ordering::Equal
}
