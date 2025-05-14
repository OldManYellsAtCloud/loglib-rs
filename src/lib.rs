//! Helper library for the log library
//!
//! This is intended to be the same as the C++ version of loglib,
//! however since C/C++ preprocessor macros are not supported in
//! Rust through FFI, it is rewritten in Rust itself.

pub mod enums;
pub mod structs;
pub mod logger;
pub mod send_log_trait;
