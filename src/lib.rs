#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

mod api_helper;
mod client;

pub mod errors;
pub mod types;
pub use client::*;
