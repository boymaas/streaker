#![feature(proc_macro_hygiene)]
#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]
// #![deny(warnings)]
#![allow(unused_imports)]

pub mod dbstate;
pub mod jwt;
pub mod model;
pub mod web;

// TODO: why does this not take on cargo test ..
// #[cfg(test)]
pub mod migrate;
//#[cfg(test)]
pub mod testdb;
//#[cfg(test)]
pub mod streaker_client;
