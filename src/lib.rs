//! Helper library for the log library
//!
//! This is intended to be the same as the C++ version of loglib,
//! however since C/C++ preprocessor macros are not supported in
//! Rust through FFI, it is rewritten in Rust itself.

mod enums;
mod structs;
mod logger;
mod send_log_trait;
