#![doc = include_str!("../README.md")]
// #![deny(unsafe_code)]
#![cfg_attr(not(any(feature = "std", test)), no_std)]

mod bytes;
pub mod metadata;
pub mod pe;
