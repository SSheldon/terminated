This crate provides types for representing NUL-terminated UTF8 strings.

The `NulTerminatedStr` type is useful when interacting with C APIs that
require/guarantee UTF8 encoding. Rust has great support for dealing with UTF8,
but C strings require a NUL terminator which Rust's `str` and `String` don't have.

``` rust
let s = ntstr!("Hello, World!");

// You can use Rust's normal string operations
assert_eq!(s.find("World"), Some(7));

// And pass it to C since it's NUL-terminated
let ptr = s.as_ptr();
```

# CStr vs NulTerminatedStr

The standard library does provide the `CStr` type that is NUL-terminated,
but it does not use any specific encoding. It's therefore insufficient
if your input needs to be both NUL-terminated and UTF8 encoded.
