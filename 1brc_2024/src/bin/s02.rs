//! Output writing

use std::{
	fs::OpenOptions,
	io::{BufReader, BufRead, Write},
	collections::HashMap
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
		let temp = std::str::from_utf8(&buf[..buf.len()-1]).unwrap().parse::<f64>().unwrap();

		stats.entry(name).or_default().update(temp);
	}

	let mut stations: Vec<(String, Stats)> = stats.into_iter().collect();
	stations.sort_unstable_by(|a, b| a.0.cmp(&b.0));
	
	let mut stations = stations.into_iter();
	let stdout = std::io::stdout();
	let mut stdout = stdout.lock();

	write!(&mut stdout, "{{").unwrap();
	if let Some((name, stats)) = stations.next() {
		write!(&mut stdout, "{}={:.1}/{:.1}/{:.1}", name, stats.min, stats.mean(), stats.max).unwrap();
	}
	for (name, stats) in stations {
		write!(&mut stdout, ", {}={:.1}/{:.1}/{:.1}", name, stats.min, stats.mean(), stats.max).unwrap();
	}
	writeln!(&mut stdout, "}}").unwrap();
}
