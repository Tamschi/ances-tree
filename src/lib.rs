//! An example crate for a blog post.
//!
//! [![Zulip Chat](https://img.shields.io/endpoint?label=chat&url=https%3A%2F%2Fiteration-square-automation.schichler.dev%2F.netlify%2Ffunctions%2Fstream_subscribers_shield%3Fstream%3Dproject%252FTODO_CRATE_NAME)](https://iteration-square.schichler.dev/#narrow/stream/project.2FTODO_CRATE_NAME)

#![doc(html_root_url = "https://docs.rs/ances-tree/0.0.1")]
#![warn(clippy::pedantic)]
#![allow(clippy::semicolon_if_nothing_returned)]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}
