//! Parallel + fixnum

use std::{
	fs::File,
	io::{BufReader, BufRead, Read, Seek},
	collections::HashMap, ops::{AddAssign, Add}
};

#[derive(Debug, Clone, Copy)]
struct FixedReal<const DEC: u8>(i32);
impl<const DEC: u8> FixedReal<DEC> {
	pub const MAX: Self = FixedReal(i32::MAX);
	pub const MIN: Self = FixedReal(i32::MIN);
	pub const ZERO: Self = FixedReal(0);
	const DECIMAL_MULT: i32 = 10i32.pow(DEC as u32);

	pub fn min(self, other: Self) -> Self {
		Self(self.0.min(other.0))
	}

	pub fn max(self, other: Self) -> Self {
		Self(self.0.max(other.0))
	}
}
impl<const DEC: u8> Add for FixedReal<DEC> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}
impl<const DEC: u8> AddAssign for FixedReal<DEC> {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
	}
}
impl<const DEC: u8> From<FixedReal<DEC>> for f64 {
	fn from(value: FixedReal<DEC>) -> Self {
		f64::from(value.0) / FixedReal::<DEC>::DECIMAL_MULT as f64
	}
}
impl<const DEC: u8> TryFrom<&str> for FixedReal<DEC> {
	type Error = std::num::ParseIntError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {		
		match value.split_once('.') {
			None => Ok(Self(
				value.parse::<i32>()? * Self::DECIMAL_MULT
			)),
			Some((whole, decimal)) => {
				let whole = whole.parse::<i32>()? * Self::DECIMAL_MULT;
				let decimal = decimal.parse::<i32>()? * whole.signum();

				Ok(Self(whole + decimal))
			}
		}
	}
}

type Real = FixedReal<1>;


#[derive(Debug)]
struct Stats {
	pub min: Real,
	pub max: Real,
	pub sum: Real,
	pub count: usize
}
impl Default for Stats {
	fn default() -> Self {
		Self {
			min: Real::MAX,
			max: Real::MIN,
			sum: Real::ZERO,
			count: 0
		}
	}
}
impl Stats {
	pub fn update(&mut self, temp: Real) {
		self.min = self.min.min(temp);
		self.max = self.max.max(temp);
		self.sum += temp;
		self.count += 1;
	}

	pub fn mean(&self) -> f64 {
		f64::from(self.sum) / self.count as f64
	}

	pub fn merge_mut(&mut self, other: Stats) {
		self.min = self.min.min(other.min);
		self.max = self.max.max(other.max);
		self.sum += other.sum;
		self.count += other.count;
	}
}

fn first_newline(mut r: impl Read) -> usize {
	let mut buf = [0u8; 64];
	loop {
		let count = r.read(&mut buf).unwrap();
		if count == 0 {
			break 0
		}
		if let Some((i, _)) = buf.iter().enumerate().find(|(_, b)| **b == b'\n') {
			break i
		}
	}
}

fn task_main(mut file: File, start: u64, end: u64) -> HashMap<String, Stats> {
	file.seek(std::io::SeekFrom::Start(start)).unwrap();
	let precise_start = if start == 0 {
		0
	} else {
		start + first_newline(&mut file) as u64 + 1
	};
	file.seek(std::io::SeekFrom::Start(end)).unwrap();
	let precise_end = end + first_newline(&mut file) as u64 + 1;

	file.seek(std::io::SeekFrom::Start(precise_start)).unwrap();
	let mut input = BufReader::new(file).take(precise_end - precise_start);

	let mut stats = HashMap::<String, Stats>::new();
	let mut buf = vec![];
	loop {
		buf.clear();
		let count = input.read_until(b';', &mut buf).unwrap();
		if count == 0 {
			break
		}

		let name = String::from(
			std::str::from_utf8(&buf[..buf.len() - 1]).unwrap()
		);
		
		buf.clear();
		input.read_until(b'\n', &mut buf).unwrap();
		let temp = std::str::from_utf8(&buf[..buf.len()-1]).unwrap(); 
		let temp = Real::try_from(temp).unwrap();

		stats.entry(name).or_default().update(temp);
	}

	stats
}

fn main() {
	let input_path = std::env::args().skip(1).next().unwrap();
	let input_size = std::fs::metadata(&input_path).unwrap().len();

	let task_count = 16;
	let mut tasks = Vec::new();
	let chunk_size = input_size / task_count;
	for i in 0 .. task_count as u64 {
		let file = std::fs::File::open(&input_path).unwrap();
		let end = if i == task_count - 1 {
			input_size
		} else {
			(i + 1) * chunk_size
		};

		tasks.push(
			std::thread::spawn(move || {
				task_main(file, i * chunk_size, end)
			})
		);
	}

	let mut stats = HashMap::<String, Stats>::new();
	for task in tasks {
		let result = task.join().unwrap();
		for (k, v) in result.into_iter() {
			stats.entry(k).or_default().merge_mut(v);
		}
	}

	let mut stations: Vec<(String, Stats)> = stats.into_iter().collect();
	stations.sort_unstable_by(|a, b| a.0.cmp(&b.0));

	let mut stations = stations.into_iter();
	print!("{{");
	if let Some((name, stats)) = stations.next() {
		print!("{}={:.1}/{:.1}/{:.1}", name, f64::from(stats.min), stats.mean(), f64::from(stats.max));
	}
	for (name, stats) in stations {
		print!(", {}={:.1}/{:.1}/{:.1}", name, f64::from(stats.min), stats.mean(), f64::from(stats.max));
	}
	println!("}}");
}

#[cfg(test)]
mod test {
	use super::Real;

	#[test]
	fn test_fixnum() {
		let a = Real::try_from("12.3").unwrap();
		let b = Real::try_from("-45.6").unwrap();

		assert_eq!(a.0, 123);
		assert_eq!(f64::from(a), 12.3);

		assert_eq!(b.0, -456);
		assert_eq!(f64::from(b), -45.6);
	}
}