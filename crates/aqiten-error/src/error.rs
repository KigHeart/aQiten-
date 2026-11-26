//! The workspace error enum.

use core::fmt;

/// Result alias used across the aQiten workspace.
pub type Result<T> = core::result::Result<T, Error>;

/// Every way an aQiten operation can fail on bad input or bad parameters.
///
/// Variants are grouped by cause. The `what` strings are static labels such as
/// `"field element"` chosen by the caller; they never contain secret values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Error {
    /// An integer computation left the range of its type.
    Overflow {
        /// The quantity being computed.
        what: &'static str,
    },
    /// A caller-supplied parameter is outside its documented domain.
    InvalidParameter {
        /// The parameter that was rejected.
        what: &'static str,
    },
    /// An encoded value was not in canonical form (for example a field element
    /// at or above the modulus, a bool other than 0 or 1).
    NonCanonical {
        /// The kind of value that was malformed.
        what: &'static str,
    },
    /// The input ended before the value was complete.
    Truncated {
        /// Bytes still needed.
        needed: usize,
        /// Bytes that were available.
        available: usize,
    },
    /// Bytes remained after a value that must consume its whole input.
    TrailingBytes {
        /// Number of unread bytes.
        remaining: usize,
    },
    /// A declared length exceeds the caller's bound.
    LengthTooLarge {
        /// The declared length.
        declared: u64,
        /// The maximum the caller allows.
        max: u64,
    },
    /// A value was outside the range required by the operation.
    OutOfRange {
        /// The value that was out of range.
        what: &'static str,
    },
    /// Inversion or division by zero (or by a non-unit).
    NotInvertible,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow { what } => write!(f, "integer overflow computing {what}"),
            Self::InvalidParameter { what } => write!(f, "invalid parameter: {what}"),
            Self::NonCanonical { what } => write!(f, "non-canonical encoding of {what}"),
            Self::Truncated { needed, available } => {
                write!(
                    f,
                    "input truncated: needed {needed} bytes, {available} available"
                )
            }
            Self::TrailingBytes { remaining } => {
                write!(f, "{remaining} trailing bytes after a complete value")
            }
            Self::LengthTooLarge { declared, max } => {
                write!(f, "declared length {declared} exceeds maximum {max}")
            }
            Self::OutOfRange { what } => write!(f, "{what} out of range"),
            Self::NotInvertible => write!(f, "element is not invertible"),
        }
    }
}

impl core::error::Error for Error {}
