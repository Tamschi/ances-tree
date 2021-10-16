//! An example crate for a blog post.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252FTODO_CRATE_NAME)](https://iteration-square.schichler.dev/#narrow/stream/project.2FTODO_CRATE_NAME)

#![doc(html_root_url = "https://docs.rs/ances-tree/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::{borrow::Borrow, sync::Arc};
use tap::Pipe;

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

/// A reference-counting inverse tree node.
#[derive(Debug, Clone)]
pub struct Node<T> {
	pub parent: Option<Arc<Self>>,
	pub value: T,
}

impl<T> Node<T> {
	/// Creates a new [`Node`] instance with the given `parent` and `value`.
	pub fn new(parent: Option<Arc<Self>>, value: T) -> Self {
		Self { parent, value }
	}

	/// Retrieves a reference to a value matching `key` iff available.
	///
	/// See also: <https://doc.rust-lang.org/stable/std/collections/hash_set/struct.HashSet.html#method.get>
	#[must_use]
	pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&T>
	where
		T: Borrow<Q>,
		Q: Eq,
	{
		let mut this = self;
		while this.value.borrow() != key {
			this = this.parent.as_ref()?
		}
		Some(&this.value)
	}

	/// Retrieves a mutable reference to a value matching `key` iff available.
	///
	/// See also: <https://doc.rust-lang.org/stable/std/collections/hash_set/struct.HashSet.html#method.get>
	///
	/// # Errors
	///
	/// Iff an ancestor is shared so that it can't be borrowed mutably.
	#[allow(clippy::result_unit_err)] // In a real crate, I'd return a `Result<Option<&mut T>, &mut Arc<Self>>` instead.
	#[must_use]
	pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Result<Option<&mut T>, ()>
	where
		T: Borrow<Q>,
		Q: Eq,
	{
		let mut this = self;
		while this.value.borrow() != key {
			match this.parent.as_mut() {
				None => return Ok(None),
				Some(parent) => this = Arc::get_mut(parent).ok_or(())?,
			}
		}
		Ok(Some(&mut this.value))
	}

	/// Retrieves a mutable to a value matching `key` iff available, by cloning ancestors as necessary.
	///
	/// See also: <https://doc.rust-lang.org/stable/std/collections/hash_set/struct.HashSet.html#method.get>
	///
	/// # Errors
	///
	/// Iff an ancestor is shared so that it can't be borrowed mutably.
	#[allow(clippy::result_unit_err)] // In a real crate, I'd return a `Result<Option<&mut T>, &mut Arc<Self>>` instead.
	pub fn make_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut T>
	where
		T: Borrow<Q> + Clone,
		Q: Eq,
	{
		// Slow, but simple for demo purposes:
		self.get(key)?;
		// We now know that the target value exists somewhere.

		let mut this = self;
		while this.value.borrow() != key {
			this = this
				.parent
				.as_mut()
				.expect("Misbehaving equality." /* We know the node exists. */)
				.pipe(Arc::make_mut)
		}
		Some(&mut this.value)
	}
}
