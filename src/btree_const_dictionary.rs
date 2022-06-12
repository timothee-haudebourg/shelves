use crate::{Ref, Shelf, Storage, StorageAllocateConst, StorageMut, StorageRemove};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::BTreeMap;

/// Bidirectional mapping between `T` and `Ref<T>`.
pub struct BTreeConstDictionary<S: Storage> {
	map: RefCell<BTreeMap<S::Value, Ref<S::Value>>>,
	values: Shelf<S>,
}

impl<S: Storage + Default> Default for BTreeConstDictionary<S> {
	fn default() -> Self {
		Self {
			map: RefCell::new(BTreeMap::new()),
			values: Shelf::default(),
		}
	}
}

impl<S: Storage> BTreeConstDictionary<S> {
	/// Creates a new empty dictionary.
	pub fn new(storage: S) -> Self {
		Self {
			map: RefCell::new(BTreeMap::new()),
			values: Shelf::new(storage),
		}
	}

	pub fn len(&self) -> usize {
		self.map.borrow().len()
	}

	pub fn is_empty(&self) -> bool {
		self.map.borrow().is_empty()
	}

	#[allow(clippy::type_complexity)]
	pub fn into_parts(self) -> (RefCell<BTreeMap<S::Value, Ref<S::Value>>>, Shelf<S>) {
		(self.map, self.values)
	}

	pub fn into_map(self) -> BTreeMap<S::Value, Ref<S::Value>> {
		self.map.into_inner()
	}

	pub fn as_map(&self) -> &RefCell<BTreeMap<S::Value, Ref<S::Value>>> {
		&self.map
	}

	pub fn into_shelf(self) -> Shelf<S> {
		self.values
	}

	pub fn as_shelf(&self) -> &Shelf<S> {
		&self.values
	}

	pub fn into_storage(self) -> S {
		self.values.into_storage()
	}

	pub fn as_storage(&self) -> &S {
		self.values.as_storage()
	}

	/// Borrows the definition (`T`) associated to the given term (`Ref<T>`).
	pub fn borrow<T>(&self, r: Ref<T>) -> Option<&T>
	where
		S::Value: Borrow<T>,
	{
		self.values.borrow(r)
	}

	/// Returns a reference to the definition (`S::Value`) associated to the given term (`Ref<T>`).
	pub fn get<T>(&self, r: Ref<T>) -> Option<&S::Value>
	where
		S::Value: Borrow<T>,
	{
		self.values.get(r)
	}
}

impl<S: StorageMut> BTreeConstDictionary<S> {
	/// Borrows a mutable reference to the definition (`T`) associated to the given term (`Ref<T>`).
	pub fn borrow_mut<T>(&mut self, r: Ref<T>) -> Option<&mut T>
	where
		S::Value: BorrowMut<T>,
	{
		self.values.borrow_mut(r)
	}

	/// Get a mutable reference to the definition (`S::Value`) associated to the given term (`Ref<T>`).
	pub fn get_mut<T>(&mut self, r: Ref<T>) -> Option<&mut S::Value>
	where
		S::Value: BorrowMut<T>,
	{
		self.values.get_mut(r)
	}
}

impl<S: StorageAllocateConst> BTreeConstDictionary<S> {
	/// Inserts a new value and returns its unique reference.
	///
	/// If the value is already registered, its old reference is returned
	/// without overriding the old value.
	pub fn insert(&self, value: S::Value) -> Ref<S::Value>
	where
		S::Value: Clone + Ord,
	{
		use std::collections::btree_map::Entry;
		let mut map = self.map.borrow_mut();
		match map.entry(value) {
			Entry::Vacant(entry) => {
				let r = self.values.insert_const(entry.key().clone());
				entry.insert(r);
				r
			}
			Entry::Occupied(entry) => *entry.get(),
		}
	}
}

impl<S: StorageRemove> BTreeConstDictionary<S> {
	pub fn remove<T>(&mut self, r: Ref<T>) -> Option<S::Value>
	where
		S::Value: Borrow<T> + Ord,
	{
		match self.values.remove(r) {
			Some(value) => {
				let mut map = self.map.borrow_mut();
				map.remove::<S::Value>(&value);
				Some(value)
			}
			None => None,
		}
	}

	pub fn remove_value<T>(&mut self, value: &S::Value) -> Option<(Ref<S::Value>, S::Value)>
	where
		S::Value: Ord,
	{
		let mut map = self.map.borrow_mut();
		map.remove(value)
			.map(|r| (r, self.values.remove(r).unwrap()))
	}
}
