use crate::storage::*;
use std::collections::{hash_map::Entry, HashMap};

impl<T> Storage for HashMap<usize, T> {
	type Value = T;

	fn get(&self, index: usize) -> Option<&Self::Value> {
		self.get(&index)
	}

	fn capacity(&self) -> usize {
		self.capacity()
	}

	fn len(&self) -> usize {
		self.len()
	}
}

impl<T> StorageIter for HashMap<usize, T> {
	type Iter<'a>
	= impl Iterator<Item = (usize, &'a Self::Value)> where
	Self: 'a;

	fn iter(&self) -> Self::Iter<'_> {
		self.iter().map(|(i, t)| (*i, t))
	}
}

impl<T> StorageIntoIter for HashMap<usize, T> {
	type IntoIter = std::collections::hash_map::IntoIter<usize, T>;

	fn into_iter(self) -> Self::IntoIter {
		std::iter::IntoIterator::into_iter(self)
	}
}

impl<T> StorageMut for HashMap<usize, T> {
	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Value> {
		self.get_mut(&index)
	}

	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> StorageIterMut for HashMap<usize, T> {
	type IterMut<'a>
	= impl Iterator<Item = (usize, &'a mut Self::Value)> where
	Self: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut().map(|(i, t)| (*i, t))
	}
}

impl<T> StorageSet for HashMap<usize, T> {
	fn set(&mut self, index: usize, value: Self::Value) -> Result<Self::Value, Self::Value> {
		match self.entry(index) {
			Entry::Occupied(mut entry) => Ok(entry.insert(value)),
			Entry::Vacant(_) => Err(value),
		}
	}
}

impl<T> StorageInsert for HashMap<usize, T> {
	fn insert(&mut self, index: usize, value: Self::Value) -> Option<Self::Value> {
		self.insert(index, value)
	}
}

impl<T> StorageRemove for HashMap<usize, T> {
	fn remove(&mut self, index: usize) -> Option<Self::Value> {
		self.remove(&index)
	}
}
