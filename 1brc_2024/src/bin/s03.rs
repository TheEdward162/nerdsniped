//! Don't parse names as strings

use std::{
	fs::OpenOptions,
	io::{BufReader, BufRead},
	collections::HashMap, ops::Deref
};

#[derive(Debug)]
struct Stats {
	pub min: f64,
	pub max: f64,
	pub sum: f64,
	pub count: usize
}
impl Default for Stats {
    fn default() -> Self {
        Self {
			min: f64::MAX,
			max: f64::MIN,
			sum: 0.0,
			count: 0
		}
    }
}
impl Stats {
	pub fn update(&mut self, temp: f64) {
		self.min = self.min.min(temp);
		self.max = self.max.max(temp);
		self.sum += temp;
		self.count += 1;
	}

	pub fn mean(&self) -> f64 {
		self.sum / self.count as f64
	}
}

fn main() {
	let input_path = std::env::args().skip(1).next().unwrap();
	let input = OpenOptions::new().read(true).open(&input_path).unwrap();
	let mut input = BufReader::new(input);

	let mut stats = HashMap::<Box<[u8]>, Stats>::new();

	let mut buf = Vec::new();
	loop {
		buf.clear();
		let count = input.read_until(b';', &mut buf).unwrap();
		if count == 0 {
			break
		}

		let name = Box::from(
			&buf[..buf.len() - 1]
		);
		
		buf.clear();
		input.read_until(b'\n', &mut buf).unwrap();
		let temp = std::str::from_utf8(&buf[..buf.len()-1]).unwrap().parse::<f64>().unwrap();

		stats.entry(name).or_default().update(temp);
	}

	let mut stations: Vec<(_, Stats)> = stats.into_iter().collect();
	stations.sort_unstable_by(|a, b| a.0.cmp(&b.0));
	
	let mut stations = stations.into_iter();
	print!("{{");
	if let Some((name, stats)) = stations.next() {
		print!("{}={:.1}/{:.1}/{:.1}", std::str::from_utf8(name.deref()).unwrap(), stats.min, stats.mean(), stats.max);
	}
	for (name, stats) in stations {
		print!(", {}={:.1}/{:.1}/{:.1}", std::str::from_utf8(name.deref()).unwrap(), stats.min, stats.mean(), stats.max);
	}
	println!("}}");
}
