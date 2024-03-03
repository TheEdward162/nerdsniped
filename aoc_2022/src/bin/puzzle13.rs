use std::{io::Read, fmt};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{anyhow, log};

#[derive(Debug, PartialEq, Eq, Clone)]
enum PacketElement {
	Value(u32),
	Array(Vec<PacketElement>)
}
impl PartialOrd for PacketElement {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}
impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
			(Self::Value(a), Self::Value(b)) => a.cmp(b),
			(Self::Array(a), Self::Array(b)) => a.cmp(b),
			(Self::Value(a), b @ Self::Array(_)) => Self::Array(vec![Self::Value(*a)]).cmp(b),
			(a @ Self::Array(_),  b@ Self::Value(_)) => b.cmp(a).reverse(),
		}
    }
}
impl fmt::Display for PacketElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
			Self::Value(value) => write!(f, "{}", value),
			Self::Array(array) => {
				write!(f, "[")?;
				for (i, elem) in array.iter().enumerate() {
					if i > 0 {
						write!(f, ",")?;
					}
					write!(f, "{}", elem)?;
				}
				write!(f, "]")?;
				Ok(())
			}
		}
    }
}

macro_rules! scan {
	(
		$char_stream: expr;
		$( depth_in = $depth_in: pat, )?
		depth_out = $depth_out: pat

	) => {
		{
			let mut end: Option<(usize, char)> = None;
			let mut depth: usize = 1;

			for (i, ch) in $char_stream {
				match ch {
					$( $depth_in => { depth += 1; }, )?
					$depth_out => {
						depth -= 1;
						if depth == 0 {
							end = Some((i, ch));
							break;
						}
					}
					_ => ()
				}
			}

			end
		}
	};
}

fn parse_next<'a>(input: &'a str) -> anyhow::Result<(PacketElement, &'a str)> {
	let mut chars = input.chars().enumerate();

	let (elem, end) = match chars.next() {
		Some((_, '[')) => {
			let (mut inner, end) = match scan!(&mut chars; depth_in = '[', depth_out = ']') {
				None => (input, input.len()),
				Some((end, _)) => (&input[1 .. end], end + 1)
			};

			log::trace!("array inner: {}", inner);

			let mut array = Vec::new();
			while inner.len() > 0 {
				let res = parse_next(inner)?;
				array.push(res.0);
				inner = res.1;
			}

			(PacketElement::Array(array), end + 1)
		}
		Some((_, '0' ..= '9')) => {
			let (inner, end) = match scan!(&mut chars; depth_out = ']' | ',') {
				None => (input, input.len()),
				Some((end, ']')) => (&input[0 .. end], end),
				Some((end, _)) => (&input[0 .. end], end + 1)
			};

			log::trace!("value inner: {}", inner);
			let value = inner.parse::<u32>().context("Failed to parse value")?;

			(PacketElement::Value(value), end)
		}
		ch => anyhow::bail!("Invalid input: {:?}", ch)
	};

	Ok((elem, input.get(end ..).unwrap_or("")))
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut pairs = Vec::new();
	for pair in input.split("\n\n").filter(|s| !s.is_empty()) {
		let (top_str, bottom_str) = pair.split_once('\n').context("Failed to split pair by '\\n'")?;

		let (top, _) = parse_next(top_str)?;
		let (bottom, _) = parse_next(bottom_str)?;

		pairs.push([top, bottom]);
	}

	// part1
	let mut correct_indices = Vec::new();
	for (i, pair) in pairs.iter().enumerate() {
		let correct_order = pair[0] <= pair[1];
		if correct_order {
			correct_indices.push(i + 1);
		}
		
		log::debug!("{}\n<\n{}: {}", pair[0], pair[1], correct_order);
	}
	println!("Correct indices sum: {}", correct_indices.iter().copied().sum::<usize>());

	// part2
	let mut all_packets: Vec<PacketElement> = pairs.into_iter().flat_map(|pair| pair.into_iter()).collect();
	let divider1 = PacketElement::Array(vec![PacketElement::Array(vec![PacketElement::Value(2)])]);
	let divider2 = PacketElement::Array(vec![PacketElement::Array(vec![PacketElement::Value(6)])]);
	all_packets.push(divider1.clone());
	all_packets.push(divider2.clone());
	all_packets.sort();
	
	let mut all_iter = all_packets.into_iter().enumerate();
	let divider1_index = all_iter.find(|(_, p)| p == &divider1).context("Failed to find divider 1")?.0 + 1;
	let divider2_index = all_iter.find(|(_, p)| p == &divider2).context("Failed to find divider 2")?.0 + 1;
	println!("Decoder key: {}", divider1_index * divider2_index);

	Ok(())
}
