use std::{io::Write, time::Instant, collections::HashMap};

const fn add(a: u16, b: u16) -> u16 {
	(a + b) % 0x8000
}

struct State {
	cache: HashMap<(u16, u16, u16), u16>,
	r7: u16
}
impl State {
	pub fn new(r7: u16) -> Self {
		State { cache: HashMap::new(), r7 }
	}

	pub fn run(&mut self, a: u16, b: u16) -> u16 {
		if let Some(cached) = self.cache.get(&(a, b, self.r7)) {
			return *cached;
		}

		let result = if a == 0 {
			add(b, 1)
		} else if b == 0 {
			self.run(add(a, 0x7FFF), self.r7)
		} else {
			let c = self.run(a, add(b, 0x7FFF));
			self.run(add(a, 0x7FFF), c)
		};

		self.cache.insert((a, b, self.r7), result);
		result
	}
}

fn main() {
	let out = std::io::stdout();
	let mut out = out.lock();

	let mut results = Vec::new();

	let mut start;
	for r7 in 1 .. 0x8000 {
		write!(&mut out, "thing(4, 1, {}) = ", r7).unwrap();
		out.flush().unwrap();

		start = Instant::now();
		let mut state = State::new(r7);
		let res = state.run(4, 1);
		writeln!(&mut out, "{} [took {:.2}s]", res, start.elapsed().as_secs_f32()).unwrap();

		if res == 6 {
			results.push(r7);
		}
	}

	println!("Results: {:?}", results);
}