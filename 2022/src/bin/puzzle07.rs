use std::io::Read;

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

#[derive(Debug)]
struct NodeFile<'a> {
	pub name: &'a str,
	pub size: usize
}
#[derive(Debug)]
struct NodeDirectory<'a> {
	pub name: &'a str,
	pub nodes: Vec<Node<'a>>,
	pub size_cache: Option<usize>
}
impl<'a> NodeDirectory<'a> {
	pub fn new(name: &'a str) -> NodeDirectory {
		Self {
			name,
			nodes: Vec::new(),
			size_cache: None
		}
	}

	pub fn find_child_mut(&mut self, name: &str) -> Option<&mut Node<'a>> {
		self.nodes.iter_mut().find(|ch| ch.name() == name)
	}
}

#[derive(Debug)]
enum Node<'a> {
	File(NodeFile<'a>),
	Directory(NodeDirectory<'a>)
}
impl<'a> Node<'a> {
	pub fn name(&self) -> &'a str {
		match self {
			Self::File(NodeFile { name, .. }) => name,
			Self::Directory(NodeDirectory { name, .. }) => name
		}
	}

	pub fn compute_size(&mut self) -> usize {
		match self {
			Self::File(NodeFile { size, .. }) => *size,
			Self::Directory(NodeDirectory { size_cache: Some(size), .. }) => *size,
			Self::Directory(NodeDirectory { nodes, size_cache, .. }) => {
				let size = nodes.iter_mut().map(|n| n.compute_size()).sum();
				*size_cache = Some(size);

				size
			}
		}
	}

	pub fn as_dir_mut(&mut self) -> Option<&mut NodeDirectory<'a>> {
		match self {
			Self::File(_) => None,
			Self::Directory(dir) => Some(dir)
		}
	}
}

fn traverse<'node, 'iter, 'tree>(base: &'tree mut NodeDirectory<'node>, path: impl Iterator<Item = &'iter str>) -> anyhow::Result<&'tree mut NodeDirectory<'node>> {
	let mut current = base;
	for part in path {
		match current.find_child_mut(part) {
			None => anyhow::bail!("Destination doesn't exist"),
			Some(Node::File(_)) => anyhow::bail!("Cannot cd into a file"),
			Some(Node::Directory(dir)) => { current = dir; }
		}
	}

	Ok(current)
}

const AT_MOST_SIZE: usize = 100000;
fn sum_small_dirs<'node>(root: &mut Node<'node>, sum: &mut usize) {
	let size = root.compute_size();
	if let Node::Directory(NodeDirectory { nodes, .. }) = root {
		if size <= AT_MOST_SIZE {
			*sum += size;
		}

		for node in nodes.iter_mut() {
			sum_small_dirs(node, sum);
		}
	}
}

const TOTAL_DISK_SPACE: usize = 70000000;
const UNUSED_SPACE_MIN: usize = 30000000;
fn select_minimum_delete<'node>(root: &mut Node<'node>, needed: usize, to_delete: &mut usize) {
	let size = root.compute_size();
	if let Node::Directory(NodeDirectory { nodes, .. }) = root {
		if size >= needed && size < *to_delete {
			*to_delete = size;
		}

		for node in nodes.iter_mut() {
			select_minimum_delete(node, needed, to_delete);
		}
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut root = Node::Directory(NodeDirectory::new("/"));
	let mut current_path: Vec<&str> = Vec::new();

	let mut lines = input.split("\n").filter(|s| !s.is_empty()).peekable();
	while let Some(line) = lines.next() {
		anyhow::ensure!(line.starts_with('$'), "Invalid command: \"{}\"", line);

		let mut command_iter = line.split(' ').skip(1);
		match command_iter.next() {
			Some("ls") => {
				let current = traverse(root.as_dir_mut().unwrap(), current_path.iter().copied())?;
				while lines.peek().map(|l| !l.starts_with('$')).unwrap_or(false) {
					let (meta, name) = lines.next().unwrap().split_once(' ').context("Invalid output line")?;
					let new_node = if meta == "dir" {
						Node::Directory(NodeDirectory::new(name))
					} else {
						Node::File(
							NodeFile { name, size: meta.parse().context("Invalid file size")? }
						)
					};
					current.nodes.push(new_node);					
				}
			}
			Some("cd") => {
				let destination = command_iter.next().context("Invalid cd command")?;
				if destination == "/" {
					current_path.clear();
				} else if destination == ".." {
					current_path.pop().context("Cannot cd outside of /")?;
				} else {
					current_path.push(destination);
				}
			}
			c => anyhow::bail!("Invalid command: {:?}", c)
		}
	}

	log::debug!("Nodes: {:#?}", root);

	let mut small_size_sum = 0;
	sum_small_dirs(&mut root, &mut small_size_sum);
	println!("Small size: {}", small_size_sum);

	let mut minimal_delete = UNUSED_SPACE_MIN;
	let space_needed = UNUSED_SPACE_MIN.checked_sub(
		TOTAL_DISK_SPACE.checked_sub(root.compute_size()).context("Total size overflows disk space")?
	).context("Enough space already available")?;
	select_minimum_delete(&mut root, space_needed, &mut minimal_delete);
	println!("Minimal delete: {}", minimal_delete);

	Ok(())
}
