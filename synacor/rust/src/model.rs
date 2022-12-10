use std::{convert::TryFrom, ops::{Add, Mul, Rem, BitAnd, BitOr, Not}, fmt};

use crate::base::{anyhow, log};

pub type Word = u16;

macro_rules! define_parseable {
	(
		$( #[$attr: meta] )*
		$visibility: vis enum $full_name: ident {
			$( $name: ident = $code: literal ),+ $(,)?
		} 
		hint = $error_name: literal
	) => {
		#[repr(u16)]
		$( #[$attr] )*
		$visibility enum $full_name {
			$($name = $code),+
		}
		impl TryFrom<Word> for $full_name {
			type Error = anyhow::Error;

			fn try_from(value: Word) -> Result<Self, Self::Error> {
				match value {
					$( $code => Ok(Self::$name), )+
					_ => anyhow::bail!(concat!("Invalid ", $error_name, " code: {}"), value)
				}
			}
		}
	};
}
macro_rules! define_instructions {
	(
		$( #[$attr: meta] )*
		$visibility: vis enum $full_name: ident (kind = $kind_name: ident) {
			$(
				$name: ident $({
					$($argument_name: ident: $argument_type: ty),+ $(,)?
				})? = $opcode: literal
			),+ $(,)?
		}
	) => {
		define_parseable! {
			$visibility enum $kind_name {
				$( $name = $opcode ),+
			}
			hint = "operation"
		}
		impl $kind_name {
			pub const fn argument_count(&self) -> usize {
				match self {
					$(
						Self::$name => {
							let count = 0;
							$(
								$(
									let $argument_name = count + 1;
									let count = $argument_name;
								)+
							)?

							count
						}
					),+
				}
			}

			pub fn decode_arguments(&self, values: &[Word]) -> anyhow::Result<$full_name> {
				let needed_arguments = self.argument_count();
				if values.len() < needed_arguments {
					anyhow::bail!("Needed {} arguments but only found {}", needed_arguments, values.len());
				}
				
				let parsed = match self {
					$(
						Self::$name => {
							#[allow(unused_variables)]
							let arg_index = 0;

							$($(
								log::trace!(concat!("Decoding argument as ", stringify!($argument_type) ,": {}"), values[arg_index]);
								let $argument_name = <$argument_type>::try_from(values[arg_index])?;
								#[allow(unused_variables)]
								let arg_index = arg_index + 1;
							)+)?

							$full_name::$name $({ $( $argument_name ),+ })?
						}
					),+
				};

				Ok(parsed)
			}
		}

		$( #[$attr] )*
		$visibility enum $full_name {
			$(
				$name $({ $($argument_name: $argument_type),+ })?
			),+
		}
		impl $full_name {
			pub const fn kind(&self) -> $kind_name {
				match self {
					$( Self::$name $({ $( $argument_name: _ ),+ })? => $kind_name::$name ),+
				}
			}

			pub const fn size(&self) -> usize {
				1 + self.kind().argument_count()
			}
		}
	};
}

define_parseable! {
	#[derive(Debug)]
	pub enum RegisterId {
		R0 = 32768,
		R1 = 32769,
		R2 = 32770,
		R3 = 32771,
		R4 = 32772,
		R5 = 32773,
		R6 = 32774,
		R7 = 32775
	}
	hint = "register"
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Number(u16);
impl TryFrom<Word> for Number {
	type Error = anyhow::Error;

	fn try_from(value: Word) -> Result<Self, Self::Error> {
		if value > 32767 {
			anyhow::bail!("Invalid number value: {}", value);
		}

		Ok(Self(value))
	}
}
impl Number {
	pub const ZERO: Self = Self(0);

	const MODULO: Word = 32768;

	pub const fn from_word(word: Word) -> Self {
		Self(word % Self::MODULO)
	}

	pub const fn to_word(self) -> Word {
		self.0
	}
}
impl Add<Self> for Number {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self((self.0 + rhs.0) % Self::MODULO)
	}
}
impl Mul<Self> for Number {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let wide = self.0 as usize * rhs.0 as usize;
		let wide = wide % Self::MODULO as usize;

		Self(wide as Word)
	}
}
impl Rem<Self> for Number {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		Self(self.0 % rhs.0)
	}
}
impl BitAnd<Self> for Number {
	type Output = Self;

	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}
impl BitOr<Self> for Number {
	type Output = Self;

	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}
impl Not for Number {
	type Output = Self;

	fn not(self) -> Self::Output {
		Self(!self.0 & !Self::MODULO)
	}
}
impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug)]
pub enum ArgumentValue {
	Number(Number),
	RegisterId(RegisterId)
}
impl TryFrom<Word> for ArgumentValue {
	type Error = anyhow::Error;

	fn try_from(value: Word) -> Result<Self, Self::Error> {
		Number::try_from(value).map(Self::Number).or(
			RegisterId::try_from(value).map(Self::RegisterId)
		).map_err(|_| anyhow::anyhow!("Invalid argument value: neither Number nor RegisterId"))
	}
}

define_instructions! {
	#[derive(Debug)]
	pub enum Instruction (kind = InstructionKind) {
		Halt = 0,
		Set {
			destination: RegisterId,
			value: ArgumentValue
		} = 1,
		Push {
			source: ArgumentValue
		} = 2,
		Pop {
			destination: RegisterId
		} = 3,
		Eq {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue
		} = 4,
		Gt {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue 
		} = 5,
		Jmp {
			address: ArgumentValue
		} = 6,
		Jt {
			test: ArgumentValue,
			address: ArgumentValue
		} = 7,
		Jf {
			test: ArgumentValue,
			address: ArgumentValue
		} = 8,
		Add {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue
		} = 9,
		Mult {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue
		} = 10,
		Mod {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue
		} = 11,
		And {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue
		} = 12,
		Or {
			destination: RegisterId,
			left: ArgumentValue,
			right: ArgumentValue
		} = 13,
		Not {
			destination: RegisterId,
			value: ArgumentValue
		} = 14,
		Rmem {
			destination: RegisterId,
			address: ArgumentValue
		} = 15,
		Wmem {
			address: ArgumentValue,
			source: ArgumentValue
		} = 16,
		Call {
			address: ArgumentValue
		} = 17,
		Ret = 18,
		Out {
			source: ArgumentValue
		} = 19,
		In {
			destination: RegisterId
		} = 20,
		Noop = 21
	}
}
impl Instruction {
	pub fn decode(memory: &[Word]) -> anyhow::Result<Self> {
		if memory.len() < 1 {
			anyhow::bail!("Cannot decode empty instruction");
		}

		let kind = InstructionKind::try_from(memory[0])?;
		let instruction = kind.decode_arguments(&memory[1..])?;

		Ok(instruction)
	}
}
