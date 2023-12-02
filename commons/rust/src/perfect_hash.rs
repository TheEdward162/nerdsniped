use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct BitHash64Assigner<T: Eq + Hash> {
	shift: u8,
	map: HashMap<T, BitHash64<T>>
}
impl<T: Eq + Hash> BitHash64Assigner<T> {
	pub fn new() -> Self {
		Self { shift: 0, map: HashMap::new() }
	}

	pub fn assign(&mut self, value: T) -> anyhow::Result<BitHash64<T>> {
		match self.map.get(&value) {
			Some(hash) => Ok(*hash),
			None => if self.shift >= 63 {
				Err(anyhow::anyhow!("Perfect hashing can only work with up to 64 elements"))
			} else {
				let hash = BitHash64(1u64 << self.shift as usize, std::marker::PhantomData);
				self.shift += 1;
				self.map.insert(value, hash);

				Ok(hash)
			}
		}
	}

	pub fn original(&self, hash: BitHash64<T>) -> Option<&T> {
		for (key, &value) in self.map.iter() {
			if value == hash {
				return Some(key);
			}
		}

		None
	}
}

#[derive(Debug)]
pub struct BitHash64<T>(u64, std::marker::PhantomData<T>);
impl<T> BitHash64<T> {
	pub fn assign(values: impl Iterator<Item = T>) -> anyhow::Result<HashMap<Self, T>> {
		let mut map = HashMap::default();
		for (i, v) in values.enumerate() {
			anyhow::ensure!(i < 64, "Perfect hashing can only work with up to 64 elements");

			let key = Self(1 << i, std::marker::PhantomData);
			map.insert(key, v);
		}

		Ok(map)
	}
}
impl<T> Clone for BitHash64<T> {
	fn clone(&self) -> Self {
		Self(self.0, std::marker::PhantomData)
	}
}
impl<T> Copy for BitHash64<T> {}
impl<T> PartialEq for BitHash64<T> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}
impl<T> Eq for BitHash64<T> {}
impl<T> Hash for BitHash64<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}
}

#[derive(Debug, Clone)]
pub struct BitHashSet64<T>(u64, std::marker::PhantomData<T>);
impl<T> BitHashSet64<T> {
	pub fn new() -> Self {
		Self(0, std::marker::PhantomData)
	}

	pub fn insert(&mut self, value: BitHash64<T>) -> bool {
		if self.contains(&value) {
			false
		} else {
			self.0 |= value.0;
			true
		}
	}

	pub fn contains(&self, value: &BitHash64<T>) -> bool {
		(self.0 & value.0) != 0
	}
}
impl<T> Default for BitHashSet64<T> {
	fn default() -> Self {
		Self::new()
	}
}