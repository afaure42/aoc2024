struct Registers {
	a: i32,
	b: i32,
	c: i32,
	ip: usize //instruction pointer
}

fn do_adv(registers: & mut Registers, operand: i32) {
	let numerator = registers.a;
	let denominator = 2_i32.pow(fetch_combo(registers, operand) as u32);

	registers.a = numerator / denominator;

	registers.ip += 2;
}

fn do_bxl(registers: & mut Registers, operand: i32) {
	registers.b = registers.b ^ operand;

	registers.ip += 2;
}

fn do_bst(registers: & mut Registers, operand: i32) {
	let modulo = fetch_combo(registers, operand) & 0b111;

	registers.b = modulo;

	registers.ip += 2;
}

fn do_jnz(registers: & mut Registers, operand: i32) {
	if registers.a != 0 {
		registers.ip = operand as usize;
	} else {
		registers.ip += 2;
	}
}

fn do_bxc(registers: & mut Registers, _operand: i32) {
	registers.b = registers.b ^ registers.c;

	registers.ip += 2;
}

fn do_out(registers: & mut Registers, operand: i32) {
	let value = fetch_combo(registers, operand) & 0b111;
	print!("{value},");

	registers.ip += 2;
}

fn do_bdv(registers: & mut Registers, operand: i32) {
	let numerator = registers.a;
	let denominator = 2_i32.pow(fetch_combo(registers, operand) as u32);

	registers.b = numerator / denominator;

	registers.ip += 2;
}

fn do_cdv(registers: & mut Registers, operand: i32) {
	let numerator = registers.a;
	let denominator = 2_i32.pow(fetch_combo(registers, operand) as u32);

	registers.c = numerator / denominator;

	registers.ip += 2;
}


struct Instruction {
	operation: i32,
	operand: i32,
}

fn fetch_combo(registers: & Registers, operand: i32) -> i32 {
	match operand {
		0 => 0,
		1 => 1,
		2 => 2,
		3 => 3,
		4 => registers.a,
		5 => registers.b,
		6 => registers.c,
		_ => panic!("INVALID OPERAND")
	}
}


fn do_op(registers: & mut Registers, instruction: &Instruction) {
	let operand = instruction.operand;
	match instruction.operation {
		0 => do_adv(registers, operand),
		1 => do_bxl(registers, operand),
		2 => do_bst(registers, operand),
		3 => do_jnz(registers, operand),
		4 => do_bxc(registers, operand),
		5 => do_out(registers, operand),
		6 => do_bdv(registers, operand),
		7 => do_cdv(registers, operand),
		_ => panic!("")
	}
}

pub fn solve(input: &str) -> i128 {
	let memory: Vec<i32>;
	let mut registers = Registers {a: 0, b: 0, c: 0, ip: 0};

	let mut iter = input.lines();
	//register A
	registers.a = iter.next().unwrap()
		.strip_prefix("Register A: ")
		.unwrap()
		.parse()
		.unwrap();
	registers.b = iter.next().unwrap()
		.strip_prefix("Register B: ")
		.unwrap()
		.parse()
		.unwrap();
	registers.c = iter.next().unwrap()
		.strip_prefix("Register C: ")
		.unwrap()
		.parse()
		.unwrap();
	
	//skip empty line
	iter.next();

	memory = iter.next().unwrap()
	.strip_prefix("Program: ")
	.unwrap()
	.split(',')
	.map(|s| s.parse::<i32>().unwrap())
	.collect();

	while registers.ip < memory.len() {
		let instruction = Instruction {operation: memory[registers.ip], operand: memory[registers.ip + 1]};
		do_op(& mut registers, &instruction);
	}
	print!("\n");

	0
}
