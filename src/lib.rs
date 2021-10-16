//! An example crate for a blog post.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252FTODO_CRATE_NAME)](https://iteration-square.schichler.dev/#narrow/stream/project.2FTODO_CRATE_NAME)

#![doc(html_root_url = "https://docs.rs/ances-tree/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::{borrow::Borrow, marker::PhantomPinned, pin::Pin};
use tap::Pipe;
use triomphe::{Arc, ArcBorrow};

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
		.pipe(Arc::new)
		.pipe(|arc| unsafe { Pin::new_unchecked(arc) })
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

#[must_use]
pub fn borrow_arc<T>(this: &Pin<Arc<Node<T>>>) -> Pin<ArcBorrow<'_, Node<T>>> {
	unsafe { &*(this as *const Pin<Arc<Node<T>>>).cast::<Arc<Node<T>>>() }
		.pipe(Arc::borrow_arc)
		.pipe(|arc_borrow| unsafe { Pin::new_unchecked(arc_borrow) })
}

#[must_use]
pub fn clone_arc<T>(this: &Pin<ArcBorrow<Node<T>>>) -> Pin<Arc<Node<T>>> {
	unsafe { &*(this as *const Pin<ArcBorrow<Node<T>>>).cast::<ArcBorrow<Node<T>>>() }
		.pipe(ArcBorrow::clone_arc)
		.pipe(|arc| unsafe { Pin::new_unchecked(arc) })
}
