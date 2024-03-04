//! Parallel + fixnum + input bytes

use std::{
	fs::File,
	io::{Read, Seek, BufReader, BufRead},
	collections::HashMap, ops::{AddAssign, Add}
};

fn i32_from_bytes(v: &[u8]) -> Result<i32, std::num::ParseIntError> {
	i32::from_str_radix(
		// We could either reimplement this ourselves (shittily), or do a little unsafe which isn't unsafe as long as we rely on internal implementation of `from_str_radix` (which calls .as_bytes() anyway)
		unsafe { std::str::from_utf8_unchecked(v) },
		10
	)
}

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
impl<const DEC: u8> TryFrom<&[u8]> for FixedReal<DEC> {
	type Error = std::num::ParseIntError;

	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
		let inner = match memchr::memchr(b'.', value) {
			None => i32_from_bytes(value)? * Self::DECIMAL_MULT,
			Some(split_index) => {
				let whole = i32_from_bytes(&value[..split_index])? * Self::DECIMAL_MULT;
				let decimal = i32_from_bytes(&value[split_index + 1..])? * whole.signum();

				whole + decimal
			}
		};

		Ok(Self(inner))
	}
}


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

type Real = FixedReal<1>;
type StatsMap = HashMap<Box<[u8]>, Stats>;

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

fn task_main(mut file: File, start: u64, end: u64) -> StatsMap {
	file.seek(std::io::SeekFrom::Start(start)).unwrap();
	let precise_start = if start == 0 {
		0
	} else {
		start + first_newline(&mut file) as u64 + 1
	};
	file.seek(std::io::SeekFrom::Start(end)).unwrap();
	let precise_end = end + first_newline(&mut file) as u64 + 1;

	file.seek(std::io::SeekFrom::Start(precise_start)).unwrap();
	let mut input = BufReader::with_capacity(1024 * 1024, file).take(precise_end - precise_start);

	let mut stats = StatsMap::new();
	let mut buf = vec![];
	loop {
		buf.clear();
		let count = input.read_until(b'\n', &mut buf).unwrap();
		if count == 0 {
			break
		}
		buf.pop();

		let (name, temp) = buf.split_at(
			memchr::memchr(b';', &buf).unwrap()
		);
		let temp = Real::try_from(&temp[1..]).unwrap();

		stats.entry(name.into()).or_default().update(temp);
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

	let mut stats = StatsMap::new();
	for task in tasks {
		let result = task.join().unwrap();
		for (k, v) in result.into_iter() {
			stats.entry(k).or_default().merge_mut(v);
		}
	}

	let mut stations: Vec<(_, Stats)> = stats.into_iter().collect();
	stations.sort_unstable_by(|a, b| a.0.cmp(&b.0));

	let mut stations = stations.into_iter();
	print!("{{");
	if let Some((name, stats)) = stations.next() {
		print!("{}={:.1}/{:.1}/{:.1}", std::str::from_utf8(&name).unwrap(), f64::from(stats.min), stats.mean(), f64::from(stats.max));
	}
	for (name, stats) in stations {
		print!(", {}={:.1}/{:.1}/{:.1}", std::str::from_utf8(&name).unwrap(), f64::from(stats.min), stats.mean(), f64::from(stats.max));
	}
	println!("}}");
}
