# loglib-rs

This is a rewrite of loglib in Rust. It's not because of the rewrite-it-in-rust movement, but
rather due to the way loglib works: it's full of C++ preprocessor macros. Unfortunately it is not
possible (AFAIK) to use such macros through Rust's C/C++ FFI, hence rather I decided to create
a small crate, so I can use it also in Rust projects.
