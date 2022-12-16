use std::{
	fmt,
	ops::{Add, Sub, Mul, Range, Neg}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2 {
	pub x: isize,
	pub y: isize
}
impl Point2 {
	pub const fn new(x: isize, y: isize) -> Self {
		Self { x, y }
	}

	pub fn min(self, other: Self) -> Self {
		Point2 { x: self.x.min(other.x), y: self.y.min(other.y) }
	}

	pub fn max(self, other: Self) -> Self {
		Point2 { x: self.x.max(other.x), y: self.y.max(other.y) }
	}

	pub fn length_manhattan(self) -> isize {
		self.x.abs() + self.y.abs()
	}

	pub fn rot_clockwise(self) -> Point2 {
		let sx = self.x.signum();
		let sy = self.y.signum();
		
		self + Point2::new(
			if sy == 0 { -sx } else { sy },
			if sx == 0 { -sy } else { -sx }
		)
	}
}
impl Neg for Point2 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self { x: -self.x, y: -self.y }
	}
}
impl Add<Self> for Point2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}
impl Sub<Self> for Point2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}
impl Mul<isize> for Point2 {
	type Output = Self;

	fn mul(self, rhs: isize) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}
impl fmt::Display for Point2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle2 {
	pub min: Point2,
	pub max: Point2
}
impl Rectangle2 {
	pub fn size(&self) -> Point2 {
		self.max - self.min
	}

	pub fn area(&self) -> usize {
		let size = self.size();
		size.x.max(0) as usize * size.y.max(0) as usize
	}

	pub fn intersection(self, rhs: &Self) -> Self {
		let min = self.min.max(rhs.min);
		let max = self.max.min(rhs.max);

		Self { min, max }
	}

	pub fn contains(&self, point: &Point2) -> bool {
		point.x >= self.min.x && point.x <= self.max.x
		&& point.y >= self.min.y && point.y <= self.max.y
	}

	pub fn points_iter(&self) -> impl Iterator<Item = Point2> {
		struct PointsIter {
			min: Point2, max: Point2,
			x: isize, y: isize
		}
		impl PointsIter {
			pub fn new(rectangle: Rectangle2) -> Self {
				Self { min: rectangle.min, max: rectangle.max, x: rectangle.min.x, y: rectangle.min.y }
			}
		}
		impl Iterator for PointsIter {
			type Item = Point2;

			#[inline(always)]
			fn next(&mut self) -> Option<Self::Item> {
				if self.y >= self.max.y {
					return None;
				}

				let result = Point2::new(self.x, self.y);

				self.x += 1;
				if self.x >= self.max.x {
					self.x = self.min.x;
					self.y += 1;
				}

				Some(result)
			}
		}

		PointsIter::new(*self)
	}
}
impl Add<Point2> for Rectangle2 {
	type Output = Self;

	fn add(self, rhs: Point2) -> Self::Output {
		Self {
			min: self.min + rhs,
			max: self.max + rhs
		}
	}
}
impl fmt::Display for Rectangle2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.min, self.max)
	}
}

#[derive(Debug, Clone)]
pub struct Circle2 {
	center: Point2,
	radius: isize
}
impl Circle2 {
	pub fn new(center: Point2, across: Point2) -> Self {
		let radius = across - center;
		let radius = radius.x.abs() + radius.y.abs();

		Self { center, radius }
	}

	pub fn new_radius(center: Point2, radius: isize) -> Self {
		Self::new(center, center + Point2::new(radius, 0))
	}

	pub fn center(&self) -> Point2 {
		self.center
	}
	
	pub fn radius(&self) -> isize {
		self.radius
	}

	pub fn contains_manhattan(&self, point: Point2) -> bool {
		(point - self.center).length_manhattan() <= self.radius
	}

	pub fn disk_points(&self) -> impl Iterator<Item = Point2> {
		struct DiskPointsIter {
			center: Point2,
			current: Point2
		}
		impl Iterator for DiskPointsIter {
			type Item = Point2;

			fn next(&mut self) -> Option<Self::Item> {
				if self.current.x == 0 && self.current.y == 0 {
					return None;
				} else {
					let result = self.current;
					
					self.current = self.current.rot_clockwise();
					if self.current.x.is_positive() && self.current.y == 0 {
						self.current = Point2::new(0, 0);
					}

					Some(self.center + result)
				}
			}
		}

		DiskPointsIter {
			center: self.center,
			current: Point2::new(self.radius, 0)
		}
	}

	pub fn bounding_box(&self) -> Rectangle2 {
		Rectangle2 {
			min: -Point2::new(self.radius, self.radius),
			max: Point2::new(self.radius + 1, self.radius + 1)
		} + self.center
	}

	pub fn inner_bounding_box(&self) -> Rectangle2 {
		Rectangle2 {
			min: -Point2::new(self.radius / 2, self.radius / 2),
			max: Point2::new(self.radius / 2 + 1, self.radius / 2 + 1)
		} + self.center
	}
}
impl fmt::Display for Circle2 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}~{}", self.center, self.radius)
	}
}

#[derive(Debug, Clone)]
pub struct Grid2<T> {
	cells: Vec<T>,
	bounding_box: Rectangle2
}
impl<T> Grid2<T> {
	pub fn new_width(cells: Vec<T>, width: isize) -> anyhow::Result<Self> {
		anyhow::ensure!(width != 0 && cells.len() > 0 && (cells.len() % width.abs() as usize == 0), "cells len must be divisible by width");

		let height = cells.len() as isize / width;
		let bounding_box = if width < 0 {
			Rectangle2 {
				min: Point2 { x: width, y: height },
				max: Point2 { x: 0, y: 0 }
			}
		} else {
			Rectangle2 {
				min: Point2 { x: 0, y: 0 },
				max: Point2 { x: width, y: height }
			}
		};

		Self::new(cells, bounding_box)
	}

	pub fn new(cells: Vec<T>, bounding_box: Rectangle2) -> anyhow::Result<Self> {
		let size = bounding_box.size();
		anyhow::ensure!(size.x.abs() as usize * size.y.abs() as usize == cells.len(), "bounding_box must define the same size as cells len");

		Ok(Self { cells, bounding_box: bounding_box })
	}

	pub fn bounding_box(&self) -> Rectangle2 {
		self.bounding_box
	}

	pub fn width(&self) -> isize {
		self.bounding_box().size().x
	}

	pub fn height(&self) -> isize {
		self.bounding_box().size().y
	}

	pub fn x_range(&self) -> Range<isize> {
		self.bounding_box.min.x .. self.bounding_box.max.x
	}

	pub fn y_range(&self) -> Range<isize> {
		self.bounding_box.min.y .. self.bounding_box.max.y
	}

	pub fn index_to_coords(&self, index: usize) -> Option<Point2> {
		if index >= self.cells.len() {
			None
		} else {
			let width = self.width().abs() as usize;
			let x = index % width;
			let y = index / width;

			Some(self.bounding_box.min + Point2::new(x as isize, y as isize))
		}
	}

	pub fn get(&self, at: Point2) -> Option<&T> {
		let at = at - self.bounding_box.min;
		self.get_relative(at)
	}

	pub fn get_relative(&self, at: Point2) -> Option<&T> {
		let size = self.bounding_box.size();
		if at.x < 0 || at.x >= size.x || at.y < 0 || at.y >= size.y {
			return None;
		}

		Some(&self.cells[at.x as usize + at.y as usize * size.x.abs() as usize])
	}

	pub fn get_mut(&mut self, at: Point2) -> Option<&mut T> {
		let at = at - self.bounding_box.min;
		self.get_mut_relative(at)
	}

	pub fn get_mut_relative(&mut self, at: Point2) -> Option<&mut T> {
		let size = self.bounding_box.size();
		if at.x < 0 || at.x >= size.x || at.y < 0 || at.y >= size.y {
			return None;
		}

		Some(&mut self.cells[at.x as usize + at.y as usize * size.x.abs() as usize])
	}

	pub fn shift(&mut self, shift: Point2) {
		self.bounding_box = self.bounding_box + shift;
	}
}
impl<T: Copy> Grid2<T> {
	pub fn new_fill(fill: T, bounding_box: Rectangle2) -> anyhow::Result<Self> {
		Self::new(vec![fill; bounding_box.area()], bounding_box)
	}

	pub fn fill(&mut self, fill: T) {
		for cell in self.cells.iter_mut() {
			*cell = fill;
		}
	}
}
impl<T: fmt::Display> fmt::Display for Grid2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for y in 0 .. self.height() {
			for x in 0 .. self.width() {
				write!(f, "{}", self.get_relative(Point2 { x, y }).unwrap())?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use std::collections::HashSet;
	use super::{Point2, Rectangle2, Circle2};

	#[test]
	fn test_rectangle_intersection() {
		let a = Rectangle2 { min: Point2::new(0, 0), max: Point2::new(2, 2) };
		let b = Rectangle2 { min: Point2::new(-2, 2), max: Point2::new(0, 3) };
		let c = Rectangle2 { min: Point2::new(-2, -2), max: Point2::new(1, 1) };
		let d = Rectangle2 { min: Point2::new(-1, -1), max: Point2::new(-1, -1) };

		// self
		assert_eq!(a.intersection(&a), a);
		// empty
		assert_eq!(d.intersection(&a), Rectangle2 { min: Point2::new(0, 0), max: Point2::new(-1, -1) });
		// empty self
		assert_eq!(d.intersection(&d), d);
		// no intersection
		assert_eq!(b.intersection(&a), Rectangle2 { min: Point2::new(0, 2), max: Point2::new(0, 2) });
		// negative coordinates
		assert_eq!(c.intersection(&a), Rectangle2 { min: Point2::new(0, 0), max: Point2::new(1, 1) });
	}

	#[test]
	fn test_rot_clockwise() {
		assert_eq!(Point2::new(1, 0).rot_clockwise(), Point2::new(0, -1));
		assert_eq!(Point2::new(2, 0).rot_clockwise(), Point2::new(1, -1));
		assert_eq!(Point2::new(-2, 2).rot_clockwise(), Point2::new(-1, 3));
		assert_eq!(Point2::new(-1, -3).rot_clockwise(), Point2::new(-2, -2));
		assert_eq!(Point2::new(2, 1).rot_clockwise(), Point2::new(3, 0));
	}

	#[test]
	fn test_disk_intersection() {
		/*
		x|01234567890123
		0|       b      |
		1|  a   b b     |
		2| a a b   b    |
		3|a A X  B  X   |
		4| a a b   X d  |
		5|  a   b X   d |
		6|       XCcD  d|
		7|        X   d |
		8|         d d  |
		9|          d   |
		*/
		let a = Circle2::new(Point2::new(2, 3), Point2::new(0, 3));
		let b = Circle2::new(Point2::new(7, 3), Point2::new(4, 3));
		let c = Circle2::new(Point2::new(8, 6), Point2::new(8, 5));
		let d = Circle2::new(Point2::new(10, 6), Point2::new(7, 6));

		fn disk_intersection(a: &Circle2, b: &Circle2) -> Vec<Point2> {
			let hs1: HashSet<Point2> = a.disk_points().collect();
			b.disk_points().filter(|p| hs1.contains(p)).collect()
		}

		assert_eq!(disk_intersection(&a, &b), &[Point2::new(4, 3)]);
		assert_eq!(disk_intersection(&a, &c), &[]);
		assert_eq!(disk_intersection(&b, &c), &[Point2::new(8, 5), Point2::new(7, 6)]);
		assert_eq!(disk_intersection(&c, &d), &[Point2::new(8, 5), Point2::new(7, 6), Point2::new(8, 7)]);
	}
}