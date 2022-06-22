#![no_std]

extern crate alloc;

mod data;
mod error;
mod interfaces;
mod staking;
pub type Time = u64;
pub use staking::Staking;
