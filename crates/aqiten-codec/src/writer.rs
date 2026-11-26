//! Append-only byte sink with checked length prefixes.

use alloc::vec::Vec;

use aqiten_error::{Error, Result};

/// An append-only buffer that produces canonical encodings.
#[derive(Clone, Debug, Default)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    /// Creates an empty writer.
    #[must_use]
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Number of bytes written so far.
    #[must_use]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Whether nothing has been written.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Appends raw bytes with no prefix. The caller is responsible for the
    /// format being self-delimiting (fixed width, or preceded by a length).
    pub fn put_raw(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    /// Appends one byte.
    pub fn put_u8(&mut self, x: u8) {
        self.buf.push(x);
    }

    /// Appends a little-endian `u16`.
    pub fn put_u16(&mut self, x: u16) {
        self.put_raw(&x.to_le_bytes());
    }

    /// Appends a little-endian `u32`.
    pub fn put_u32(&mut self, x: u32) {
        self.put_raw(&x.to_le_bytes());
    }

    /// Appends a little-endian `u64`.
    pub fn put_u64(&mut self, x: u64) {
        self.put_raw(&x.to_le_bytes());
    }

    /// Appends a `bool` as the byte `0` or `1`.
    pub fn put_bool(&mut self, x: bool) {
        self.put_u8(u8::from(x));
    }

    /// Appends a `u32` little-endian length prefix.
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] when `len` does not fit in `u32`.
    pub fn put_len(&mut self, len: usize) -> Result<()> {
        let prefix = u32::try_from(len).map_err(|_| Error::LengthTooLarge {
            declared: u64::try_from(len).unwrap_or(u64::MAX),
            max: u64::from(u32::MAX),
        })?;
        self.put_u32(prefix);
        Ok(())
    }

    /// Appends length-prefixed bytes.
    ///
    /// # Errors
    /// [`Error::LengthTooLarge`] when `bytes.len()` does not fit in `u32`.
    pub fn put_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.put_len(bytes.len())?;
        self.put_raw(bytes);
        Ok(())
    }

    /// Returns the encoded bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}
