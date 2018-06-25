//! I2C Commands for pH EZO Chip, taken from their Datasheet.
//! This chip is used for pH measurement. It features calibration, sleep mode,
//! scale, etc.

#![feature(str_checked_slicing)]

#![feature(exclusive_range_pattern)]

#![feature(inclusive_range_syntax)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate ezo_common;
extern crate i2cdev;

// Use error-chain.
pub mod errors;

/// Issuable commands for the PH EZO Chip.
pub mod command;

/// Parseable responses from the PH EZO Chip.
pub mod response;
