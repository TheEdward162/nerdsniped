use std::{
	fmt,
	ops::{Add, Sub, Mul, Range}
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
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
		size.x.abs() as usize * size.y.abs() as usize
	}
}

#[derive(Debug, Clone)]
pub struct Grid2<T> {
	cells: Vec<T>,
	boundaries: Rectangle2	
}
impl<T> Grid2<T> {
	pub fn new_width(cells: Vec<T>, width: isize) -> anyhow::Result<Self> {
		anyhow::ensure!(width != 0 && cells.len() > 0 && (cells.len() % width.abs() as usize == 0), "cells len must be divisible by width");

		let height = cells.len() as isize / width;
		let boundaries = if width < 0 {
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
		
		Self::new(cells, boundaries)
	}
	
	pub fn new(cells: Vec<T>, boundaries: Rectangle2) -> anyhow::Result<Self> {
		let size = boundaries.size();
		anyhow::ensure!(size.x.abs() as usize * size.y.abs() as usize == cells.len(), "boundaries must define the same size as cells len");

		Ok(Self { cells, boundaries })
	}

	pub fn boundaries(&self) -> Rectangle2 {
		self.boundaries
	}

	pub fn width(&self) -> isize {
		self.boundaries().size().x
	}

	pub fn height(&self) -> isize {
		self.boundaries().size().y
	}

	pub fn x_range(&self) -> Range<isize> {
		self.boundaries.min.x .. self.boundaries.max.x
	}

	pub fn y_range(&self) -> Range<isize> {
		self.boundaries.min.y .. self.boundaries.max.y
	}

	pub fn index_to_coords(&self, index: usize) -> Option<Point2> {
		if index >= self.cells.len() {
			None
		} else {
			let width = self.width().abs() as usize;
			let x = index % width;
			let y = index / width;

			Some(self.boundaries.min + Point2::new(x as isize, y as isize))
		}
	}

	pub fn get(&self, at: Point2) -> Option<&T> {
		let at = at - self.boundaries.min;
		self.get_relative(at)
	}

	pub fn get_relative(&self, at: Point2) -> Option<&T> {
		let size = self.boundaries.size();
		if at.x < 0 || at.x >= size.x || at.y < 0 || at.y >= size.y {
			return None;
		}
		
		Some(&self.cells[at.x as usize + at.y as usize * size.x.abs() as usize])
	}

	pub fn get_mut(&mut self, at: Point2) -> Option<&mut T> {
		let at = at - self.boundaries.min;
		self.get_mut_relative(at)
	}

	pub fn get_mut_relative(&mut self, at: Point2) -> Option<&mut T> {
		let size = self.boundaries.size();
		if at.x < 0 || at.x >= size.x || at.y < 0 || at.y >= size.y {
			return None;
		}
		
		Some(&mut self.cells[at.x as usize + at.y as usize * size.x.abs() as usize])
	}
}
impl<T: Copy> Grid2<T> {
	pub fn new_fill(fill: T, boundaries: Rectangle2) -> anyhow::Result<Self> {
		Self::new(vec![fill; boundaries.area()], boundaries)
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
