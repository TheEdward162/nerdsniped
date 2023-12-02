use std::{collections::HashSet, io::Read};

use anyhow::Context;

use aoc_commons as aoc;
use aoc::{anyhow, log};

use aoc::{
	geometry::{Point, Rectangle, Circle2},
	macros::FromStrToTryFromAdapter
};

type Point2 = Point<2>;
type Rectangle2 = Rectangle<2>;

const SCAN_ROW: isize = 2000000;
const DISTRESS_BOUNDARIES: Rectangle2 = Rectangle2 {
	min: Point2::new(0, 0),
	max: Point2::new(4000000 + 1, 4000000 + 1)
};

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut beacons_and_sensors = HashSet::<Point2>::new();
	let mut circles = Vec::<Circle2>::new();
	for line in input.split('\n').filter(|s| !s.is_empty()) {
		let (sx, sy, bx, by) = aoc::match_tokens!(
			line.split([' ', '=', ',', ':']).filter(|s| !s.is_empty());
			"Sensor", "at",
			"x", sx: FromStrToTryFromAdapter<isize>,
			"y", sy: FromStrToTryFromAdapter<isize>,
			"closest", "beacon", "is", "at",
			"x", bx: FromStrToTryFromAdapter<isize>,
			"y", by: FromStrToTryFromAdapter<isize>
		)?;

		let sensor = Point2::new(sx.0, sy.0);
		let beacon = Point2::new(bx.0, by.0);
		beacons_and_sensors.insert(sensor);
		beacons_and_sensors.insert(beacon);

		let circle = Circle2::new(sensor, beacon);
		log::debug!("Circle: {}", circle);
		circles.push(circle);
	}

	let bounding_box = beacons_and_sensors.iter().fold(
		Rectangle2 { min: Point2::new(isize::MAX, isize::MAX), max: Point2::new(isize::MIN, isize::MIN) },
		|acc, &point| Rectangle2 {
			min: acc.min.min(point),
			max: acc.max.max(point)
		}
	);

	// part1
	let b1 = Rectangle2 {
		min: Point2::new(bounding_box.min.x() - bounding_box.size().x().abs(), SCAN_ROW),
		max: Point2::new(bounding_box.max.x() + bounding_box.size().x().abs(), SCAN_ROW + 1)
	};
	log::debug!("Boundaries: {}", b1);
	let mut cleared_count = 0;
	for point in b1.points_iter().filter(|p| !beacons_and_sensors.contains(p)) {
		if circles.iter().any(|c| c.contains_manhattan(point)) {
			cleared_count += 1;
		}
	}
	log::info!("Done part 1");
	
	// part2
	let circle_outside_points: Vec<HashSet<Point2>> = circles.iter().map(
		|c| {
			let outside_circle = Circle2::new_radius(c.center(), c.radius() + 1);

			let points = outside_circle.disk_points().filter(
				|p| DISTRESS_BOUNDARIES.contains(p)
			);

			points.collect()
		}
	).collect();

	let mut empty_point = None::<Point2>;
	'toploop: for i1 in 0 .. circles.len() {
		let points1 = &circle_outside_points[i1];

		for i2 in i1 + 1 .. circles.len() {
			let points2 = &circle_outside_points[i2];

			log::debug!("Scanning circle intersection: {} and {}", i1, i2);			

			for &point in points1.intersection(&points2) {
				log::trace!("Scanning point: {}", point);
				if !circles.iter().any(|c| c.contains_manhattan(point)) {
					empty_point = Some(point);
					break 'toploop;
				}
			}
		}
	}
	log::info!("Done part 2");

	println!("Cleared count: {}", cleared_count);
	println!("Tuning frequency: {}", empty_point.map(|p| p.x() * 4000000 + p.y()).context("Did not find empty point")?);

	Ok(())
}
