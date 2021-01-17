#![allow(clippy::suspicious_arithmetic_impl)]
// Some structs do not have AddAssign or MulAssign impl.
#![allow(clippy::suspicious_op_assign_impl)]
// Variables have always the same names in respect to wires.
#![allow(clippy::many_single_char_names)]
// Bool expr are usually easier to read with match statements.
#![allow(clippy::match_bool)]
// Clippy does not have `broken_intra_doc_links` as a known lint.
#![allow(unknown_lints)]
#![deny(broken_intra_doc_links)]
#![deny(missing_debug_implementations)]
#![deny(unsafe_code)]

#[macro_use]
mod macros;

pub mod commitment_scheme;
pub mod fft;
pub mod transcript;
pub mod serialisation;
mod utils;
pub mod lagrange_interpolation;
mod demo;


/// Re-exported dusk-bls12_381 fork.
pub use dusk_bls12_381 as bls12_381;

/// Re-exported dusk-jubjub fork.
pub use dusk_jubjub as jubjub;

