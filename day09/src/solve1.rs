
enum Part {
	File(u32),
	Free
}

pub fn solve(input: &str) -> i128 {
	let mut disk: Vec<Part> = Vec::new();
	let line = input.lines().next().expect("expected at least one line in file");

	let mut id = 0;
	let mut free_space = false;
	for c in line.chars() {
		let size = c.to_digit(10).expect("Only digits can be in input file");

		if free_space {
			for _ in 0..size {
				disk.push(Part::Free);
			}
		} else {
			for _ in 0..size {
				disk.push(Part::File(id));
			}
			id += 1;
		}

		free_space = !free_space;
	}

	compress(&mut disk);
	checksum(&disk) as i128
}


fn compress(disk: & mut Vec<Part>) {
	let mut first_free = find_first_free(&disk, 0).expect("No free in input");
	let mut last_num = find_last_num(&disk, 0).expect("No num in input");				

	//commpression 
	while first_free < last_num {
		disk.swap(first_free, last_num);

		while first_free < last_num
		&& !matches!(disk[first_free], Part::Free) {
			first_free += 1;
		}

		while last_num > 0 
		&& ! matches!(disk[last_num], Part::File(_)) {
			last_num -= 1;
		}
	}
}

fn checksum (disk: &[Part]) -> usize {
	let mut ret = 0;
	for (index, p) in disk.iter().enumerate() {
		match p {
			Part::Free => continue,
			Part::File(n) => ret += index * *n as usize
		}
	}
	ret
}

fn find_first_free (parts: &[Part], start: usize) -> Option<usize> {
	parts.iter()
	.enumerate()
	.skip(start)
	.find(|(_index, p)| matches!(p, Part::Free))
	.unzip()
	.0
}

fn find_last_num (parts: &[Part], start: usize) -> Option<usize> {
	parts.iter()
	.enumerate()
	.rev()
	.skip(start)
	.find(|(_index, p)| matches!(p, Part::File(_)) )
	.unzip()
	.0
}
