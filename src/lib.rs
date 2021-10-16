//! An example crate for a blog post.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252FTODO_CRATE_NAME)](https://iteration-square.schichler.dev/#narrow/stream/project.2FTODO_CRATE_NAME)

#![doc(html_root_url = "https://docs.rs/ances-tree/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::{borrow::Borrow, marker::PhantomPinned, pin::Pin, sync::Arc};
use tap::Pipe;

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

/// A reference-counting inverse tree node.
#[derive(Debug)]
pub struct Node<T> {
	pub parent: Option<Pin<Arc<Self>>>,
	pub value: T,
	_pin: PhantomPinned,
}

impl<T> Node<T> {
	/// Creates a new [`Node`] instance with the given `parent` and `value`.
	pub fn new(parent: Option<Pin<Arc<Self>>>, value: T) -> Pin<Arc<Self>> {
		Self {
			parent,
			value,
			_pin: PhantomPinned,
		}
		.pipe(Arc::pin)
	}

	/// Retrieves a reference to a [`Node`] with a value matching `key` iff available.
	///
	/// See also: <https://doc.rust-lang.org/stable/std/collections/hash_set/struct.HashSet.html#method.get>
	#[must_use]
	pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&Self>
	where
		T: Borrow<Q>,
		Q: Eq,
	{
		let mut this = self;
		while this.value.borrow() != key {
			this = this.parent.as_ref()?
		}
		Some(this)
	}

	// The standard library doesn't provide mutability helpers on `Arc` that work (safely) with a pinned value.
	// Mutability will be back, eventually.
}
