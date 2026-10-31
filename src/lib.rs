//! # alloy-eips
//!
//! Ethereum Improvement Proposals — Prague-ready hardfork activation logic.
//!
//! Provides a `Hardfork` enum, block-number / timestamp activation checks,
//! and per-EIP feature flags.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod eip1559;
pub mod eip4844;
pub mod hardfork;
