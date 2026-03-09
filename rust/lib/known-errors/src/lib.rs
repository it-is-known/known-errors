// This is free and unencumbered software released into the public domain.

//! This crate provides well-known errors.
//!
//! ```edition2024
//! # use known_errors::*;
//! ```

#![no_std]
#![deny(unsafe_code)]

mod features;
pub use features::*;

#[cfg(feature = "sysexits")]
pub mod sysexits;

#[doc = include_str!("../../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
