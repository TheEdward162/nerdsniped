use std::collections::VecDeque;
use std::{fmt, collections::HashMap, io::Read};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

use base::{
	combinatorics::Combinations,
	perfect_hash::{BitHash64Assigner, BitHash64, BitHashSet64},
	macros::FromStrToTryFromAdapter
};

type Pressure = u16;
type Time = u8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct NodeName([u8; 2]);
impl<'a> TryFrom<&'a str> for NodeName {
	type Error = anyhow::Error;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let bytes = value.as_bytes();
		anyhow::ensure!(bytes.len() == 2, "Input must be exactly two bytes long");
		
		let name = [bytes[0], bytes[1]];
		anyhow::ensure!(name[0].is_ascii_uppercase() && name[1].is_ascii_uppercase(), "Input must be uppercase");

		Ok(Self(name))
	}
}
impl fmt::Display for NodeName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
	}
}
impl fmt::Debug for NodeName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ValveName({}{})", self.0[0] as char, self.0[1] as char)
	}
}

type NodeNameHash = BitHash64<NodeName>;

#[derive(Debug)]
struct Edge {
	cost: Time,
	end: NodeNameHash
}

#[derive(Debug)]
struct Node {
	rate: Pressure,
	edges: Vec<Edge>
}

#[derive(Debug)]
struct Graph {
	nodes: HashMap<NodeNameHash, Node>,
	sorted_rates: Vec<(NodeNameHash, Pressure)>,
	#[allow(dead_code)]
	assigner: BitHash64Assigner<NodeName>
}

#[derive(Clone, Debug)]
struct SearchState<const N: usize> {
	position: [(NodeNameHash, Time); N],
	opened: BitHashSet64<NodeName>,
	accumulated: Pressure,
	rate: Pressure,
	minute: Time
}
#[derive(Debug)]
enum Action {
	Travel,
	Walk(NodeNameHash, Time),
	Open(NodeNameHash, Pressure)
}
impl<const N: usize> SearchState<N> {
	pub fn new(position: [NodeNameHash; N]) -> Self {
		Self {
			position: position.map(|p| (p, 0)),
			opened: Default::default(),
			accumulated: 0, rate: 0, minute: 0
		}
	}

	pub fn possible_actions(&self, i: usize, graph: &Graph, dest: &mut Vec<Action>) -> anyhow::Result<()> {
		let pos = self.position[i];
		if pos.1 > 1 {
			dest.push(Action::Travel);
			return Ok(());
		}

		let node = graph.nodes.get(&pos.0).context("Invalid current position")?;

		if !self.opened.contains(&pos.0) {
			if !(0 .. i).any(|i2| self.position[i2].0 == pos.0) {
				dest.push(Action::Open(pos.0, node.rate));
			}
		}

		for edge in node.edges.iter() {
			dest.push(Action::Walk(edge.end, edge.cost));
		}

		Ok(())
	}

	pub fn perform_action(&mut self, i: usize, action: &Action) -> anyhow::Result<()> {
		match action {
			Action::Travel => {
				self.position[i].1 = self.position[i].1.checked_sub(1).context("Invalid Travel action")?;
			}
			Action::Open(valve, rate) => {
				if !self.opened.insert(*valve) { anyhow::bail!("Already opened") };
				self.rate += rate;
			},
			Action::Walk(dest, time) => {
				self.position[i] = (*dest, *time);
			}
		}

		Ok(())
	}

	pub fn tick(&self) -> Self {
		Self {
			position: self.position,
			opened: self.opened.clone(),
			accumulated: self.accumulated + self.rate,
			rate: self.rate,
			minute: self.minute + 1
		}
	}

	pub fn minimum_projection(&self, until: Time) -> Pressure {
		self.accumulated + self.rate * (until - self.minute) as Pressure
	}

	pub fn maximum_projection(&self, until: Time, graph: &Graph) -> Pressure {
		let mut res = self.minimum_projection(until);

		for (i, rate) in graph.sorted_rates.iter().filter(|s| self.opened.contains(&s.0)).enumerate() {
			res += rate.1 * until.saturating_sub(self.minute + 1 + (i as Time / N as Time) * 2) as Pressure;
		}

		res
	}
}


fn solve<const N: usize>(
	graph: &Graph,
	start: NodeNameHash,
	max_minutes: Time
) -> anyhow::Result<Pressure> {
	let mut active = VecDeque::new();
	active.push_back(SearchState::new([start; N]));

	let mut best_minimum_projection: Pressure = 0;

	let mut action_pools: [Vec<Action>; N] = [(); N].map(|_| Vec::new());

	let mut debug_current_minute = 0;
	while let Some(state) = active.pop_front() {
		if state.minute > debug_current_minute {
			debug_current_minute = state.minute;
			log::debug!("Current minute: {:0>2}, branches: {}", debug_current_minute, active.len());
		}

		best_minimum_projection = best_minimum_projection.max(state.minimum_projection(max_minutes));
		
		if state.maximum_projection(max_minutes, graph) < best_minimum_projection {
			log::trace!("Culled state {:?}", state);
			continue;
		}
		if state.minute >= max_minutes {
			log::trace!("Reached {} minutes with {:?}", max_minutes, state);
			continue;
		}
		
		for i in 0 .. N {
			action_pools[i].clear();
			state.possible_actions(i, graph, &mut action_pools[i])?;
		}

		for combination in Combinations::new(&action_pools) {			
			let mut new_state = state.tick();

			for i in 0 .. N {
				new_state.perform_action(i, combination[i])?;
			}

			active.push_back(new_state);
		}
	}

	Ok(best_minimum_projection)
}

const START_VALVE: NodeName = NodeName([b'A', b'A']);
const MAX_MINUTES: Time = 30;
const MAX_MINUTES2: Time = 26;

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let (valve_graph, start): (Graph, NodeNameHash) = {
		let mut assigner = BitHash64Assigner::new();
		let mut nodes: HashMap<NodeNameHash, Node> = HashMap::new();
		let mut rates: Vec<(NodeNameHash, Pressure)> = Vec::new();

		let start = assigner.assign(START_VALVE)?;

		for line in input.split('\n').filter(|s| !s.is_empty()) {
			let (name, rate, reachable) = base::match_tokens!(
				line.split([' ', '=', ',', ';']).filter(|s| !s.is_empty());
				"Valve", name: NodeName, "has", "flow", "rate", rate: FromStrToTryFromAdapter<Pressure> {.0},
				"tunnel" | "tunnels", "leads" | "lead", "to", "valve" | "valves", ...reachable: Vec<NodeName>
			)?;

			log::trace!("Valve {}, rate {}, reachable {:?}", name, rate, reachable);

			let name = assigner.assign(name)?;
			nodes.insert(
				name,
				Node {
					rate,
					edges: reachable.into_iter().map(
						|r| Edge { cost: 1, end: assigner.assign(r).unwrap() }
					).filter(|edge| edge.end != name).collect()
				}
			);

			if rate > 0 {
				rates.push((name, rate));
			}
		}
		rates.sort_by(|a, b| a.1.cmp(&b.1).reverse());

		{
			let collapsible_nodes: Vec<NodeNameHash> = nodes.iter().filter(|(_, node)| node.rate == 0).map(|(name, _)| *name).collect();
			for name in collapsible_nodes {
				if name == start {
					continue;
				}

				let node = nodes.remove(&name).unwrap();

				for (&other_name, other_node) in nodes.iter_mut() {
					if let Some((edge_i, _)) = other_node.edges.iter().enumerate().find(|(_, e)| e.end == name) {
						let cost = other_node.edges.swap_remove(edge_i).cost;

						for edge in node.edges.iter().filter(|e| e.end != other_name) {
							log::trace!(
								"Collapsing edge from {} through {} to {}, cost {}",
								assigner.original(other_name).unwrap(),
								assigner.original(name).unwrap(),
								assigner.original(edge.end).unwrap(),
								cost + edge.cost
							);

							other_node.edges.push(Edge { cost: cost + edge.cost, end: edge.end });
						}
					}
				}
			}
		}

		(Graph { nodes, sorted_rates: rates, assigner }, start)
	};
	
	log::trace!("Valve graph: {:#?}", valve_graph);

	// part1
	let best_accumulated = solve::<1>(&valve_graph, start, MAX_MINUTES)?;
	println!("Best accumulated: {}", best_accumulated);

	// part2
	let best_accumulated2 = solve::<2>(&valve_graph, start, MAX_MINUTES2)?;
	println!("Best accumulated2: {}", best_accumulated2);

	Ok(())
}
