use lib::utils::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
	Empty,
	Wall,
	PackageLeft,
	PackageRight,
	Bot
}

pub fn solve(input: &str) -> i128 {
	let dirs: [Vec2<i32>; 4] = [
		Vec2::new(0, -1),
		Vec2::new(0, 1),
		Vec2::new(-1, 0),
		Vec2::new(1, 0)
	];
	let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
	let mut bot_pos: Vec2<i32> = Vec2::new(0 ,0);

	let mut grid: Vec<Vec<Cell>> = Vec::new();
	let mut moves: Vec<usize> = Vec::new();

	//parse grid
	for (line_nbr, line) in grid_input.lines().enumerate() {
		let mut new_line = Vec::<Cell>::new();
		for (index, c) in line.chars().enumerate() {
			match c {
				'#' => {
					new_line.push(Cell::Wall);
					new_line.push(Cell::Wall);
				}
				'.' => {
					new_line.push(Cell::Empty);
					new_line.push(Cell::Empty);
				}
				'O' => {
					new_line.push(Cell::PackageLeft);
					new_line.push(Cell::PackageRight);
				}
				'@' => {
					new_line.push(Cell::Bot);
					new_line.push(Cell::Empty);
					bot_pos = Vec2::<i32>::new((index * 2) as i32, line_nbr as i32);
				}
				_ => panic!("INVALID CHAR IN GRID")
			}
		}
		grid.push(new_line);
	}

	for line in moves_input.lines() {
		for c in line.chars() {
			match c {
				'^' => moves.push(0),
				'v' => moves.push(1),
				'<' => moves.push(2),
				'>' => moves.push(3),
				_ => panic!("INVALID CHAR IN MOVE INPUT")
			}
		}
	}

	for action in moves {
		let dir = dirs[action];
		if true == test_move(&mut grid, &bot_pos, &dir) {
			apply_move(&mut grid, &bot_pos, &dir);
			bot_pos = bot_pos + dir;
		}
		// print_grid(&grid);
	}

	let mut ret = 0;

	for (y, line) in grid.iter().enumerate() {
		for (x, cell) in line.iter().enumerate() {
			if *cell == Cell::PackageLeft {
				ret += y * 100 + x;
			}
		}
	}

	ret as i128
}

fn print_grid(grid: & Vec<Vec<Cell>>) {
	for line in grid {
		let mut line_str = String::new();

		for cell in line {
			match cell {
				Cell::Bot => line_str.push('@'),
				Cell::Wall => line_str.push('#'),
				Cell::PackageLeft => line_str.push('['),
				Cell::PackageRight => line_str.push(']'),
				Cell::Empty => line_str.push('.'),
			}
		}
		println!("{line_str}");
	}
}

fn apply_move(grid: &mut Vec<Vec<Cell>>, pos: &Vec2<i32>, dir: &Vec2<i32>) {
	use Cell::*;
	let target_pos = *pos + *dir;

	let target_cell = grid[target_pos.y as usize][target_pos.x as usize];

	match target_cell {
		Wall => panic!("CANNOT MOVE INTO A WALL"),
		Bot => panic!("CANNOT MOVE INTO ROBOT"),
		Empty => {},
		PackageLeft => {
			//going up or down
			if dir.y != 0 {
				//move package right as well
				let right_target = target_pos + Vec2::new(1, 0);
				apply_move(grid, &right_target, dir);
			}
			apply_move(grid, &target_pos, dir);

		},
		PackageRight => {
			//going up or down
			if dir.y != 0 {
				//move package right as well
				let left_target = target_pos + Vec2::new(-1, 0);
				apply_move(grid, &left_target, dir);
			}
			apply_move(grid, &target_pos, dir);
		}
	}

	let swap_target = grid[target_pos.y as usize][target_pos.x as usize];
	let swap_current = grid[pos.y as usize][pos.x as usize];

	grid[target_pos.y as usize][target_pos.x as usize] = swap_current;
	grid[pos.y as usize][pos.x as usize] = swap_target;
}

fn test_move(grid: & mut Vec<Vec<Cell>>, pos: &Vec2<i32>, dir: &Vec2<i32>) -> bool {
	use Cell::*;
	let target_pos = *pos + *dir;

	let target_cell = grid[target_pos.y as usize][target_pos.x as usize];
	
	let status: bool;
	
	match target_cell {
		Wall => status = false,
		PackageLeft => {
			// going left or right
			if dir.y == 0 {
				status = test_move(grid, &target_pos, dir)
			}
			//going up or down
			else {
				let right_target = target_pos + Vec2::new(1, 0);
				status = test_move(grid, &target_pos, dir) && test_move(grid, &right_target, dir);
			}
		}
		PackageRight => {
			// going left or right
			if dir.y == 0 {
				status = test_move(grid, &target_pos, dir)
			}
			//going up or down
			else {
				let left_target = target_pos + Vec2::new(-1, 0);
				status = test_move(grid, &target_pos, dir) && test_move(grid, &left_target, dir);
			}
		}
		Empty => {
			status = true
		}
		Bot	=> panic!("TARGET IS BOT, NOT POSSIBLE")
		
	}
	// println!("AT POS {pos}, status is {status}");
	status
}
