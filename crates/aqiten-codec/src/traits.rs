//! The `Encode` / `Decode` traits and strict whole-input helpers.

use alloc::vec::Vec;

use aqiten_error::Result;

use crate::{Reader, Writer};

/// A type with exactly one canonical byte encoding.
pub trait Encode {
    /// Appends the canonical encoding of `self` to `w`.
    ///
    /// # Errors
    /// Fails only when a variable-length component does not fit its length
    /// prefix; fixed-width types never fail.
    fn encode(&self, w: &mut Writer) -> Result<()>;
}

/// A type that can be decoded from its canonical encoding and nothing else.
pub trait Decode: Sized {
    /// Reads one value from `r`, rejecting every non-canonical encoding.
    ///
    /// # Errors
    /// Any [`aqiten_error::Error`] describing why the bytes are not the
    /// canonical encoding of a value of this type.
    fn decode(r: &mut Reader<'_>) -> Result<Self>;
}

/// Encodes `value` into a fresh vector.
///
/// # Errors
/// As [`Encode::encode`].
pub fn encode_to_vec<T: Encode + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut w = Writer::new();
    value.encode(&mut w)?;
    Ok(w.into_bytes())
}

/// Decodes a `T` that must occupy **all** of `bytes`.
///
/// # Errors
/// Any decoding error from `T`, or [`aqiten_error::Error::TrailingBytes`].
pub fn decode_exact<T: Decode>(bytes: &[u8]) -> Result<T> {
    let mut r = Reader::new(bytes);
    let value = T::decode(&mut r)?;
    r.finish()?;
    Ok(value)
}

/// Encodes a slice as `u32` length prefix followed by each element.
///
/// # Errors
/// [`aqiten_error::Error::LengthTooLarge`] when the slice is longer than
/// `u32::MAX`, or any element's encoding error.
pub fn encode_slice<T: Encode>(items: &[T], w: &mut Writer) -> Result<()> {
    w.put_len(items.len())?;
    for item in items {
        item.encode(w)?;
    }
    Ok(())
}

/// Decodes a length-prefixed sequence of at most `max_len` elements.
///
/// The declared length is checked against `max_len` before allocating, and the
/// initial capacity is further capped by the number of unread bytes (every
/// element must occupy at least one byte), so a lying prefix cannot force a
/// large allocation.
///
/// # Errors
/// [`aqiten_error::Error::LengthTooLarge`] if the prefix exceeds `max_len`, or
/// any element decoding error (including truncation).
pub fn decode_vec<T: Decode>(r: &mut Reader<'_>, max_len: usize) -> Result<Vec<T>> {
    let len = r.read_len(max_len)?;
    let mut out = Vec::with_capacity(len.min(r.remaining()));
    for _ in 0..len {
        out.push(T::decode(r)?);
    }
    Ok(out)
}

macro_rules! impl_uint {
    ($t:ty, $put:ident, $read:ident) => {
        impl Encode for $t {
            fn encode(&self, w: &mut Writer) -> Result<()> {
                w.$put(*self);
                Ok(())
            }
        }
        impl Decode for $t {
            fn decode(r: &mut Reader<'_>) -> Result<Self> {
                r.$read()
            }
        }
    };
}

impl_uint!(u8, put_u8, read_u8);
impl_uint!(u16, put_u16, read_u16);
impl_uint!(u32, put_u32, read_u32);
impl_uint!(u64, put_u64, read_u64);

impl Encode for bool {
    fn encode(&self, w: &mut Writer) -> Result<()> {
        w.put_bool(*self);
        Ok(())
    }
}
impl Decode for bool {
    fn decode(r: &mut Reader<'_>) -> Result<Self> {
        r.read_bool()
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, w: &mut Writer) -> Result<()> {
        w.put_raw(self);
        Ok(())
    }
}
impl<const N: usize> Decode for [u8; N] {
    fn decode(r: &mut Reader<'_>) -> Result<Self> {
        r.read_array::<N>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aqiten_error::Error;

    #[test]
    fn integers_are_little_endian_known_answers() {
        assert_eq!(encode_to_vec(&0x0102_u16), Ok(alloc::vec![0x02, 0x01]));
        assert_eq!(encode_to_vec(&0x0102_0304_u32), Ok(alloc::vec![4, 3, 2, 1]));
        assert_eq!(
            encode_to_vec(&0x0102_0304_0506_0708_u64),
            Ok(alloc::vec![8, 7, 6, 5, 4, 3, 2, 1])
        );
        assert_eq!(encode_to_vec(&true), Ok(alloc::vec![1]));
        assert_eq!(encode_to_vec(&[9u8, 8, 7]), Ok(alloc::vec![9, 8, 7]));
    }

    #[test]
    fn round_trips() {
        for x in [0u64, 1, 255, 256, u64::MAX, 0xDEAD_BEEF_CAFE_F00D] {
            let b = encode_to_vec(&x).unwrap_or_default();
            assert_eq!(decode_exact::<u64>(&b), Ok(x));
        }
        for x in [0u32, 1, u32::MAX] {
            let b = encode_to_vec(&x).unwrap_or_default();
            assert_eq!(decode_exact::<u32>(&b), Ok(x));
        }
        let arr = [1u8, 2, 3, 4, 5];
        let b = encode_to_vec(&arr).unwrap_or_default();
        assert_eq!(decode_exact::<[u8; 5]>(&b), Ok(arr));
    }

    #[test]
    fn bool_rejects_non_canonical() {
        assert_eq!(decode_exact::<bool>(&[0]), Ok(false));
        assert_eq!(decode_exact::<bool>(&[1]), Ok(true));
        for b in 2u8..=255 {
            assert_eq!(
                decode_exact::<bool>(&[b]),
                Err(Error::NonCanonical { what: "bool" })
            );
        }
    }

    #[test]
    fn truncation_and_trailing_bytes_are_rejected() {
        assert_eq!(
            decode_exact::<u32>(&[1, 2, 3]),
            Err(Error::Truncated {
                needed: 4,
                available: 3
            })
        );
        assert_eq!(
            decode_exact::<u32>(&[1, 2, 3, 4, 5]),
            Err(Error::TrailingBytes { remaining: 1 })
        );
        assert_eq!(
            decode_exact::<u8>(&[]),
            Err(Error::Truncated {
                needed: 1,
                available: 0
            })
        );
    }

    #[test]
    fn a_failed_take_does_not_consume() {
        let mut r = Reader::new(&[1, 2, 3]);
        assert!(r.take(4).is_err());
        assert_eq!(r.remaining(), 3);
        assert_eq!(r.take(3), Ok(&[1u8, 2, 3][..]));
        assert_eq!(r.finish(), Ok(()));
    }

    #[test]
    fn vec_round_trip_and_length_bound() {
        let items: [u16; 3] = [1, 2, 0xFFFF];
        let mut w = Writer::new();
        assert_eq!(encode_slice(&items, &mut w), Ok(()));
        let bytes = w.into_bytes();
        assert_eq!(bytes, alloc::vec![3, 0, 0, 0, 1, 0, 2, 0, 0xFF, 0xFF]);

        let mut r = Reader::new(&bytes);
        assert_eq!(decode_vec::<u16>(&mut r, 3), Ok(alloc::vec![1, 2, 0xFFFF]));
        assert_eq!(r.finish(), Ok(()));

        let mut r = Reader::new(&bytes);
        assert_eq!(
            decode_vec::<u16>(&mut r, 2),
            Err(Error::LengthTooLarge {
                declared: 3,
                max: 2
            })
        );
    }

    #[test]
    fn a_lying_length_prefix_cannot_force_allocation() {
        // Declares u32::MAX elements but supplies none; must fail fast on the bound
        // or on truncation, never attempt a huge allocation.
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF];
        let mut r = Reader::new(&bytes);
        assert_eq!(
            decode_vec::<u8>(&mut r, 1 << 20),
            Err(Error::LengthTooLarge {
                declared: u64::from(u32::MAX),
                max: 1 << 20
            })
        );
        // A prefix within the bound but longer than the data: truncation, small capacity.
        let bytes = [0x00, 0x00, 0x10, 0x00]; // little-endian 0x0010_0000 = 1_048_576 elements declared
        let mut r = Reader::new(&bytes);
        assert!(matches!(
            decode_vec::<u8>(&mut r, usize::MAX),
            Err(Error::Truncated { .. })
        ));
    }

    #[test]
    fn decode_then_encode_is_identity_on_accepted_inputs() {
        // Canonicality: for every accepted byte string, re-encoding reproduces it.
        // Exhaustive over all 2-byte inputs for the `u16`, and over all 1-byte inputs for `bool`.
        for hi in 0u8..=255 {
            for lo in 0u8..=255 {
                let bytes = [lo, hi];
                if let Ok(v) = decode_exact::<u16>(&bytes) {
                    assert_eq!(encode_to_vec(&v), Ok(alloc::vec![lo, hi]));
                }
            }
        }
        for b in 0u8..=255 {
            if let Ok(v) = decode_exact::<bool>(&[b]) {
                assert_eq!(encode_to_vec(&v), Ok(alloc::vec![b]));
            }
        }
    }

    #[test]
    fn decoders_never_panic_on_pseudorandom_prefixes() {
        // Deterministic xorshift stream; try every prefix of several buffers against
        // each decoder. The assertion is simply that nothing panics.
        let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        for _ in 0..200 {
            let buf: Vec<u8> = (0..40).map(|_| next().to_le_bytes()[0]).collect();
            for n in 0..=buf.len() {
                let prefix = buf.get(..n).unwrap_or(&[]);
                let _ = decode_exact::<u64>(prefix);
                let _ = decode_exact::<bool>(prefix);
                let _ = decode_exact::<[u8; 16]>(prefix);
                let mut r = Reader::new(prefix);
                let _ = decode_vec::<u32>(&mut r, 8);
            }
        }
    }
}
