//! Bounds-checked cursor over untrusted bytes.

use aqiten_error::{Error, Result};

/// A forward-only cursor over an immutable byte slice.
///
/// Every read returns an error when too few bytes remain; nothing is ever
/// indexed directly, so no read can panic.
#[derive(Clone, Debug)]
pub struct Reader<'a> {
    rest: &'a [u8],
}

impl<'a> Reader<'a> {
    /// Starts reading at the beginning of `bytes`.
    #[must_use]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { rest: bytes }
    }

    /// Number of unread bytes.
    #[must_use]
    pub fn remaining(&self) -> usize {
        self.rest.len()
    }

    /// Consumes and returns exactly `n` bytes.
    ///
    /// # Errors
    /// [`Error::Truncated`] when fewer than `n` bytes remain; the reader is left
    /// unchanged in that case.
    pub fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        match self.rest.split_at_checked(n) {
            Some((head, tail)) => {
                self.rest = tail;
                Ok(head)
            }
            None => Err(Error::Truncated {
                needed: n,
                available: self.rest.len(),
            }),
        }
    }

    /// Consumes exactly `N` bytes as a fixed-size array.
    ///
    /// # Errors
    /// [`Error::Truncated`] when fewer than `N` bytes remain.
    pub fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let head = self.take(N)?;
        // `take` returned exactly N bytes, so this conversion cannot fail; the
        // error arm exists only so that no panic path is compiled.
        <[u8; N]>::try_from(head).map_err(|_| Error::Truncated {
            needed: N,
            available: head.len(),
        })
    }

    /// Reads one byte.
    ///
    /// # Errors
    /// [`Error::Truncated`] at end of input.
    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(u8::from_le_bytes(self.read_array::<1>()?))
    }

    /// Reads a little-endian `u16`.
    ///
    /// # Errors
    /// [`Error::Truncated`] when fewer than 2 bytes remain.
    pub fn read_u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.read_array::<2>()?))
    }

    /// Reads a little-endian `u32`.
    ///
    /// # Errors
    /// [`Error::Truncated`] when fewer than 4 bytes remain.
    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_array::<4>()?))
    }

    /// Reads a little-endian `u64`.
    ///
    /// # Errors
    /// [`Error::Truncated`] when fewer than 8 bytes remain.
    pub fn read_u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.read_array::<8>()?))
    }

    /// Reads a `bool`, accepting only the bytes `0` and `1`.
    ///
    /// # Errors
    /// [`Error::NonCanonical`] for any other byte value.
    pub fn read_bool(&mut self) -> Result<bool> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::NonCanonical { what: "bool" }),
        }
    }

    /// Reads a `u32` length prefix and checks it against `max` **before** the
    /// caller allocates anything.
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] when the declared length exceeds `max`, and
    /// [`Error::Overflow`] if it does not fit `usize` on this target.
    pub fn read_len(&mut self, max: usize) -> Result<usize> {
        let declared = self.read_u32()?;
        let max_u64 = aqiten_error::arith::u64_from_usize(max, "length bound")?;
        if u64::from(declared) > max_u64 {
            return Err(Error::LengthTooLarge {
                declared: u64::from(declared),
                max: max_u64,
            });
        }
        aqiten_error::arith::usize_from_u64(u64::from(declared), "declared length")
    }

    /// Asserts that the whole input has been consumed.
    ///
    /// # Errors
    /// [`Error::TrailingBytes`] when unread bytes remain.
    pub fn finish(self) -> Result<()> {
        if self.rest.is_empty() {
            Ok(())
        } else {
            Err(Error::TrailingBytes {
                remaining: self.rest.len(),
            })
        }
    }
}
