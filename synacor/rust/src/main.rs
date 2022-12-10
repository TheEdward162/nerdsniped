use std::{
	io::{self, Read, Write, Seek},
	ops::{Not, BitOr, Add, Mul, Rem, BitAnd},
	env, fs::OpenOptions, fmt
};

use serde::{Serialize, Deserialize};

use base::anyhow::{self, Context};
use base::log;

use aoc_commons as base;

mod model;
use model::{Word, Number, RegisterId, ArgumentValue, Instruction};

fn next_byte<R: Read>(mut stream: R) -> anyhow::Result<u8> {
	let mut buf = [0u8; 1];
	stream.read(&mut buf).context("Failed to read from in stream")?;

	Ok(buf[0])
}

#[derive(Serialize, Deserialize)]
struct CpuSnapshot {
	memory: Vec<Word>,
	registers: [Word; 8],
	stack: Vec<Word>,
	instruction_pointer: Number,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CpuTickResult {
	Continue,
	Debug(u8),
	Halt
}

struct CpuDebugState {
	last_input_was_newline: bool
}
impl CpuDebugState {
	pub fn new() -> Self {
		Self {
			last_input_was_newline: true
		}
	}
}

struct Cpu<R: Read, W: Write> {
	registers: [Word; 8],
	stack: Vec<Word>,
	memory: Vec<Word>,
	instruction_pointer: Number,
	in_stream: R,
	out_stream: W,
	cpu_debug: CpuDebugState
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
			out_stream,
			cpu_debug: CpuDebugState::new()
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

	fn set_register(&mut self, id: RegisterId, value: Word) {
		self.registers[id as u16 as usize - RegisterId::R0 as u16 as usize] = value;
	}

	fn register(&self, id: RegisterId) -> Word {
		self.registers[id as u16 as usize - RegisterId::R0 as u16 as usize]
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

	pub fn tick(&mut self) -> anyhow::Result<CpuTickResult> {
		log::trace!("CPU tick at: {}", self.instruction_pointer);
		let memory = &self.memory[self.instruction_pointer.to_word() as usize ..];
		let memory = &memory[.. 4.min(memory.len())];
		log::trace!("Decoding memory: {:?}", memory);

		let instruction = Instruction::decode(memory)?;
		let old_instruction_pointer = self.instruction_pointer;
		self.instruction_pointer = self.instruction_pointer + Number::from_word(instruction.size() as u16);

		log::debug!("Decoded instruction: {:?}", instruction);
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
				write!(self.out_stream, "{}", char::from(self.argument(source) as u8)).context("Failed to write to out stream")?;
			},
			Instruction::In { destination } => {
				let byte = next_byte(&mut self.in_stream)?;
				if byte == b'!' && self.cpu_debug.last_input_was_newline {
					self.cpu_debug.last_input_was_newline = false;

					let code = next_byte(&mut self.in_stream)?;
					if next_byte(&mut self.in_stream)? != b'\n' {
						log::warn!("Expected newline after debug command");
					}
					self.instruction_pointer = old_instruction_pointer;

					return Ok(CpuTickResult::Debug(code));
				}
				self.cpu_debug.last_input_was_newline = byte == b'\n';

				self.set_register(destination, byte as u16);
			},
			Instruction::Noop => ()
		}

		Ok(CpuTickResult::Continue)
	}
}
impl<R: Read, W: Write> fmt::Debug for Cpu<R, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

		writeln!(f)?;

		Ok(())
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

	let mut snapshot_file = match env::var("CPU_SNAPSHOT") {
		Err(_) => None,
		Ok(value) => Some(OpenOptions::new().read(true).write(true).create(true).open(value).context("Failed to open snapshot file")?)
	};

	let mut stepping: bool = false;
	loop {
		if stepping {
			println!("?");
			match next_byte(&mut cpu.in_stream)? {
				b'q' => {
					stepping = false;
				}
				_ => ()
			}
		}

		match cpu.tick() {
			Err(err) => {
				log::error!("CPU error: {}", err);
				println!("{:?}", cpu);

				return Err(err);
			}
			Ok(CpuTickResult::Halt) => {
				log::info!("CPU halted");
				break
			}
			Ok(CpuTickResult::Debug(code)) => {
				match code {
					b's' => if let Some(snapshot_file) = snapshot_file.as_mut() {
						let snapshot = cpu.save();

						snapshot_file.set_len(0).context("Failed to truncate file")?;
						snapshot_file.seek(io::SeekFrom::Start(0)).context("Failed to seek snapshot file")?;
						serde_json::to_writer(snapshot_file, &snapshot).context("Failed to write snapshot file")?;
					},
					b'l' => if let Some(snapshot_file) = snapshot_file.as_mut() {
						snapshot_file.seek(io::SeekFrom::Start(0)).context("Failed to seek snapshot file")?;
						match serde_json::from_reader::<_, CpuSnapshot>(snapshot_file).context("Failed to read snapshot file") {
							Err(err) => log::error!("{}", err),
							Ok(snapshot) => cpu.restore(snapshot)
						}
					},
					b'd' => { stepping = true; }
					code => log::warn!("Invalid debug code '{}' - known codes: s, l, d", code)
				}
			}
			Ok(CpuTickResult::Continue) => ()
		}

		if stepping {
			println!("{:?}", cpu);
		}
	}

	Ok(())
}

#[cfg(test)]
mod test {
    use crate::CpuTickResult;

    use super::{Cpu, RegisterId};

	#[test]
	fn runs_example_program_correctly() {
		let memory = vec![9,32768,32769,4,19,32768];

		let mut out = Vec::<u8>::new();

		let mut cpu = Cpu::new(memory, std::io::empty(), &mut out);
		let out_value = 4 + cpu.register(RegisterId::R1);

		assert_eq!(cpu.tick().unwrap(), CpuTickResult::Continue);
		assert_eq!(cpu.tick().unwrap(), CpuTickResult::Continue);
		assert_eq!(cpu.tick().is_err(), true);

		assert_eq!(cpu.register(RegisterId::R0), out_value);
		assert_eq!(out.get(0).copied(), Some(out_value as u8));
	}
}