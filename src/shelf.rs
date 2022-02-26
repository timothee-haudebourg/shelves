use crate::{Ref, Storage, StorageAllocate, StorageMut, StorageRemove, StorageSet};
use std::borrow::{Borrow, BorrowMut};

pub struct Shelf<S> {
	storage: S,
}

impl<S> Shelf<S> {
	pub fn new(storage: S) -> Self {
		Self { storage }
	}

	pub fn into_storage(self) -> S {
		self.storage
	}

	pub fn as_storage(&self) -> &S {
		&self.storage
	}
}

impl<S: Default> Default for Shelf<S> {
	fn default() -> Self {
		Self {
			storage: S::default(),
		}
	}
}

impl<S: Storage> Shelf<S> {
	pub fn len(&self) -> usize {
		self.storage.len()
	}

	pub fn is_empty(&self) -> bool {
		self.storage.is_empty()
	}

	pub fn borrow<T>(&self, r: Ref<T>) -> Option<&T>
	where
		S::Value: Borrow<T>,
	{
		self.storage.get(r.index()).map(|v| v.borrow())
	}

	pub fn get<T>(&self, r: Ref<T>) -> Option<&S::Value>
	where
		S::Value: Borrow<T>,
	{
		self.storage.get(r.index())
	}
}

impl<S: StorageMut> Shelf<S> {
	pub fn borrow_mut<T>(&mut self, r: Ref<T>) -> Option<&mut T>
	where
		S::Value: BorrowMut<T>,
	{
		self.storage.get_mut(r.index()).map(|v| v.borrow_mut())
	}

	pub fn get_mut<T>(&mut self, r: Ref<T>) -> Option<&mut S::Value>
	where
		S::Value: BorrowMut<T>,
	{
		self.storage.get_mut(r.index())
	}
}

impl<S: StorageAllocate> Shelf<S> {
	pub fn insert(&mut self, value: S::Value) -> Ref<S::Value> {
		Ref::new(self.storage.allocate(value))
	}
}

impl<S: StorageSet> Shelf<S> {
	pub fn set<T>(&mut self, r: Ref<T>, value: S::Value) -> Result<S::Value, S::Value>
	where
		S::Value: Borrow<T>,
	{
		self.storage.set(r.index(), value)
	}
}

impl<S: StorageRemove> Shelf<S> {
	pub fn remove<T>(&mut self, r: Ref<T>) -> Option<S::Value>
	where
		S::Value: Borrow<T>,
	{
		self.storage.remove(r.index())
	}
}

impl<S: StorageSet + StorageRemove> Shelf<S> {
	pub fn set_or_remove<T>(
		&mut self,
		r: Ref<T>,
		value: Option<S::Value>,
	) -> Result<Option<S::Value>, S::Value>
	where
		S::Value: Borrow<T>,
	{
		match value {
			Some(value) => self.storage.set(r.index(), value).map(Some),
			None => Ok(self.storage.remove(r.index())),
		}
	}
}
