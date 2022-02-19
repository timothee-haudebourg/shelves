//! This is a small utility library for storing values of and
//! reference them using a unique typed index, `Ref<T>`,
//! which is a simple typed wrapper around `usize`.
//! Any data structure can be used behind the shelf as long as it provides
//! a way to store and fetch values by `usize`.
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
use derivative::Derivative;

pub mod storage;
mod shelf;
mod map;

pub use storage::*;
pub use shelf::Shelf;
pub use map::Map;

/// Typed reference to a stored value.
#[derive(Derivative)]
#[derivative(Clone(bound=""), Copy(bound=""), PartialEq(bound=""), Eq(bound=""), Hash(bound=""), PartialOrd(bound=""), Ord(bound=""), Debug(bound=""))]
pub struct Ref<T>(usize, std::marker::PhantomData<T>);

impl<T> Ref<T> {
	/// Creates a new reference from an index.
	pub fn new(index: usize) -> Self {
		Self(index, std::marker::PhantomData)
	}

	/// Returns the underlying index referencing the value.
	pub fn index(&self) -> usize {
		self.0
	}

	/// Changes the reference type.
	pub fn cast<U>(self) -> Ref<U> {
		Ref::new(self.0)
	}
}