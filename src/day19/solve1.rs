pub fn solve(input: &str) -> i128 {
	let mut iter = input.lines();


	let towels: Vec<_> = iter.next().unwrap()
		.split(',')
		.map(|string| if string.starts_with(' ') {string.strip_prefix(' ').unwrap()} else {string})
		.collect();

	iter.next(); // skip empty line

	let mut ret = 0;

	for towel in iter {
		if recurse(String::new(), towel, &towels) {
			ret += 1;
		}
	}

	ret as i128
}

fn recurse(solution: String, target: &str, towels: &[&str]) -> bool {
	// println!("Solution:\t{solution}\ntarget:\t\t{target}");
	for towel in towels {
		let test_solution = solution.clone() + towel;

		if test_solution.len() > target.len() {continue} 
		if test_solution == target {return true}
		if test_solution != target.split_at(test_solution.len()).0 {continue;}

		if recurse(test_solution, target, towels) {
			return true
		}
	}

	false
}
