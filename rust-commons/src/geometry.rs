use std::{
	fmt,
	ops::{Add, Sub, Mul, Neg, Range}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<const D: usize>([isize; D]);
impl Point<1> {
	pub const fn new(x: isize) -> Self {
		Self([x])
	}

	pub const fn x(&self) -> isize {
		self.0[0]
	}
}
impl Point<2> {
	pub const fn new(x: isize, y: isize) -> Self {
		Self([x, y])
	}

	pub const fn x(&self) -> isize {
		self.0[0]
	}

	pub const fn y(&self) -> isize {
		self.0[1]
	}

	pub fn rot2d_clockwise(self) -> Self {
		let sx = self.0[0].signum();
		let sy = self.0[1].signum();

		Self([
			self.0[0] + if sy == 0 { -sx } else { sy },
			self.0[1] + if sx == 0 { -sy } else { -sx }
		])
	}
}
impl Point<3> {
	pub const fn new(x: isize, y: isize, z: isize) -> Self {
		Self([x, y, z])
	}

	pub const fn x(&self) -> isize {
		self.0[0]
	}

	pub const fn y(&self) -> isize {
		self.0[1]
	}

	pub const fn z(&self) -> isize {
		self.0[2]
	}
}
impl<const D: usize> Point<D> {
	pub const MIN: Self = Self([isize::MIN; D]);
	pub const MAX: Self = Self([isize::MAX; D]);

	pub fn min(self, other: Self) -> Self {
		let mut v = [0; D];
		for i in 0 .. D {
			v[i] = self.0[i].min(other.0[i]);
		}

		Self(v)
	}

	pub fn max(self, other: Self) -> Self {
		let mut v = [0; D];
		for i in 0 .. D {
			v[i] = self.0[i].max(other.0[i]);
		}

		Self(v)
	}


	pub fn length_manhattan(self) -> isize {
		self.0.map(|v| v.abs()).into_iter().sum()
	}
}
impl<const D: usize> Neg for Point<D> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self(self.0.map(|v| -v))
	}
}
impl<const D: usize> Add<Self> for Point<D> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut v = [0; D];
		for i in 0 .. D {
			v[i] = self.0[i] + rhs.0[i];
		}

		Self(v)
	}
}
impl<const D: usize> Sub<Self> for Point<D> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let mut v = [0; D];
		for i in 0 .. D {
			v[i] = self.0[i] - rhs.0[i];
		}

		Self(v)
	}
}
impl<const D: usize> Mul<isize> for Point<D> {
	type Output = Self;

	fn mul(self, rhs: isize) -> Self::Output {
		Self(self.0.map(|v| v * rhs))
	}
}
impl<const D: usize> fmt::Display for Point<D> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if D == 0 {
			return write!(f, "()");
		}
		
		write!(f, "({}", self.0[0])?;

		for i in 1 .. D {
			write!(f, ", {}", self.0[i])?;
		}

		write!(f, ")")?;

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle<const D: usize> {
	pub min: Point<D>,
	pub max: Point<D>
}
impl Rectangle<2> {
	pub fn points_iter(&self) -> impl Iterator<Item = Point<2>> {
		struct PointsIter {
			min: Point<2>, max: Point<2>,
			x: isize, y: isize
		}
		impl PointsIter {
			pub fn new(rectangle: Rectangle<2>) -> Self {
				Self { min: rectangle.min, max: rectangle.max, x: rectangle.min.x(), y: rectangle.min.y() }
			}
		}
		impl Iterator for PointsIter {
			type Item = Point<2>;

			#[inline(always)]
			fn next(&mut self) -> Option<Self::Item> {
				if self.y >= self.max.y() {
					return None;
				}

				let result = Point::<2>::new(self.x, self.y);

				self.x += 1;
				if self.x >= self.max.x() {
					self.x = self.min.x();
					self.y += 1;
				}

				Some(result)
			}
		}

		PointsIter::new(*self)
	}
}
impl Rectangle<3> {
	pub fn points_iter(&self) -> impl Iterator<Item = Point<3>> {
		struct PointsIter {
			min: Point<3>, max: Point<3>,
			at: Point<3>
		}
		impl PointsIter {
			pub fn new(rectangle: Rectangle<3>) -> Self {
				Self { min: rectangle.min, max: rectangle.max, at: rectangle.min }
			}
		}
		impl Iterator for PointsIter {
			type Item = Point<3>;

			#[inline(always)]
			fn next(&mut self) -> Option<Self::Item> {
				if self.at.z() >= self.max.z() {
					return None;
				}

				let at = self.at;

				self.at.0[0] += 1;
				if self.at.x() >= self.max.x() {
					self.at.0[0] = self.min.x();
					self.at.0[1] += 1;

					if self.at.y() >= self.max.y() {
						self.at.0[1] = self.min.y();
						self.at.0[2] += 1;
					}
				}

				Some(at)
			}
		}

		PointsIter::new(*self)
	}
}
impl<const D: usize> Rectangle<D> {
	pub fn size(&self) -> Point<D> {
		self.max - self.min
	}

	pub fn area(&self) -> usize {
		let size = self.size();

		size.0.map(|v| v.max(0) as usize).into_iter().product()
	}

	pub fn contains(&self, point: &Point<D>) -> bool {		
		for i in 0 .. D {
			if !(point.0[i] >= self.min.0[i] && point.0[i] < self.max.0[i]) {
				return false;
			}
		}
		
		true
	}

	pub fn intersection(self, rhs: &Self) -> Self {
		let min = self.min.max(rhs.min);
		let max = self.max.min(rhs.max);

		Self { min, max }
	}
}
impl<const D: usize> Add<Point<D>> for Rectangle<D> {
	type Output = Self;

	fn add(self, rhs: Point<D>) -> Self::Output {
		Self {
			min: self.min + rhs,
			max: self.max + rhs
		}
	}
}
impl<const D: usize> fmt::Display for Rectangle<D> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[{}, {}]", self.min, self.max)
	}
}

#[derive(Debug, Clone)]
pub struct Circle2 {
	center: Point<2>,
	radius: isize
}
impl Circle2 {
	pub fn new(center: Point<2>, across: Point<2>) -> Self {
		let radius = across - center;
		let radius = radius.length_manhattan();

		Self { center, radius }
	}

	pub fn new_radius(center: Point<2>, radius: isize) -> Self {
		Self::new(center, center + Point::<2>::new(radius, 0))
	}

	pub fn center(&self) -> Point<2> {
		self.center
	}
	
	pub fn radius(&self) -> isize {
		self.radius
	}

	pub fn contains_manhattan(&self, point: Point::<2>) -> bool {
		(point - self.center).length_manhattan() <= self.radius
	}

	pub fn disk_points(&self) -> impl Iterator<Item = Point<2>> {
		struct DiskPointsIter {
			center: Point<2>,
			current: Point<2>
		}
		impl Iterator for DiskPointsIter {
			type Item = Point<2>;

			fn next(&mut self) -> Option<Self::Item> {
				if self.current.x() == 0 && self.current.y() == 0 {
					return None;
				} else {
					let result = self.current;
					
					self.current = self.current.rot2d_clockwise();
					if self.current.x().is_positive() && self.current.y() == 0 {
						self.current = Point::<2>::new(0, 0);
					}

					Some(self.center + result)
				}
			}
		}

		DiskPointsIter {
			center: self.center,
			current: Point::<2>::new(self.radius, 0)
		}
	}

	pub fn bounding_box(&self) -> Rectangle<2> {
		Rectangle {
			min: -Point::<2>::new(self.radius, self.radius),
			max: Point::<2>::new(self.radius + 1, self.radius + 1)
		} + self.center
	}

	pub fn inner_bounding_box(&self) -> Rectangle<2> {
		Rectangle {
			min: -Point::<2>::new(self.radius / 2, self.radius / 2),
			max: Point::<2>::new(self.radius / 2 + 1, self.radius / 2 + 1)
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
	bounding_box: Rectangle<2>
}
impl<T> Grid2<T> {
	pub fn new_width(cells: Vec<T>, width: isize) -> anyhow::Result<Self> {
		anyhow::ensure!(width != 0 && cells.len() > 0 && (cells.len() % width.abs() as usize == 0), "cells len must be divisible by width");

		let height = cells.len() as isize / width;
		let bounding_box = if width < 0 {
			Rectangle {
				min: Point::<2>::new(width, height),
				max: Point::<2>::new(0, 0)
			}
		} else {
			Rectangle {
				min: Point::<2>::new(0, 0),
				max: Point::<2>::new(width, height)
			}
		};

		Self::new(cells, bounding_box)
	}

	pub fn new(cells: Vec<T>, bounding_box: Rectangle<2>) -> anyhow::Result<Self> {
		let size = bounding_box.size();
		anyhow::ensure!(size.x().abs() as usize * size.y().abs() as usize == cells.len(), "bounding_box must define the same size as cells len");

		Ok(Self { cells, bounding_box: bounding_box })
	}

	pub fn bounding_box(&self) -> Rectangle<2> {
		self.bounding_box
	}

	pub fn width(&self) -> isize {
		self.bounding_box().size().x()
	}

	pub fn height(&self) -> isize {
		self.bounding_box().size().y()
	}

	pub fn x_range(&self) -> Range<isize> {
		self.bounding_box.min.x() .. self.bounding_box.max.x()
	}

	pub fn y_range(&self) -> Range<isize> {
		self.bounding_box.min.y() .. self.bounding_box.max.y()
	}

	pub fn index_to_coords(&self, index: usize) -> Option<Point<2>> {
		if index >= self.cells.len() {
			None
		} else {
			let width = self.width().abs() as usize;
			let x = index % width;
			let y = index / width;

			Some(self.bounding_box.min + Point::<2>::new(x as isize, y as isize))
		}
	}

	pub fn get(&self, at: Point<2>) -> Option<&T> {
		let at = at - self.bounding_box.min;
		self.get_relative(at)
	}

	pub fn get_relative(&self, at: Point<2>) -> Option<&T> {
		let size = self.bounding_box.size();
		if at.x() < 0 || at.x() >= size.x() || at.y() < 0 || at.y() >= size.y() {
			return None;
		}

		Some(&self.cells[at.x() as usize + at.y() as usize * size.x().abs() as usize])
	}

	pub fn get_mut(&mut self, at: Point<2>) -> Option<&mut T> {
		let at = at - self.bounding_box.min;
		self.get_mut_relative(at)
	}

	pub fn get_mut_relative(&mut self, at: Point<2>) -> Option<&mut T> {
		let size = self.bounding_box.size();
		if at.x() < 0 || at.x() >= size.x() || at.y() < 0 || at.y() >= size.y() {
			return None;
		}

		Some(&mut self.cells[at.x() as usize + at.y() as usize * size.x().abs() as usize])
	}

	pub fn shift(&mut self, shift: Point<2>) {
		self.bounding_box = self.bounding_box + shift;
	}
}
impl<T: Copy> Grid2<T> {
	pub fn new_fill(fill: T, bounding_box: Rectangle<2>) -> anyhow::Result<Self> {
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
				write!(f, "{}", self.get_relative(Point::<2>::new(x, y)).unwrap())?;
			}
			writeln!(f)?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use std::collections::HashSet;
	use super::{Point, Rectangle, Circle2};

	type Point2 = Point::<2>;

	#[test]
	fn test_rectangle_intersection() {
		let a = Rectangle { min: Point2::new(0, 0), max: Point2::new(2, 2) };
		let b = Rectangle { min: Point2::new(-2, 2), max: Point2::new(0, 3) };
		let c = Rectangle { min: Point2::new(-2, -2), max: Point2::new(1, 1) };
		let d = Rectangle { min: Point2::new(-1, -1), max: Point2::new(-1, -1) };

		// self
		assert_eq!(a.intersection(&a), a);
		// empty
		assert_eq!(d.intersection(&a), Rectangle { min: Point2::new(0, 0), max: Point2::new(-1, -1) });
		// empty self
		assert_eq!(d.intersection(&d), d);
		// no intersection
		assert_eq!(b.intersection(&a), Rectangle { min: Point2::new(0, 2), max: Point2::new(0, 2) });
		// negative coordinates
		assert_eq!(c.intersection(&a), Rectangle { min: Point2::new(0, 0), max: Point2::new(1, 1) });
	}

	#[test]
	fn test_rot_clockwise() {
		assert_eq!(Point2::new(1, 0).rot2d_clockwise(), Point2::new(0, -1));
		assert_eq!(Point2::new(2, 0).rot2d_clockwise(), Point2::new(1, -1));
		assert_eq!(Point2::new(-2, 2).rot2d_clockwise(), Point2::new(-1, 3));
		assert_eq!(Point2::new(-1, -3).rot2d_clockwise(), Point2::new(-2, -2));
		assert_eq!(Point2::new(2, 1).rot2d_clockwise(), Point2::new(3, 0));
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