use cached::proc_macro::cached;
use cached::SizedCache;

pub fn solve(input: &str) -> i128 {
	let mut iter = input.lines();


	let towels: Vec<_> = iter.next().unwrap()
		.split(',')
		.map(|string| if string.starts_with(' ') {string.strip_prefix(' ').unwrap()} else {string})
		.collect();

	iter.next(); // skip empty line

	let mut ret = 0;

	for towel in iter {
		ret += recurse(String::new(), towel, &towels);
	}

	ret as i128
}

#[cached(
    ty = "SizedCache<String, u128>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}{}", solution, target) }"#
)]
fn recurse(solution: String, target: &str, towels: &[&str]) -> u128 {
	// println!("Solution:\t{solution}\ntarget:\t\t{target}");
	let mut ret = 0;
	for towel in towels {
		let test_solution = solution.clone() + towel;

		if test_solution == target {
			// println!("Found possible");
			ret += 1
		}
		if test_solution.len() >= target.len() {continue} 
		if test_solution != target.split_at(test_solution.len()).0 {continue;}

		ret += recurse(test_solution, target, towels)
	}

	ret
}
