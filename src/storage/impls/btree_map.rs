use crate::storage::*;
use std::collections::{BTreeMap, btree_map::Entry};

impl<T> Storage for BTreeMap<usize, T> {
	type Value = T;

	fn get(&self, index: usize) -> Option<&Self::Value> {
		self.get(&index)
	}

	fn capacity(&self) -> usize {
		self.len()
	}

	fn len(&self) -> usize {
		self.len()
	}
}

impl<T> StorageIter for BTreeMap<usize, T> {
	type Iter<'a> where Self: 'a = impl Iterator<Item=(usize, &'a Self::Value)>;

	fn iter(&self) -> Self::Iter<'_> {
		self.iter().map(|(i, t)| (*i, t))
	}
}

impl<T> StorageIntoIter for BTreeMap<usize, T> {
	type IntoIter = std::collections::btree_map::IntoIter<usize, T>;

	fn into_iter(self) -> Self::IntoIter {
		std::iter::IntoIterator::into_iter(self)
	}
}

impl<T> StorageMut for BTreeMap<usize, T> {
	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Value> {
		self.get_mut(&index)
	}

	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> StorageIterMut for BTreeMap<usize, T> {
	type IterMut<'a> where Self: 'a = impl Iterator<Item=(usize, &'a mut Self::Value)>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.iter_mut().map(|(i, t)| (*i, t))
	}
}

impl<T> StorageSet for BTreeMap<usize, T> {
	fn set(&mut self, index: usize, value: Self::Value) -> Result<Self::Value, Self::Value> {
		match self.entry(index) {
			Entry::Occupied(mut entry) => Ok(entry.insert(value)),
			Entry::Vacant(_) => Err(value)
		}
	}
}

impl<T> StorageInsert for BTreeMap<usize, T> {
	fn insert(&mut self, index: usize, value: Self::Value) -> Option<Self::Value> {
		self.insert(index, value)
	}
}

impl<T> StorageRemove for BTreeMap<usize, T> {
	fn remove(&mut self, index: usize) -> Option<Self::Value> {
		self.remove(&index)
	}
}