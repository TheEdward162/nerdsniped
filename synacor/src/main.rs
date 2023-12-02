use std::{
	io::{self, Read, Seek},
	env, fs::OpenOptions
};

use aoc_commons as base;

use base::anyhow::{self, Context};
use base::log;

mod model;
mod disassembler;
mod cpu;

use model::{Word, Number};
use cpu::{Cpu, CpuTickResult, CpuSnapshot};

fn next_byte<R: Read>(mut stream: R) -> anyhow::Result<Option<u8>> {
	let mut buf = [0u8; 1];
	match stream.read(&mut buf) {
		Err(err) if err.kind() == io::ErrorKind::WouldBlock => return Ok(None),
		Ok(0) => return Ok(None),
		Err(err) => anyhow::bail!("Failed to read from in stream: {}", err),
		Ok(_) => Ok(Some(buf[0]))
	}
}

struct InputStream {
	buffer: Vec<u8>,
	cursor: usize
}
impl InputStream {
	pub fn new() -> Self {
		Self { buffer: Vec::new(), cursor: 0 }
	}

	pub fn as_slice(&self) -> &[u8] {
		&self.buffer[self.cursor ..]
	}

	pub fn extend(&mut self, iter: impl Iterator<Item = u8>) {
		self.buffer.extend(iter);
	}

	pub fn clean(&mut self) {
		let _ = self.buffer.drain(.. self.cursor);
		self.cursor = 0;
	}
}
impl Read for InputStream {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let read_len = (self.buffer.len() - self.cursor).min(buf.len());

		buf[..read_len].copy_from_slice(&self.buffer[self.cursor ..][.. read_len]);
		self.cursor += read_len;

		Ok(read_len)
	}
}

struct U16Value(u16);
impl<'a> TryFrom<&'a str> for U16Value {
	type Error = std::num::ParseIntError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		if value.starts_with("0x") {
			u16::from_str_radix(&value[2..], 16)
		} else {
			u16::from_str_radix(value, 10)
		}.map(Self)
	}
}

enum RunState {
	Run,
	Step,
	UntilAddress(Number)
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

	// dump disassembly
	match env::var("MEMORY_DISASSEMBLY") {
		Err(_) => (),
		Ok(value) => {
			let file = OpenOptions::new().write(true).create(true).truncate(true).open(value).context("Failed to open disassembly file")?;
			disassembler::disassemble(file, &memory).context("Failed to disassemble")?;
		}
	};
	let mut snapshot_file = match env::var("CPU_SNAPSHOT") {
		Err(_) => None,
		Ok(value) => Some(OpenOptions::new().read(true).write(true).create(true).open(value).context("Failed to open snapshot file")?)
	};

	// cpu state
	let mut cpu = Cpu::new(memory);
	let mut in_stream = InputStream::new();
	let mut out_stream = Vec::<u8>::new();

	// main loop
	let mut need_input = false;
	let mut run_state = RunState::Run;
	loop {
		let pause = match run_state {
			RunState::Step => true,
			RunState::UntilAddress(address) if cpu.instruction_pointer() == address => {
				run_state = RunState::Step;
				true
			}
			_ => false,
		};

		if pause {
			eprint!("{:?}", cpu);
			eprintln!("Buffered input: {:?}", String::from_utf8_lossy(in_stream.as_slice()));
			eprintln!("Buffered output: {:?}", String::from_utf8_lossy(out_stream.as_slice()));
			
			eprint!("> ");
			need_input = true;
		}
		if let Some(b'\n') = out_stream.last() {
			if !pause {
				print!("{}", String::from_utf8_lossy(out_stream.as_slice()));
			}
			out_stream.clear();
		}

		if need_input {
			let line = std::io::stdin().lines().next()
				.context("Stdin ended")?
				.context("Failed to read stdin")?
			;
			need_input = false;
			
			let continue_tick = match line.as_str() {
				"!save" => {
					if let Some(snapshot_file) = snapshot_file.as_mut() {
						let snapshot = cpu.save();

						snapshot_file.set_len(0).context("Failed to truncate file")?;
						snapshot_file.seek(io::SeekFrom::Start(0)).context("Failed to seek snapshot file")?;
						serde_json::to_writer(snapshot_file, &snapshot).context("Failed to write snapshot file")?;
					} else {
						log::warn!("No snapshot file");
					}

					false
				}
				"!load" => {
					if let Some(snapshot_file) = snapshot_file.as_mut() {
						snapshot_file.seek(io::SeekFrom::Start(0)).context("Failed to seek snapshot file")?;
						match serde_json::from_reader::<_, CpuSnapshot>(snapshot_file).context("Failed to read snapshot file") {
							Err(err) => log::error!("{}", err),
							Ok(snapshot) => cpu.restore(snapshot)
						}
					} else {
						log::warn!("No snapshot file");
					}

					false
				}
				"!step" => {
					run_state = RunState::Step;

					false
				}
				line if line.starts_with("!continue") => {
					if let Ok(address) = base::match_tokens!(line.split(' '); "!continue", address: U16Value) {
						run_state = RunState::UntilAddress(Number::from_word(address.0));
					} else {
						run_state = RunState::Run;
					}

					true
				}
				line if line.starts_with("!set") => {
					if let Ok((register, value)) = base::match_tokens!(line.split(' '); "!set", "reg", register: model::RegisterId, value: U16Value) {
						cpu.set_register(register, value.0);
					} else if let Ok((address, value)) = base::match_tokens!(line.split(' '); "!set", "mem", address: U16Value, value: U16Value) {
						cpu.set_memory(address.0, value.0);
					} else {
						log::error!("Invalid !set command: \"{}\"", line);
					}

					false
				}
				_ if line.starts_with("!input") => {
					in_stream.extend(line.into_bytes().into_iter().skip(7));
					in_stream.extend(std::iter::once(b'\n'));
					
					false
				}
				_ if matches!(run_state, RunState::Run) => {
					in_stream.clean();
					in_stream.extend(line.into_bytes().into_iter());
					in_stream.extend(std::iter::once(b'\n'));

					true
				}
				"" => true,
				line => {
					log::error!("Invalid debug command '{}' - known codes: !save, !load, !step, !continue [hex address], !set [reg value]", line);

					false
				}
			};

			if !continue_tick {
				continue;
			}
		}

		match cpu.tick(&mut in_stream, &mut out_stream) {
			Err(err) => {
				log::error!("CPU error: {}", err);
				eprintln!("{:?}", cpu);

				return Err(err);
			}
			Ok(CpuTickResult::Halt) => {
				log::info!("CPU halted");
				break
			}
			Ok(CpuTickResult::Input) => {
				log::debug!("Waiting for input");
				need_input = true;
			}
			Ok(CpuTickResult::Continue) => ()
		}
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use crate::cpu::{Cpu, CpuTickResult};
	use crate::model::RegisterId;

	#[test]
	fn runs_example_program_correctly() {
		let memory = vec![9,32768,32769,4,19,32768];

		let mut out = Vec::<u8>::new();

		let mut cpu = Cpu::new(memory);
		let out_value = 4 + cpu.register(RegisterId::R1);

		assert_eq!(cpu.tick(std::io::empty(), &mut out).unwrap(), CpuTickResult::Continue);
		assert_eq!(cpu.tick(std::io::empty(), &mut out).unwrap(), CpuTickResult::Continue);
		assert_eq!(cpu.tick(std::io::empty(), &mut out).is_err(), true);

		assert_eq!(cpu.register(RegisterId::R0), out_value);
		assert_eq!(out.get(0).copied(), Some(out_value as u8));
	}
}