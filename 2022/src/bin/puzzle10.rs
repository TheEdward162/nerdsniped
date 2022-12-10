use std::{io::Read, fmt, ops::RangeInclusive};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

#[derive(Debug)]
enum Instruction {
	Noop,
	Addx(isize)
}

struct Cpu {
	instructions: Vec<Instruction>,
	registers: [isize; 1],
	current_instruction: usize,
	current_instruction_cycles: usize,
	clock: usize
}
impl Cpu {
	pub fn new(instructions: Vec<Instruction>) -> Self {
		let mut me = Self {
			registers: [1],
			current_instruction: 0,
			current_instruction_cycles: 0,
			clock: 0,
			instructions,
		};

		me.load_instruction();

		me
	}

	fn load_instruction(&mut self) {
		self.current_instruction_cycles = self.instructions.get(self.current_instruction).map(
			|instr| match instr {
				Instruction::Noop => 1,
				Instruction::Addx(_) => 2
			}
		).unwrap_or(1);
	}

	pub fn tick(&mut self) {
		if self.current_instruction < self.instructions.len() {
			self.current_instruction_cycles -= 1;
		}
		
		if self.current_instruction_cycles == 0 {
			if let Some(instruction) = self.instructions.get(self.current_instruction) {
				log::trace!("Applying {:?} @ {}: {:?}", instruction, self.clock, self.registers);
				match instruction {
					Instruction::Noop => (),
					Instruction::Addx(x) => { self.registers[0] += x; }
				}
			}

			self.current_instruction += 1;
			self.load_instruction();
		}
		
		self.clock += 1;
	}

	pub fn registers(&self) -> &[isize; 1] {
		&self.registers
	}

	pub fn clock(&self) -> usize {
		self.clock
	}
}

struct Screen {
	rows: [bool; Self::WIDTH * Self::HEIGHT]
}
impl Screen {
	const WIDTH: usize = 40;
	const HEIGHT: usize = 6;

	pub fn new() -> Self {
		Self {
			rows: [false; Self::WIDTH * Self::HEIGHT]
		}
	}

	pub fn update(&mut self, clock: usize, sprite: RangeInclusive<isize>) {
		let x = clock % Self::WIDTH;
		let y = (clock / Self::WIDTH) % Self::HEIGHT;
		
		let pixel = &mut self.rows[y * Self::WIDTH + x];

		let x_isize = x as isize;
		*pixel = sprite.contains(&x_isize);

		log::trace!("Updating pixel ({}, {}) @ {} in [{}; {}]: {}", x, y, clock, sprite.start(), sprite.end(), *pixel);
	}
}
impl fmt::Display for Screen {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0 .. Self::HEIGHT {
			for x in 0 .. Self::WIDTH {
				if self.rows[y * Self::WIDTH + x] {
					write!(f, "#")?;
				} else {
					write!(f, ".")?;
				}
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut instructions = Vec::new();
	for instruction_str in input.split('\n').filter(|s| !s.is_empty()) {
		let mut parts = instruction_str.split(' ');
		let instruction = match parts.next() {
			Some("noop") => Instruction::Noop,
			Some("addx") => Instruction::Addx(parts.next().context("Missing addx command value")?.parse().context("Invalid addx command value")?),
			instr => anyhow::bail!("Invalid instruction: \"{:?}\"", instr)
		};

		instructions.push(instruction);
	}

	let mut cpu = Cpu::new(instructions);
	let mut sample_points = Vec::new();
	let mut screen = Screen::new();

	loop {
		let reg_x = cpu.registers()[0];
		screen.update(cpu.clock(), reg_x - 1 ..= reg_x + 1);

		cpu.tick();

		match cpu.clock() + 1 {
			clock @ (20 | 60 | 100 | 140 | 180 | 220) => {
				sample_points.push(clock as isize * cpu.registers()[0]);
			}
			240 => break,
			_ => ()
		}
	}
	log::debug!("Sample points: {:?}", sample_points);

	println!("Sample points count: {}", sample_points.into_iter().sum::<isize>());
	println!("Screen:\n{}", screen);

	Ok(())
}
