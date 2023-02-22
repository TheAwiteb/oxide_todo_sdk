#![doc = include_str!("../README.md")]
#![forbid(missing_docs)]
#![forbid(unsafe_code)]

mod api_helper;
mod client;
mod types;

pub mod errors;
pub use client::*;
