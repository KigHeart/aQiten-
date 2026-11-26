//! Prime fields for aQiten.
//!
//! This crate provides [`Fp<P>`], arithmetic modulo a prime `P < 2^63` given as a
//! const generic, and the [`Field`] trait that downstream crates (ring,
//! transcript, sum-check) are written against.
//!
//! # Design decisions
//!
//! * **The modulus is checked at compile time.** Using `Fp<P>` with a `P` that is
//!   not an odd prime in `3..2^63` is a compile error (a deterministic
//!   Miller–Rabin test, valid for all 64-bit integers, runs in a `const`
//!   context). A wrong modulus can therefore never reach a running program.
//! * **Montgomery form with `R = 2^64`.** Values are stored fully reduced in
//!   `[0, P)`, so the stored word is a unique representative and byte-wise
//!   equality is element equality.
//! * **No secret-dependent branches in the source.** Addition, subtraction,
//!   negation, multiplication, `pow` and the signed conversions use
//!   mask-and-select instead of `if`. Reaching *hardware* constant time also
//!   depends on the compiler and CPU; that is not formally verified here
//!   (tag: CONJECTURE, to be tested with a timing harness in a later work item).
//!   Operations that fail on public conditions (`from_canonical` on an
//!   out-of-range word, `inv` of zero) do branch; those branch on the *validity
//!   of an input*, which the protocol treats as public.
//! * **Extension fields are deliberately absent.** Their degree and the base
//!   field are fixed by the parameter work (P1); adding them now would freeze a
//!   choice that has not been made.
//!
//! # Cross-crate role
//!
//! `aqiten-ring` builds polynomial rings over a [`Field`]; `aqiten-transcript`
//! absorbs and samples field elements through [`aqiten_codec`]; `aqiten-sumcheck`
//! evaluates univariate round polynomials over a [`Field`].

#![no_std]

extern crate alloc;

mod field;
mod fp;
mod prime;

pub use field::Field;
pub use fp::Fp;
pub use prime::is_prime;
