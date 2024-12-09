use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Type {
	File(u32),
	Free
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Part {
	t: Type,
	size: usize
}

impl fmt::Debug for Type {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Type::Free => write!(f, "F"),
			Type::File(n) => write!(f, "{n}")
		}
	}
}

impl fmt::Debug for Part {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{:?}, {}]", self.t, self.size)
	}
}

pub fn solve(input: &str) -> i128 {
	let mut disk: Vec<Part> = Vec::new();
	let line = input.lines().next().expect("expected at least one line in file");

	let mut id = 0;
	let mut free_space = false;
	for c in line.chars() {
		let size = c.to_digit(10).expect("Only digits can be in input file");

		if free_space {
				disk.push(Part {t: Type::Free, size: size as usize});
		} else {
				disk.push(Part {t: Type::File(id), size: size as usize});
			id += 1;
		}

		free_space = !free_space;
	}

	
	compress(& mut disk, id - 1);

	checksum(&disk) as i128
}

fn compress(disk: & mut Vec<Part>, highest_id: u32) {
	// let mut first_free = find_first_free(&disk, 0).expect("No free in input");
	for n in (0..=highest_id).rev() {
		let index = disk.iter().enumerate().find(|p| p.1.t == Type::File(n)).expect("Part not found").0;
		let part_size = disk[index].size;
		if let Some(first_free) = find_first_free(disk, part_size) {
			if first_free > index {
				continue;
			}
			let free_size = disk[first_free].size;
			
			disk.swap(first_free, index);

			if free_size > part_size {
				disk[index].size = part_size;
				disk.insert(first_free + 1, Part {t: Type::Free, size: free_size - part_size});
			}

		}
	}
}

fn checksum (disk: &[Part]) -> usize {
	let mut ret = 0;
	let mut index = 0;
	for p in disk.iter() {
		match p.t {
			Type::File(n) => {
				for i in 0..p.size {
					ret += (index + i as usize) * n as usize;
				}
			}
			Type::Free => {}
		}

		index += p.size;
	}
	ret
}

fn find_first_free (parts: &[Part], size: usize) -> Option<usize> {
	parts.iter()
	.enumerate()
	.find(|(_index, p)| matches!(p.t, Type::Free) && p.size >= size)
	.unzip()
	.0
}

// fn find_last_num (parts: &[Part], start: usize) -> Option<usize> {
// 	parts.iter()
// 	.enumerate()
// 	.rev()
// 	.skip(start)
// 	.find(|(_index, p)| matches!(p.t, Type::File(_)) )
// 	.unzip()
// 	.0
// }
