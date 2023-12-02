use std::{io::Write, fmt::Write as FmtWrite, collections::HashMap, ops::RangeInclusive};

use crate::{
	aoc::anyhow,
	model::{Word, Instruction, ArgumentValue}
};

pub fn disassemble(mut file: impl Write, memory: &[Word]) -> anyhow::Result<()> {
	let mut instructions = Vec::<Instr>::new();
	
	let mut address = 0;
	while address < memory.len() {
		let instruction = Instruction::decode(&memory[address ..]);
		
		match instruction {
			Err(err) => {
				writeln!(
					file, "0x{:0>4X}: 0x{:0>4X} ({:?}) ({})",
					address,
					memory[address],
					memory[address] as u8 as char,
					err
				)?;
				address += 1;
			}
			Ok(instruction) => {
				let size = instruction.size();

				instructions.push(Instr { a: address as Word, i: instruction });
				address += size;
			}
		}
	}
	writeln!(file, "------------------------------")?;
	
	let mut disassembler = Disassembler::new(instructions)?;
	while disassembler.next(&mut file)? {
		// do
	}

	Ok(())
}


struct Instr {
	pub a: Word,
	pub i: Instruction
}

struct AugmentResult {
	pub indent: u8
}
impl AugmentResult {
	pub fn merge(self, other: Self) -> Self {
		Self {
			indent: self.indent.max(other.indent)
		}
	}
}
impl Default for AugmentResult {
	fn default() -> Self {
		Self {
			indent: 0
		}
	}
}

trait Augment: Sized {
	fn initialize(instructions: &[Instr]) -> anyhow::Result<Self>;
	fn on_instruction(&mut self, instructions: &[Instr], out: impl Write) -> anyhow::Result<AugmentResult>;
}

struct AugOutString {
	string: String,
	last: bool
}
impl Augment for AugOutString {
	fn initialize(_instructions: &[Instr]) -> anyhow::Result<Self> {
		Ok(Self { string: String::new(), last: false })
	}

	fn on_instruction(&mut self, instructions: &[Instr], mut out: impl Write) -> anyhow::Result<AugmentResult> {
		if !self.last {
			let mut it = instructions.iter();

			while let Some(Instr { i: Instruction::Out { source: value }, .. }) = it.next() {
				match value {
					ArgumentValue::Literal(lit) => self.string.push(lit.to_word() as u8 as char),
					ArgumentValue::Register(reg) => {
						let _ = self.string.write_fmt(format_args!("<{:?}>", reg));
					}
				};
			}

			if self.string.len() > 0 {
				writeln!(out, ">> Out string: {:?}", self.string)?;
			}
			self.string.clear();
		}

		self.last = matches!(&instructions[0].i, Instruction::Out { source: ArgumentValue::Literal(_) });

		Ok(AugmentResult::default())
	}
}

struct AugCallMap {
	map: HashMap<Word, Vec<Word>>
}
impl Augment for AugCallMap {
	fn initialize(instructions: &[Instr]) -> anyhow::Result<Self> {
		let mut map = HashMap::<Word, Vec<Word>>::new();
		for instr in instructions.iter() {
			match instr.i {
				Instruction::Call { address: ArgumentValue::Literal(dest) }
				| Instruction::Jmp { address: ArgumentValue::Literal(dest) }
				| Instruction::Jf { address: ArgumentValue::Literal(dest), .. }
				| Instruction::Jt { address: ArgumentValue::Literal(dest), .. }
				=> {
					map.entry(dest.to_word()).or_default().push(instr.a);
				}
				_ => ()
			}
		}
		
		Ok(Self { map })
	}

	fn on_instruction(&mut self, instructions: &[Instr], mut out: impl Write) -> anyhow::Result<AugmentResult> {
		let instr = &instructions[0];
		
		if let Some(static_calls) = self.map.get(&instr.a) {
			write!(out, ">> Called from:")?;
			for call in static_calls.iter() {
				write!(out, " 0x{:0>4X}", call)?;
			}
			writeln!(out)?;
		}
		
		Ok(AugmentResult::default())
	}
}

struct AugIndents {
	intervals: Vec<RangeInclusive<Word>>
}
impl Augment for AugIndents {
	fn initialize(_instructions: &[Instr]) -> anyhow::Result<Self> {
		Ok(Self {
			intervals: vec![
				0x05B2 ..= 0x05ED,
				0x05C8 ..= 0x05E0,
				0x178B ..= 0x17B3,
				0x0731 ..= 0x07D0
			]
		})
	}

	fn on_instruction(&mut self, instructions: &[Instr], mut _out: impl Write) -> anyhow::Result<AugmentResult> {
		let instr = &instructions[0];
		
		let mut counter = 0;
		for interval in self.intervals.iter() {
			if interval.contains(&instr.a) {
				counter += 1;
			}
		}

		Ok(AugmentResult { indent: counter as u8 })
	}
}

struct Disassembler {
	instructions: Vec<Instr>,
	current_index: usize,
	aug_out_string: AugOutString,
	aug_call_map: AugCallMap,
	aug_indents: AugIndents
}
impl Disassembler {
	pub fn new(instructions: Vec<Instr>) -> anyhow::Result<Self> {
		Ok(Self {
			aug_out_string: AugOutString::initialize(&instructions)?,
			aug_call_map: AugCallMap::initialize(&instructions)?,
			aug_indents: AugIndents::initialize(&instructions)?,
			instructions,
			current_index: 0
		})
	}

	pub fn next(&mut self, mut out: impl Write) -> anyhow::Result<bool> {
		let instr = &self.instructions[self.current_index];

		let aug = self.aug_call_map.on_instruction(&self.instructions[self.current_index ..], &mut out)?.merge(
			self.aug_out_string.on_instruction(&self.instructions[self.current_index ..], &mut out)?
		).merge(
			self.aug_indents.on_instruction(&self.instructions[self.current_index ..], &mut out)?
		);

		for _ in 0 .. aug.indent {
			write!(out, "  ")?;
		}
		writeln!(out, "0x{:0>4X}: {}", instr.a, instr.i)?;

		self.current_index += 1;
		Ok(self.current_index < self.instructions.len())
	}
}
