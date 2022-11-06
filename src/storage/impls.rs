mod btree_map;
mod hash_map;
mod vec;

#[cfg(feature = "slab-storage")]
mod slab;

pub struct MapStorageIter<I> {
	inner: I,
}

impl<I> MapStorageIter<I> {
	pub fn new(inner: I) -> Self {
		Self { inner }
	}
}

impl<'a, T: 'a, I: Iterator<Item = (&'a usize, T)>> Iterator for MapStorageIter<I> {
	type Item = (usize, T);

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|(k, v)| (*k, v))
	}
}
