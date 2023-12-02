use std::mem::MaybeUninit;

pub struct Combinations<'a, T, const N: usize> {
	pools: Option<&'a [Vec<T>; N]>,
	indices: [usize; N]
}
impl<'a, T, const N: usize> Combinations<'a, T, N> {
	pub fn new(pools: &'a [Vec<T>; N]) -> Self {
		Self { pools: Some(pools), indices: [0; N] }
	}

	fn increase_index(&mut self) {
		for i in 0 .. N {
			self.indices[i] = (self.indices[i] + 1) % self.pools.unwrap()[i].len();
			if self.indices[i] != 0 {
				break;
			}
		}

		if self.indices == [0; N] {
			self.pools = None;
		}
	}
}
impl<'a, T, const N: usize> Iterator for Combinations<'a, T, N> {
	type Item = [&'a T; N];

	fn next(&mut self) -> Option<Self::Item> {
		match self.pools {
			None => None,
			Some(pools) => {
				let mut result: [MaybeUninit<&T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

				for i in 0 .. N {
					let index = self.indices[i];
					result[i].write(&pools[i][index]);
				}

				self.increase_index();
				Some(result.map(|t| unsafe { t.assume_init() }))
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::Combinations;

	#[test]
	fn test_combinations() {
		let a = vec![1, 2, 3, 4];
		let b = vec![5, 6, 7];
		let c = vec![8, 9];
		
		let pools = [a, b, c];
		let combinations = Combinations::<usize, 3>::new(&pools);
		assert_eq!(
			combinations.collect::<Vec<_>>(),
			vec![
				[&1, &5, &8],
				[&2, &5, &8],
				[&3, &5, &8],
				[&4, &5, &8],
				[&1, &6, &8],
				[&2, &6, &8],
				[&3, &6, &8],
				[&4, &6, &8],
				[&1, &7, &8],
				[&2, &7, &8],
				[&3, &7, &8],
				[&4, &7, &8],
				//
				[&1, &5, &9],
				[&2, &5, &9],
				[&3, &5, &9],
				[&4, &5, &9],
				[&1, &6, &9],
				[&2, &6, &9],
				[&3, &6, &9],
				[&4, &6, &9],
				[&1, &7, &9],
				[&2, &7, &9],
				[&3, &7, &9],
				[&4, &7, &9],
			]
		);
	}
}