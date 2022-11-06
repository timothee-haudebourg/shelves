mod impls;

pub use impls::*;

/// Basic storage trait.
pub trait Storage {
	/// Type of stored values.
	type Value;

	/// Get the value behind the given index, if any.
	fn get(&self, index: usize) -> Option<&Self::Value>;

	/// Returns a lower bound to the capacity of th storage.
	fn capacity(&self) -> usize;

	/// Returns the number of values stored.
	fn len(&self) -> usize;

	/// Checks if the storage is empty.
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

pub trait StorageIter: Storage {
	type Iter<'a>: Iterator<Item = (usize, &'a Self::Value)>
	where
		Self: 'a;

	fn iter(&self) -> Self::Iter<'_>;
}

pub trait StorageIntoIter: Storage {
	type IntoIter: Iterator<Item = (usize, Self::Value)>;

	fn into_iter(self) -> Self::IntoIter;
}

pub trait StorageMut: Storage {
	fn get_mut(&mut self, index: usize) -> Option<&mut Self::Value>;

	fn clear(&mut self);
}

pub trait StorageIterMut: Storage {
	type IterMut<'a>: Iterator<Item = (usize, &'a mut Self::Value)>
	where
		Self: 'a;

	fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

pub trait StorageAllocate: Storage {
	/// Allocates an index for the given value.
	fn allocate(&mut self, value: Self::Value) -> usize;
}

pub trait StorageAllocateConst: Storage {
	/// Allocates an index for the given value using interior mutability only.
	fn allocate_const(&self, value: Self::Value) -> usize;
}

pub trait StorageSet: Storage {
	/// Sets the already allocated index to the given value.
	///
	/// Returns the previous value, or `Err(value)` if the
	/// given index is not allocated.
	fn set(&mut self, index: usize, value: Self::Value) -> Result<Self::Value, Self::Value>;
}

pub trait StorageInsert: Storage {
	/// Inserts the given value at the given index.
	///
	/// Returns the previous value, or `None`
	/// if the given was not allocated.
	/// Contrarily to `StorageSet`, this function does allocate
	/// the given index if necessary, and stores the value there.
	fn insert(&mut self, index: usize, value: Self::Value) -> Option<Self::Value>;
}

pub trait StorageInsertConst: Storage {
	/// Inserts the given value at the given index using interior mutability only.
	///
	/// Returns the previous value, or `None`
	/// if the given was not allocated.
	/// Contrarily to `StorageSet`, this function does allocate
	/// the given index if necessary, and stores the value there.
	fn insert_const(&self, index: usize, value: Self::Value) -> Option<Self::Value>;
}

pub trait StorageRemove: Storage {
	fn remove(&mut self, index: usize) -> Option<Self::Value>;
}
