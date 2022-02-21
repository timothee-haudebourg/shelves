use crate::{Ref, Storage, StorageInsert, StorageMut};
use std::marker::PhantomData;

pub struct Map<K, S> {
	storage: S,
	key: PhantomData<K>,
}

impl<K, S: Default> Default for Map<K, S> {
	fn default() -> Self {
		Self {
			storage: S::default(),
			key: PhantomData
		}
	}
}

impl<K, S> Map<K, S> {
	pub fn new(storage: S) -> Self {
		Self {
			storage,
			key: PhantomData
		}
	}

	pub fn into_storage(self) -> S {
		self.storage
	}

	pub fn as_storage(&self) -> &S {
		&self.storage
	}
}

impl<K, S: Storage> Map<K, S> {
	pub fn get(&self, r: Ref<K>) -> Option<&S::Value> {
		self.storage.get(r.index())
	}
}

impl<K, S: StorageMut> Map<K, S> {
	pub fn get_mut(&mut self, r: Ref<K>) -> Option<&mut S::Value> {
		self.storage.get_mut(r.index())
	}
}

impl<K, S: StorageInsert> Map<K, S> {
	pub fn insert(&mut self, r: Ref<K>, value: S::Value) -> Option<S::Value> {
		self.storage.insert(r.index(), value)
	}
}
