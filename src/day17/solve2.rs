struct Registers {
	a: u128,
	b: u128,
	c: u128,
	ip: usize, //instruction pointer
	output: Vec<u8>
}

impl Registers {
	fn new() -> Self {
		Self { a: 0, b: 0, c: 0, ip: 0, output: Vec::with_capacity(16)}
	}

	fn clear(& mut self) {
		self.a = 0;
		self.b = 0;
		self.c = 0;
		self.ip = 0;
		self.output.clear();
	}
}

struct InstructionError;
struct InstructionValid;


fn do_adv(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	let numerator = registers.a;
	let denominator = 2_u128.pow(fetch_combo(registers, operand) as u32);

	registers.a = numerator / denominator;

	registers.ip += 2;
Ok(InstructionValid)
}

fn do_bxl(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	registers.b = registers.b ^ operand;

	registers.ip += 2;
	Ok(InstructionValid)
}

fn do_bst(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	let modulo = fetch_combo(registers, operand) & 0b111;

	registers.b = modulo;

	registers.ip += 2;
	Ok(InstructionValid)
}

fn do_jnz(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	if registers.a != 0 {
		registers.ip = operand as usize;
	} else {
		registers.ip += 2;
	}
	Ok(InstructionValid)
}

fn do_bxc(registers: & mut Registers, _operand: u128) -> Result<InstructionValid, InstructionError> {
	registers.b = registers.b ^ registers.c;

	registers.ip += 2;
	Ok(InstructionValid)
}

fn do_out(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	let value = fetch_combo(registers, operand) & 0b111;
	registers.output.push(value as u8);

	registers.ip += 2;
	Ok(InstructionValid)
}

fn do_bdv(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	let numerator = registers.a;
	let denominator = 2_u128.pow(fetch_combo(registers, operand) as u32);

	registers.b = numerator / denominator;

	registers.ip += 2;
	Ok(InstructionValid)
}

fn do_cdv(registers: & mut Registers, operand: u128) -> Result<InstructionValid, InstructionError> {
	let numerator = registers.a;
	let denominator = 2_u128.pow(fetch_combo(registers, operand) as u32);

	registers.c = numerator / denominator;

	registers.ip += 2;
	Ok(InstructionValid)
}


struct Instruction {
	operation: u128,
	operand: u128,
}

fn fetch_combo(registers: & Registers, operand: u128) -> u128 {
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


fn do_op(registers: & mut Registers, instruction: &Instruction) -> Result<InstructionValid, InstructionError> {
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



fn run_program(registers: & mut Registers, memory: &[u128]) {
	while registers.ip < memory.len() {
		let instruction = Instruction{operation: memory[registers.ip], operand: memory[registers.ip + 1]};
		if do_op(registers, &instruction).is_err() {
			break;
		}
	}
}

/*

	2, 4 = bst B = A % 8
	1, 5 = bxl B = B ^ 5 (0b101)
	7, 5 = cdv C = A / 32 
	1, 6 = bxl B = B ^ 6 (0b110)
	4, 2 = bxc B = B ^ C
	5, 5 = out register B % 8
	0, 3 = adv A = A / 8 (2 ^ 3) A MUST BE BIGGER THAN 8 TO KEEP GOING
	3, 0 = jnz Jump to 0 if A != 0

	the solution is a list of octal values [0b000][0b000][0b000]

	the programm goes from the least significant octal value to the most significant
	and each iteration it hashes and outputs a value

	more significant values have an influence over less significant values in the hashing function

	so we backtrack from the most significant value to the less significant value
	that's why the memory_index is not equal to the index
 */
fn recurse(solution: u128, index: usize, memory: &[u128]) -> Option<u128> {
	
	let memory_size = (memory.len() - 1) as usize;
	let memory_index = memory_size - index;
	let mut registers: Registers = Registers::new();

	for n in 0..8 {

		let test_solution = solution | n << memory_index * 3;
		registers.clear();
		registers.a = test_solution;
		run_program(&mut registers, memory);

		if	registers.output.len() >= memory_index
			&& registers.output[memory_index] == memory[memory_index] as u8 {
			if index == memory_size {
				return Some(test_solution);
			}
			if let Some(ret) = recurse(test_solution, index + 1, memory) {
				return Some(ret);
			}
		}
	}
	None
}

pub fn solve(input: &str) -> i128 {
	let memory: Vec<u128>;
	let mut registers = Registers {a: 0, b: 0, c: 0, ip: 0, output: Vec::new()};

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
	.map(|s| s.parse::<u128>().unwrap())
	.collect();

	let solution = recurse(0, 0, &memory).expect("No possible solutions for this program");

	solution as i128
}
