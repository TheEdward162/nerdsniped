use std::{
	io::Read,
	fmt
};

use anyhow::Context;

use aoc_commons as base;
use base::{
	anyhow,
	log
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnafuDigit {
	Two,
	One,
	Zero,
	MinusOne,
	MinusTwo
}
impl<'a> TryFrom<char> for SnafuDigit {
	type Error = anyhow::Error;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		let me = match value {
			'2' => Self::Two,
			'1' => Self::One,
			'0' => Self::Zero,
			'-' => Self::MinusOne,
			'=' => Self::MinusTwo,
			_ => anyhow::bail!("Invalid SNAFU digit")
		};

		Ok(me)
	}
}
impl SnafuDigit {
	pub fn to_decimal(self) -> isize {
		match self {
			Self::Two => 2,
			Self::One => 1,
			Self::Zero => 0,
			Self::MinusOne => -1,
			Self::MinusTwo => -2
		}
	}

	pub fn add(self, rhs: Self) -> [Self; 2] {
		match self.to_decimal() + rhs.to_decimal() {
			-4 => [Self::One, Self::MinusOne],
			-3 => [Self::Two, Self::MinusOne],
			-2 => [Self::MinusTwo, Self::Zero],
			-1 => [Self::MinusOne, Self::Zero],
			0 => [Self::Zero, Self::Zero],
			1 => [Self::One, Self::Zero],
			2 => [Self::Two, Self::Zero],
			3 => [Self::MinusTwo, Self::One],
			4 => [Self::MinusOne, Self::One],
			_ => unreachable!()
		}
	}
}
impl fmt::Display for SnafuDigit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Two => write!(f, "2"),
			Self::One => write!(f, "1"),
			Self::Zero => write!(f, "0"),
			Self::MinusOne => write!(f, "-"),
			Self::MinusTwo => write!(f, "=")
		}
	}
}

struct SnafuNumber(Vec<SnafuDigit>);
impl<'a> TryFrom<&'a str> for SnafuNumber {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let mut digits = Vec::new();
		for ch in value.chars() {
			digits.insert(
				0,
				SnafuDigit::try_from(ch)?
			);
		}

		Ok(Self(digits))
	}
}
impl SnafuNumber {
	pub fn zero() -> Self {
		Self(vec![SnafuDigit::Zero])
	}

	pub fn one() -> Self {
		Self(vec![SnafuDigit::One])
	}

	pub fn to_decimal(&self) -> isize {
		let mut result = 0;
		for (i, d) in self.0.iter().enumerate() {
			result += 5isize.pow(i as u32) * d.to_decimal();
		}

		result
	}

	pub fn add(&mut self, rhs: &Self) {
		let mut previous_carry = SnafuDigit::Zero;

		log::debug!(
			"{}({}) + {}({})",
			self, self.to_decimal(),
			rhs, rhs.to_decimal()
		);

		for i in 0 .. self.0.len().max(rhs.0.len()) {
			match (self.0.get(i), rhs.0.get(i)) {
				(Some(&left), Some(&right)) => {
					let [new_left, carry] = left.add(right);
					let [new_left, carry2] = new_left.add(previous_carry);
					log::trace!("{} + {} + {} = {} [{}+{}]", left, right, previous_carry, new_left, carry, carry2);
					previous_carry = carry.add(carry2)[0];
					assert_eq!(carry.add(carry2)[1], SnafuDigit::Zero);

					self.0[i] = new_left;
				}
				(Some(&left), None) => {
					let [new_left, carry] = left.add(previous_carry);
					log::trace!("{} + {} = {} [{}]", left, previous_carry, new_left, carry);
					previous_carry = carry;

					self.0[i] = new_left;
					if previous_carry == SnafuDigit::Zero {
						break;
					}
				}
				(None, Some(&right)) => {
					let [new_left, carry] = right.add(previous_carry);
					log::trace!("{} + {} = {} [{}]", right, previous_carry, new_left, carry);
					
					self.0.push(new_left);

					if carry != SnafuDigit::Zero {
						self.0.push(carry);
					}
					previous_carry = SnafuDigit::Zero;
				}
				(None, None) => unreachable!()
			}
		}
		if previous_carry != SnafuDigit::Zero {
			self.0.push(previous_carry);
		}

		log::debug!("= {}({})", self, self.to_decimal());
	}
}
impl fmt::Display for SnafuNumber {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for d in self.0.iter().rev() {
			write!(f, "{}", d)?;
		}

		Ok(())
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut numbers = Vec::new();
	for line in input.lines().filter(|s| !s.is_empty()) {
		numbers.push(
			SnafuNumber::try_from(line)?
		);
	}

	let mut sum = SnafuNumber::zero();
	for number in numbers.iter() {
		log::debug!("Number: {} = {}", number, number.to_decimal());
		sum.add(number);
	}
	log::info!("Sum: {} = {}", sum, sum.to_decimal());
	
	log::info!("Done");

	Ok(())
}

#[cfg(test)]
mod test {
	use super::SnafuNumber;

	#[test]
	fn test() {
		let mut test = SnafuNumber::zero();

		const RES: isize = 2022;
		for _ in 0 .. RES {
			test.add(&SnafuNumber::one());
		}

		assert_eq!(test.to_decimal(), RES);
	}
}
