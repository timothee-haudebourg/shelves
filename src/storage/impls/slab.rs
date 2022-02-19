use crate::storage::*;
use slab::Slab;

impl<T> Storage for Slab<T> {
	type Value = T;

	fn get(&self, index: usize) -> Option<&Self::Value> {
		self.get(index)
	}

	fn capacity(&self) -> usize {
		self.capacity()
	}

	fn len(&self) -> usize {
		self.len()
	}
}

impl<T> StorageIter for Slab<T> {
	type Iter<'a> where Self: 'a = slab::Iter<'a, T>;

	fn iter(&self) -> Self::Iter<'_> {
		self.iter()
	}
}

impl<T> StorageIntoIter for Slab<T> {
	type IntoIter = slab::IntoIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		std::iter::IntoIterator::into_iter(self)
	}
}

impl<T> StorageMut for Slab<T> {
	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Value> {
		self.get_mut(index)
	}

	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> StorageIterMut for Slab<T> {
	type IterMut<'a> where Self: 'a = slab::IterMut<'a, T>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut()
	}
}

impl<T> StorageSet for Slab<T> {
	fn set(&mut self, index: usize, mut value: Self::Value) -> Result<Self::Value, Self::Value> {
		if self.contains(index) {
			std::mem::swap(&mut self[index], &mut value);
			Ok(value)
		} else {
			Err(value)
		}
	}
}

impl<T> StorageRemove for Slab<T> {
	fn remove(&mut self, index: usize) -> Option<Self::Value> {
		if self.contains(index) {
			Some(self.remove(index))
		} else {
			None
		}
	}
}