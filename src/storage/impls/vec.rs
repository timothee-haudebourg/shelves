use crate::storage::*;

impl<T> Storage for Vec<T> {
	type Value = T;

	fn get(&self, index: usize) -> Option<&Self::Value> {
		self.as_slice().get(index)
	}

	fn capacity(&self) -> usize {
		self.capacity()
	}

	fn len(&self) -> usize {
		self.len()
	}
}

impl<T> StorageIter for Vec<T> {
	type Iter<'a> where Self: 'a = std::iter::Enumerate<std::slice::Iter<'a, T>>;

	fn iter(&self) -> Self::Iter<'_> {
		self.as_slice().iter().enumerate()
	}
}

impl<T> StorageIntoIter for Vec<T> {
	type IntoIter = std::iter::Enumerate<std::vec::IntoIter<T>>;

	fn into_iter(self) -> Self::IntoIter {
		std::iter::IntoIterator::into_iter(self).enumerate()
	}
}

impl<T> StorageMut for Vec<T> {
	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Value> {
		self.as_mut_slice().get_mut(index)
	}

	fn clear(&mut self) {
		self.clear()
	}
}

impl<T> StorageIterMut for Vec<T> {
	type IterMut<'a> where Self: 'a = std::iter::Enumerate<std::slice::IterMut<'a, T>>;

	fn iter_mut(&mut self) -> Self::IterMut<'_> {
		self.as_mut_slice().iter_mut().enumerate()
	}
}

impl<T> StorageAllocate for Vec<T> {
	fn allocate(&mut self, value: Self::Value) -> usize {
		let index = self.len();
		self.push(value);
		index
	}
}

impl<T> StorageSet for Vec<T> {
	fn set(&mut self, index: usize, mut value: Self::Value) -> Result<Self::Value, Self::Value> {
		if index < self.len() {
			std::mem::swap(&mut self[index], &mut value);
			Ok(value)
		} else {
			Err(value)
		}
	}
}