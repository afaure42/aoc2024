

#[derive(Clone, Copy, Debug)]
enum OP {
	MUL,
	ADD,
}

pub fn solve (input : &str) -> u128
{
	let mut ret = 0;
	for line in input.lines() {
		let (res_str, nums_str) = line.split_once(':').expect("Wrong file Format");

		let res = res_str.parse::<u128>().expect("Wrong file format");
		let nums: Vec<u128> = nums_str
				.split_whitespace()
				.map(|s| s.parse::<u128>().expect("Wrong File format"))
				.collect();
		let mut operations = vec![OP::MUL; nums.len() - 1];
		// println!("NUMS : {:?}", nums);
		// println!("LIMITS: {res}");
		if is_valid(&res, &nums, & mut operations, 0) {
			// println!("FOUND VALID FOR {:?}", nums);
			ret += res as u128;
		}
	}
	
	ret
}

fn is_valid (res: &u128, nums: &Vec<u128>, operations: & mut Vec<OP>, depth: usize) -> bool {
	// let indent = "\t".repeat(depth);
	// println!("{indent}OP: {:?} DEPTH: {depth}", operations);
	//fail stop condition
	if depth == operations.len() {
		if let Some(calc) = do_op(nums, operations, res) {
			return *res == calc;
		} else {
			return false;
		}
	}

	//success stop condition

	//try with current combination but deeper depth
	if is_valid(res, nums, operations, depth + 1) { return true }

	//mutate current op as much as possible
	let start = operations[depth];
	let mut optionnal = next(&operations[depth]);

	while let Some(op) = optionnal {
		operations[depth] = op;
		// println!("{indent}MUTATING TO {:?}", op);
		if is_valid(res, nums, operations, depth + 1) { return true }
		optionnal = next(&op);
	}
	operations[depth] = start;

	false
}

fn do_op (nums: & Vec<u128>, operations: & Vec<OP>, limit: &u128) -> Option<u128> {
	let mut ret = nums[0];
	// println!("TRY :{:?} WITH OPS {:?}", nums, operations);
	for (index, op) in operations.iter().enumerate() {
		// println!("{:?}", ret);
		let num = nums[index + 1];
		match op {
			OP::ADD => ret += num,
			OP::MUL => ret *= num,
		}
		if ret > *limit {return None}
	}
	// println!("res = {ret}");
	Some(ret)
}

fn next (e: & OP) -> Option<OP> {
	match e {
		OP::MUL =>	Some(OP::ADD),
		OP::ADD =>	None,
	}
}
