use std::{
	io::{Read, Write},
	ops::{Not, BitOr, Add, Mul, Rem, BitAnd},
	fmt
};

use serde::{Serialize, Deserialize};

use crate::aoc::{log, anyhow};
use anyhow::Context;

use crate::model::{Word, Number, RegisterId, ArgumentValue, Instruction};

#[derive(Serialize, Deserialize)]
pub struct CpuSnapshot {
	memory: Vec<Word>,
	registers: [Word; 8],
	stack: Vec<Word>,
	instruction_pointer: Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuTickResult {
	Continue,
	Input,
	Halt
}

pub struct Cpu {
	registers: [Word; 8],
	stack: Vec<Word>,
	memory: Vec<Word>,
	instruction_pointer: Number
}
impl Cpu {
	pub fn new(
		memory: Vec<Word>
	) -> Self {
		Self {
			registers: [0; 8],
			stack: Vec::new(),
			memory,
			instruction_pointer: Number::ZERO
		}
	}

	pub fn save(&self) -> CpuSnapshot {
		log::info!("Saving to snapshot");

		CpuSnapshot {
			memory: self.memory.clone(),
			registers: self.registers,
			stack: self.stack.clone(),
			instruction_pointer: self.instruction_pointer
		}
	}

	pub fn restore(&mut self, snapshot: CpuSnapshot) {
		log::info!("Restoring from snapshot");

		self.memory = snapshot.memory;
		self.registers = snapshot.registers;
		self.stack = snapshot.stack;
		self.instruction_pointer = snapshot.instruction_pointer;
	}

	pub fn instruction_pointer(&self) -> Number {
		self.instruction_pointer
	}

	pub fn set_register(&mut self, id: RegisterId, value: Word) {
		self.registers[id as u16 as usize - RegisterId::R0 as u16 as usize] = value;
	}

	pub fn register(&self, id: RegisterId) -> Word {
		self.registers[id as u16 as usize - RegisterId::R0 as u16 as usize]
	}

	pub fn set_memory(&mut self, address: Word, value: Word) {
		self.memory[address as usize] = value;
	}

	fn argument(&self, argument: ArgumentValue) -> Word {
		match argument {
			ArgumentValue::Literal(number) => number.to_word(),
			ArgumentValue::Register(id) => self.register(id)
		}
	}

	fn argument_number(&self, argument: ArgumentValue) -> Number {
		match argument {
			ArgumentValue::Literal(number) => number,
			ArgumentValue::Register(id) => Number::from_word(self.register(id))
		}
	}

	pub fn tick(&mut self, mut in_stream: impl Read, mut out_stream: impl Write) -> anyhow::Result<CpuTickResult> {
		log::trace!("CPU tick at: {}", self.instruction_pointer);
		let memory = &self.memory[self.instruction_pointer.to_word() as usize ..];
		let memory = &memory[.. 4.min(memory.len())];
		log::trace!("Decoding memory: {:?}", memory);

		let instruction = Instruction::decode(memory)?;
		let old_instruction_pointer = self.instruction_pointer;
		self.instruction_pointer = self.instruction_pointer + Number::from_word(instruction.size() as u16);

		log::trace!("Decoded instruction: {:?}", instruction);
		match instruction {
			Instruction::Halt => return Ok(CpuTickResult::Halt),
			Instruction::Set { destination, value } => self.set_register(destination, self.argument(value)),
			Instruction::Push { source } => self.stack.push(self.argument(source)),
			Instruction::Pop { destination } => { let value = self.stack.pop().context("Invalid pop instruction: Stack empty")?; self.set_register(destination, value) },
			Instruction::Eq { destination, left, right } => self.set_register(
				destination, if self.argument(left) == self.argument(right) { 1 } else { 0 }
			),
			Instruction::Gt { destination, left, right } => self.set_register(
				destination, if self.argument(left) > self.argument(right) { 1 } else { 0 }
			),
			Instruction::Jmp { address } => {
				self.instruction_pointer = self.argument_number(address);
			}
			Instruction::Jt { test, address } => if self.argument(test) != 0 {
				self.instruction_pointer = self.argument_number(address);
			},
			Instruction::Jf { test, address } => if self.argument(test) == 0 {
				self.instruction_pointer = self.argument_number(address);
			},
			Instruction::Add { destination, left, right } => self.set_register(destination, self.argument_number(left).add(self.argument_number(right)).to_word()),
			Instruction::Mult { destination, left, right } => self.set_register(destination, self.argument_number(left).mul(self.argument_number(right)).to_word()),
			Instruction::Mod { destination, left, right } => self.set_register(destination, self.argument_number(left).rem(self.argument_number(right)).to_word()),
			Instruction::And { destination, left, right } => self.set_register(destination, self.argument_number(left).bitand(self.argument_number(right)).to_word()),
			Instruction::Or { destination, left, right } => self.set_register(destination, self.argument_number(left).bitor(self.argument_number(right)).to_word()),
			Instruction::Not { destination, value } => self.set_register(destination, self.argument_number(value).not().to_word()),
			Instruction::Rmem { destination, address } => {
				let address = self.argument(address) as usize;
				let memory = self.memory.get(address).context("Memory read out of bounds")?;
				self.set_register(destination, *memory);
			}
			Instruction::Wmem { address, source } => {
				let address = self.argument(address) as usize;
				let source = self.argument(source);
				let memory = self.memory.get_mut(address).context("Memory write out of bounds")?;
				*memory = source;
			}
			Instruction::Call { address } => {
				self.stack.push(self.instruction_pointer.to_word());
				self.instruction_pointer = self.argument_number(address);
			}
			Instruction::Ret => {
				let value = match self.stack.pop() {
					None => return Ok(CpuTickResult::Halt),
					Some(v) => v
				};

				self.instruction_pointer = Number::from_word(value);
			}
			Instruction::Out { source } => {
				write!(out_stream, "{}", char::from(self.argument(source) as u8)).context("Failed to write to out stream")?;
			},
			Instruction::In { destination } => {
				match crate::next_byte(&mut in_stream)? {
					None => {
						self.instruction_pointer = old_instruction_pointer;
						return Ok(CpuTickResult::Input);
					}
					Some(byte) => {
						self.set_register(destination, byte as u16);
					}
				}				
			},
			Instruction::Noop => ()
		}

		Ok(CpuTickResult::Continue)
	}
}
impl fmt::Debug for Cpu {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "---------------CPU---------------")?;
		writeln!(f, "Registers:")?;
		for r in 0 .. self.registers.len() {
			writeln!(f, "R{}: 0x{:0>4X}", r, self.registers[r])?;							
		}
		writeln!(f)?;

		writeln!(f, "Stack:")?;
		for (i, value) in self.stack.iter().enumerate() {
			writeln!(f, "{: >3}: 0x{:0>4X}", i, value)?;
		}
		writeln!(f)?;

		writeln!(f, "Diassembly:")?;

		let mut count = 0;
		let mut ip = self.instruction_pointer.to_word() as usize;
		while count < 7 && ip < self.memory.len() {
			write!(f, "0x{:0>4X}: ", ip)?;
			
			let memory = &self.memory[ip ..];
			let memory = &memory[.. memory.len().min(4)];
			match Instruction::decode(memory) {
				Ok(instr) => {
					writeln!(f, "{:?}", instr)?;
					ip += instr.size();
				}
				Err(_) => {
					writeln!(f, "<{}>", memory[0])?;
					ip += 1;
				}
			}

			count += 1;
		}

		writeln!(f, "---------------------------------")?;

		Ok(())
	}
}
