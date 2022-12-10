use std::{
	io::{self, Read, Write},
	ops::{Not, BitOr, Add, Mul, Rem, BitAnd}
};

use base::anyhow::{self, Context};
use base::log;

use aoc_commons as base;

mod model;
use model::{Word, Number, RegisterId, ArgumentValue, Instruction};

struct Cpu<R: Read, W: Write> {
	registers: [Word; 8],
	stack: Vec<Word>,
	memory: Vec<Word>,
	instruction_pointer: Number,
	in_stream: R,
	out_stream: W
}
impl<R: Read, W: Write> Cpu<R, W> {
	pub fn new(
		memory: Vec<Word>,
		in_stream: R,
		out_stream: W
	) -> Self {
		Self {
			registers: [0; 8],
			stack: Vec::new(),
			memory,
			instruction_pointer: Number::ZERO,
			in_stream,
			out_stream
		}
	}

	fn set_register(&mut self, id: RegisterId, value: Word) {
		self.registers[id as u16 as usize - RegisterId::R0 as u16 as usize] = value;
	}

	fn register(&self, id: RegisterId) -> Word {
		self.registers[id as u16 as usize - RegisterId::R0 as u16 as usize]
	}

	fn argument(&self, argument: ArgumentValue) -> Word {
		match argument {
			ArgumentValue::Number(number) => number.to_word(),
			ArgumentValue::RegisterId(id) => self.register(id)
		}
	}

	fn argument_number(&self, argument: ArgumentValue) -> Number {
		match argument {
			ArgumentValue::Number(number) => number,
			ArgumentValue::RegisterId(id) => Number::from_word(self.register(id))
		}
	}

	pub fn tick(&mut self) -> anyhow::Result<bool> {
		log::trace!("CPU tick at: {}", self.instruction_pointer);
		let memory = &self.memory[self.instruction_pointer.to_word() as usize ..];
		let memory = &memory[.. 4.min(memory.len())];
		log::trace!("Decoding memory: {:?}", memory);

		let instruction = Instruction::decode(memory)?;
		self.instruction_pointer = self.instruction_pointer + Number::from_word(instruction.size() as u16);

		log::debug!("Decoded instruction: {:?}", instruction);
		match instruction {
			Instruction::Halt => return Ok(true),
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
					None => return Ok(true),
					Some(v) => v
				};

				self.instruction_pointer = Number::from_word(value);
			}
			Instruction::Out { source } => {
				write!(self.out_stream, "{}", char::from(self.argument(source) as u8)).context("Failed to write to out stream")?;
			},
			Instruction::In { destination } => {
				let mut buf = [0u8; 1];
				self.in_stream.read(&mut buf).context("Failed to read from in stream")?;
				self.set_register(destination, buf[0] as u16);
			},
			Instruction::Noop => ()
		}
		
		Ok(false)
	}
}

fn main() -> anyhow::Result<()> {
	let mut input = base::initialize()?;

	let memory: Vec<Word> = {
		let mut memory = Vec::new();
		
		let mut buf = [0u8; 2];
		loop {
			match input.read_exact(&mut buf) {
				Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => break,
				Err(err) => anyhow::bail!("Failed to read input: {}", err),
				Ok(_) => memory.push(u16::from_le_bytes(buf))
			}
		}

		memory
	};

	let mut cpu = Cpu::new(memory, std::io::stdin(), std::io::stdout());
	loop {
		match cpu.tick() {
			Err(err) => {
				log::error!("CPU error: {}", err);
				log::info!("Stack: {:?}", cpu.stack);
				log::info!("Registers: {:?}", cpu.registers);

				return Err(err);
			}
			Ok(true) => {
				log::info!("CPU halted");
				break
			}
			Ok(false) => ()
		}
	}

	Ok(())
}

#[cfg(test)]
mod test {
    use super::{Cpu, RegisterId};

	#[test]
	fn runs_example_program_correctly() {
		let memory = vec![9,32768,32769,4,19,32768];

		let mut out = Vec::<u8>::new();
		
		let mut cpu = Cpu::new(memory, std::io::empty(), &mut out);
		let out_value = 4 + cpu.register(RegisterId::R1);
		
		assert_eq!(cpu.tick().unwrap(), false);
		assert_eq!(cpu.tick().unwrap(), false);
		assert_eq!(cpu.tick().is_err(), true);
		
		assert_eq!(cpu.register(RegisterId::R0), out_value);
		assert_eq!(out.get(0).copied(), Some(out_value as u8));
	}
}