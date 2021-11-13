//! An example crate for a blog post.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252FTODO_CRATE_NAME)](https://iteration-square.schichler.dev/#narrow/stream/project.2FTODO_CRATE_NAME)

#![doc(html_root_url = "https://docs.rs/ances-tree/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::{borrow::Borrow, pin::Pin};
use tap::Pipe;
use tiptoe::{Arc, IntrusivelyCountable, ManagedClone, TipToe};

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

/// A reference-counting inverse tree node.
#[pin_project::pin_project]
#[derive(Debug)]
pub struct Node<T> {
	pub parent: Option<NodeHandle<T>>,
	pub value: T,
	#[pin] // Required to keep `Node<T>: !Unpin`!
	tip_toe: TipToe,
}

pub type NodeHandle<T> = Pin<Arc<Node<T>>>;

impl<T> Node<T> {
	/// Creates a new [`Node`] instance with the given `parent` and `value`.
	pub fn new(parent: Option<NodeHandle<T>>, value: T) -> NodeHandle<T> {
		Node {
			parent,
			value,
			tip_toe: TipToe::new(),
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

	#[must_use]
	pub fn clone_handle(&self) -> NodeHandle<T> {
		Pin::clone(unsafe {
			// SAFETY: It's impossible for a consumer to acquire a reference to a `Node` outside an `Arc`.
			Arc::borrow_pin_from_inner_ref(&self)
		})
	}

	#[must_use]
	pub fn parent_mut<'a>(self: &'a mut Pin<&mut Self>) -> &'a mut Option<NodeHandle<T>> {
		self.as_mut().project().parent
	}

	#[must_use]
	pub fn value_mut<'a>(self: &'a mut Pin<&mut Self>) -> &'a mut T {
		self.as_mut().project().value
	}
}

unsafe impl<T> IntrusivelyCountable for Node<T> {
	// SAFETY:
	// The returned reference-counter is a not otherwise decremented private per-instance counter and `Node` is `!Unpin`,
	// which fulfills the semantic requirements for such a reference-counter even if `Pin<&mut Node<T>>` can be acquired.

	type RefCounter = TipToe;

	#[allow(clippy::inline_always)]
	#[inline(always)]
	fn ref_counter(&self) -> &Self::RefCounter {
		&self.tip_toe
	}
}

impl<T> ManagedClone for Node<T>
where
	T: Clone,
{
	unsafe fn managed_clone(&self) -> Self {
		// SAFETY:
		// These are fully safe operations.
		// Cloning `TipToe` returns a new instance with internal count zero, which is correct here.
		Self {
			parent: Option::clone(&self.parent),
			value: self.value.clone(),
			tip_toe: self.tip_toe.clone(),
		}
	}
}
