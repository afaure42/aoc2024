use core::fmt;
use std::fmt::{Debug, Display, Pointer};

use lib::utils::math::Vec2;

// #[derive(Debug, Clone)]
// enum PadContent {
// 	Code([i32; 4], usize),
// 	Pad(Box<DirPad>)
// }

// impl PadContent {
// 	fn inBounds(&self, pos: &Vec2<i32>) -> bool {
// 		use PadContent::*;
// 		if pos.y < 0 || pos.x < 0 {return false}

// 		match self {
// 			Code(_, _) => return pos.y < 4 && pos.x < 3,
// 			Pad(_) => return pos.y < 2 && pos.x < 3
// 		}
// 	}
// 	fn press(& mut self, pos: &Vec2<i32>) -> bool {
// 		use PadContent::*;
// 		if !self.inBounds(pos) {return false}

// 		match self {
// 			Code(c, i) => {
// 				if let Some(n) = PadContent::posToNum(pos) {
// 					c[*i] = n;
// 					return true
// 				}
// 				return false
// 			}
// 			Pad(p) => {
// 				return 
// 			}
// 		}

// 		None
// 	}
// 	fn posToNum(pos: &Vec2<i32>) -> Option<i32> {
// 		match pos {
// 			Vec2{x: 0, y: 0} => Some(7),
// 			Vec2{x: 1, y: 0} => Some(8),
// 			Vec2{x: 2, y: 0} => Some(9),
// 			Vec2{x: 0, y: 1} => Some(4),
// 			Vec2{x: 1, y: 1} => Some(5),
// 			Vec2{x: 2, y: 1} => Some(6),
// 			Vec2{x: 0, y: 2} => Some(1),
// 			Vec2{x: 1, y: 2} => Some(2),
// 			Vec2{x: 2, y: 2} => Some(3),
// 			Vec2{x: 0, y: 3} => None,
// 			Vec2{x: 1, y: 3} => Some(0),
// 			Vec2{x: 2, y: 3} => Some(10),
// 			_ => None
// 		}
// 	}
// }

// #[derive(Debug, Clone)]
// struct DirPad {
// 	pos: Vec2<i32>,
// 	content: PadContent
// }

// impl DirPad {
// 	fn new(pos: Vec2<i32>, content: PadContent) -> DirPad {
// 		DirPad {
// 			pos,
// 			content
// 		}
// 	}

// 	fn apply_dir(&self, dir: &Vec2<i32>) -> Option<Self> {
// 		let test_pos = self.pos + *dir;

// 		if !self.content.inBounds(&test_pos) {
// 			return None
// 		} else {
// 			return Some(DirPad::new(test_pos, self.content.clone()))
// 		}
// 	}

// 	fn press(&self) -> Option<Self> {

// 	}

// 	fn mut_press(& mut self) -> bool {

// 	}

// 	fn getCode(&self) -> [i32; 4] {

// 	}
// }

// enum Direction {
// 	UP,
// 	DOWN,
// 	LEFT,
// 	RIGHT
// }

// impl Direction {
// 	fn toVec2(&self) -> Vec2<i32> {
// 		use Direction::*;
// 		match self {
// 			UP => Vec2::new(0, -1),
// 			DOWN => Vec2::new(0, 1),
// 			LEFT => Vec2::new(-1, 0),
// 			RIGHT => Vec2::new(1, 0),
// 		}
// 	}
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
	DirAction(DirAction),
	CodeAction(CodeAction)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodeAction {
	ZERO,
	ONE,
	TWO,
	THREE,
	FOUR,
	FIVE,
	SIX,
	SEVEN,
	EIGHT,
	NINE,
	A,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirAction {
	UP,
	DOWN,
	LEFT,
	RIGHT,
	PRESS,
}

#[derive(Debug, Clone, Copy)]
struct CodePad {
	code: [i32; 4],
	index: usize,
}

#[derive(Debug, Clone, Copy)]
struct DirPad {
	bot_arm_pos: Vec2<i32>,
}

impl fmt::Display for DirPad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.bot_arm_pos)
	}
}

impl CodeAction {
	fn value (&self) -> i32 {
		use CodeAction::*;
		match self {
			ZERO => 0,
			ONE => 1,
			TWO => 2,
			THREE => 3,
			FOUR => 4,
			FIVE => 5,
			SIX => 6,
			SEVEN => 7,
			EIGHT => 8,
			NINE => 9,
			A => 10,
		}
	}
}

impl CodePad {
	fn new() -> CodePad {
		CodePad {code: [-1; 4], index: 0}
	}

	fn end(&self) -> bool {
		self.index == 4
	}

	fn get_code(&self) -> [i32; 4] {
		self.code
	}

	fn generate_action(pos: &Vec2<i32>) -> Option<CodeAction> {
		use CodeAction::*;
		match pos {
			Vec2{x: 0, y: 0} => Some(SEVEN),
			Vec2{x: 1, y: 0} => Some(EIGHT),
			Vec2{x: 2, y: 0} => Some(NINE),
			Vec2{x: 0, y: 1} => Some(FOUR),
			Vec2{x: 1, y: 1} => Some(FIVE),
			Vec2{x: 2, y: 1} => Some(SIX),
			Vec2{x: 0, y: 2} => Some(ONE),
			Vec2{x: 1, y: 2} => Some(TWO),
			Vec2{x: 2, y: 2} => Some(THREE),
			Vec2{x: 0, y: 3} => None,
			Vec2{x: 1, y: 3} => Some(ZERO),
			Vec2{x: 2, y: 3} => Some(A),
			_ => None
		}
	}
	
	fn apply_action(&mut self, action: &CodeAction) {
		if self.end() {panic!("Cannot add to code")}
		
		self.code[self.index] = action.value();
		self.index += 1;
	}
}

impl fmt::Display for CodePad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.code)
	}
}


impl DirPad {
	fn new() -> DirPad {
		DirPad { bot_arm_pos: Vec2::new(2, 0)}
	}

	fn apply_action(&mut self, action: &DirAction) {
		use DirAction::*;
		match action {
			UP => self.bot_arm_pos += Vec2::new(0, -1),
			DOWN => self.bot_arm_pos += Vec2::new(0, 1),
			LEFT => self.bot_arm_pos += Vec2::new(-1, 0),
			RIGHT => self.bot_arm_pos += Vec2::new(1, 0),
			PRESS => ()
		}
	}

	fn generate_action(pos: &Vec2<i32>) -> Option<DirAction> {
		use DirAction::*;
		match pos {
			Vec2{x: 0, y: 0} => None,
			Vec2{x: 1, y: 0} => Some(UP),
			Vec2{x: 2, y: 0} => Some(PRESS),
			Vec2{x: 0, y: 1} => Some(LEFT),
			Vec2{x: 1, y: 1} => Some(DOWN),
			Vec2{x: 2, y: 1} => Some(RIGHT),
			_ => None
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum Pad {
	CodePad(CodePad),
	DirPad(DirPad)
}

impl Action {
	fn toDir(&self) -> DirAction {
		match self {
			Action::DirAction(a) => return *a,
			Action::CodeAction(_) => panic!("INVALID CONVERSION")
		}
	}
	fn toCode(&self) -> CodeAction {
		match self {
			Action::CodeAction(a) => return a,
			Action::DirAction(_) => panic!("INVALID CONVERSION")
		}
	}
}

impl fmt::Display for Pad {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Pad::CodePad(a) => <CodePad as Display>::fmt(a, f),
			Pad::DirPad(a) => <DirPad as Display>::fmt(a, f)
		}
	}
}

impl Pad {
	fn generate_action(&self, pos: &Vec2<i32>) -> Option<Action> {
		match self {
			Pad::CodePad(_) => {
				Some(Action::CodeAction(CodePad::generate_action(pos)?))
			},
			Pad::DirPad(_) => {
				Some(Action::DirAction(DirPad::generate_action(pos)?))
			}
		}
	}

	fn apply_action(& mut self, action: &Action) {
		match self {
			Pad::CodePad(pad) => {
				pad.apply_action(&action.toCode());
			}
			Pad::DirPad(pad) => {
				pad.apply_action(&action.toDir());
			}
		}
	}

	fn get_A_pos(&self) -> Vec2<i32> {
		match self {
			Pad::CodePad(_) => Vec2::new(2, 3),
			Pad::DirPad(_) => Vec2::new(2, 0)
		}
	}

	fn get_arm_pos(&self) -> Option<Vec2<i32>> {
		match self {
			Pad::CodePad(_) => None,
			Pad::DirPad(pad) => Some(pad.bot_arm_pos)
		}
	}
}


#[derive(Debug, Clone, Copy)]
struct State {
	pads: [Pad; 4],
}

impl fmt::Display for State
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.pads)
	}
}

impl State {
	fn new() -> State {
		let mut ret = State {
			pads: [
				Pad::DirPad(DirPad::new()),
				Pad::DirPad(DirPad::new()),
				Pad::DirPad(DirPad::new()),
				Pad::CodePad(CodePad::new()),
			]
		};

		//init start position so all arms focus A
		for i in 1..ret.pads.len() {
			let a_pos = ret.pads[i].get_A_pos();
			if let Pad::DirPad(pad) = & mut ret.pads[i - 1] {
				pad.bot_arm_pos = a_pos;
			}
		}

		ret
	}

	fn end(&self) -> bool {
		self.get_code_pad().end()
	}

	fn get_code(&self) -> [i32; 4] {
		self.get_code_pad().get_code()
	}

	fn get_code_pad(&self) -> &CodePad {
		if let Pad::CodePad(pad) = self.pads.last().unwrap() {
			return pad
		} else {
			panic!("LAST IS NOT CODEPAD")
		}
	}

	fn apply_action(&self, action: &Action) -> Option<State> {
		let mut ret = self.clone();
		let mut current_action = Some(*action);

		let mut index = 0;


		loop {
			ret.pads[index].apply_action(&current_action.unwrap());
			let arm_pos = ret.pads[index].get_arm_pos();

			//case of chained dir pads
			if index + 1 < ret.pads.len() {
				current_action = ret.pads[index + 1].generate_action(&arm_pos.expect("NO ARM POS"));
			}
			
		}
		

		//we stopped because we got returned an invalid action ( pressed blank )
	

		//we stopped because we pressed through all dirpads
		

		//we stopped because we stopped going through presses and have an action to apply
		

		None
	}
}

pub fn solve(input: &str) -> i128 {
	use DirAction::*;
	let mut state = State::new();

	println!("{:}", state);
	state = state.apply_action(&PRESS).unwrap();
	println!("{:}", state);
	state = state.apply_action(&LEFT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&DOWN).unwrap();
	println!("{:}", state);
	state = state.apply_action(&LEFT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&PRESS).unwrap();
	println!("{:}", state);
	state = state.apply_action(&UP).unwrap();
	println!("{:}", state);
	state = state.apply_action(&RIGHT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&RIGHT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&PRESS).unwrap();
	println!("{:}", state);
	state = state.apply_action(&RIGHT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&RIGHT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&RIGHT).unwrap();
	println!("{:}", state);
	state = state.apply_action(&RIGHT).unwrap();

	0
}

//something to convert code to list of coords to type it

//something to pathfind through a list of coords to list all best paths

//something to convert path to a list of coords


//repeat
