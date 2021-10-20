//! An example crate for a blog post.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252FTODO_CRATE_NAME)](https://iteration-square.schichler.dev/#narrow/stream/project.2FTODO_CRATE_NAME)

#![doc(html_root_url = "https://docs.rs/ances-tree/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

use std::{borrow::Borrow, marker::PhantomPinned, pin::Pin};
use tap::Pipe;
use tiptoe::{Arc, IntrusivelyCountable, TipToe};

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
	_pin: PhantomPinned,
}

pub type NodeHandle<T> = Pin<Arc<Node<T>>>;

impl<T> Node<T> {
	/// Creates a new [`Node`] instance with the given `parent` and `value`.
	pub fn new(parent: Option<NodeHandle<T>>, value: T) -> NodeHandle<T> {
		Node {
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

	#[must_use]
	pub fn clone_handle(&self) -> NodeHandle<T> {
		Pin::clone(unsafe { Arc::borrow_pin_from_inner_ref(&self) })
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
	type RefCounter = TipToe;

	fn ref_counter(&self) -> &Self::RefCounter {
		todo!()
	}
}
