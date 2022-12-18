use std::{
	collections::{HashSet, VecDeque},
	io::Read
};

use anyhow::Context;

use aoc_commons as base;
use base::{anyhow, log};

use base::{
	geometry::{Point, Rectangle}
};

type Point3 = Point<3>;
type Rectangle3 = Rectangle<3>;

fn cube_siders_iter(center: Point3) -> impl Iterator<Item = Point3> {
	[
		center + Point3::new(-1, 0, 0),
		center + Point3::new(1, 0, 0),
		center + Point3::new(0, -1, 0),
		center + Point3::new(0, 1, 0),
		center + Point3::new(0, 0, -1),
		center + Point3::new(0, 0, 1),
	].into_iter()
}

fn count_reachable(
	start: Point3,
	lava: &HashSet<Point3>,
	bounding_box: &Rectangle3,
	seen: &mut HashSet<Point3>
) -> (bool, usize) {
	if lava.contains(&start) {
		return (false, 0);
	}

	let mut reachable = false;
	let mut area = 0;

	let mut search_queue = VecDeque::new();
	search_queue.push_back(start);

	while let Some(current) = search_queue.pop_front() {
		if !bounding_box.contains(&current) {
			reachable = true;
			continue;
		}

		if seen.contains(&current) {
			continue;
		}

		seen.insert(current);

		for side in cube_siders_iter(current) {
			if lava.contains(&side) {
				area += 1;
			} else if !seen.contains(&side) {
				search_queue.push_back(side);
			}
		}
	}

	(reachable, area)

}

fn main() -> anyhow::Result<()> {
	let mut file = base::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut lava: HashSet<Point3> = HashSet::default();
	let mut bounding_box = Rectangle3 { min: Point3::MAX, max: Point3::MIN };

	for line in input.lines().filter(|s| !s.is_empty()) {
		let (x, y, z) = base::match_tokens!(
			line.split(',');
			x: base::macros::FromStrToTryFromAdapter<isize>,
			y: base::macros::FromStrToTryFromAdapter<isize>,
			z: base::macros::FromStrToTryFromAdapter<isize>
		)?;

		let current = Point3::new(x.0, y.0, z.0);
		lava.insert(current);
		bounding_box.min = bounding_box.min.min(current);
		bounding_box.max = bounding_box.max.max(current);
	}

	let mut surface_area = 0;
	let mut reachable_area = 0;

	let bounding_box = Rectangle3 {
		min: bounding_box.min - Point3::new(1, 1, 1),
		max: bounding_box.max + Point3::new(2, 2, 2)
	};
	log::debug!("Total bb: {}", bounding_box);
	let mut seen: HashSet<Point3> = HashSet::new();
	for point in bounding_box.points_iter() {
		log::trace!("Point: {} (is lava: {})", point, lava.contains(&point));

		let (reachable, area) = count_reachable(point, &lava, &bounding_box, &mut seen);
		surface_area += area;
		if reachable {
			reachable_area += area;
		}
	}

	println!("Surface area: {}", surface_area);
	println!("Reachable area: {}", reachable_area);
	log::info!("Done");

	Ok(())
}
