//! Canonical wire encoding of ring elements: `D` coefficients, each in the
//! canonical fixed-width encoding of [`aqiten_field::Fp`], no length prefix.

use aqiten_codec::{Decode, Encode, Reader, Writer};
use aqiten_error::Result;
use aqiten_field::Fp;

use crate::RingElem;

impl<const P: u64, const D: usize> RingElem<P, D> {
    /// Encoded size in bytes: `D * Fp::ENCODED_BYTES`.
    pub const ENCODED_BYTES: usize = D * Fp::<P>::ENCODED_BYTES;
}

impl<const P: u64, const D: usize> Encode for RingElem<P, D> {
    fn encode(&self, w: &mut Writer) -> Result<()> {
        for x in &self.c {
            x.encode(w)?;
        }
        Ok(())
    }
}

impl<const P: u64, const D: usize> Decode for RingElem<P, D> {
    fn decode(r: &mut Reader<'_>) -> Result<Self> {
        let mut out = Self::ZERO;
        for slot in &mut out.c {
            *slot = Fp::<P>::decode(r)?;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use aqiten_codec::{decode_exact, encode_to_vec};
    use aqiten_error::Error;

    use crate::RingElem;

    type R = RingElem<257, 4>; // 2 bytes per coefficient

    #[test]
    fn known_answer_and_width() {
        let a = R::from_centered(&[1, 2, 3, -1]).unwrap_or(R::ZERO);
        let b = encode_to_vec(&a).unwrap_or_default();
        assert_eq!(b, alloc::vec![1, 0, 2, 0, 3, 0, 0, 1]); // -1 = 256 = 0x0100 LE
        assert_eq!(b.len(), R::ENCODED_BYTES);
        assert_eq!(decode_exact::<R>(&b), Ok(a));
    }

    #[test]
    fn rejects_non_canonical_truncated_and_trailing() {
        // coefficient 257 (= modulus) is non-canonical
        let bad = [1, 0, 2, 0, 3, 0, 1, 1];
        assert!(matches!(
            decode_exact::<R>(&bad),
            Err(Error::NonCanonical { .. })
        ));
        assert!(matches!(
            decode_exact::<R>(&[1, 0, 2, 0]),
            Err(Error::Truncated { .. })
        ));
        assert!(matches!(
            decode_exact::<R>(&[0; 9]),
            Err(Error::TrailingBytes { remaining: 1 })
        ));
    }

    #[test]
    fn accepted_inputs_re_encode_identically() {
        let mut s: u64 = 7;
        for _ in 0..2000 {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            let bytes = s.to_le_bytes();
            if let Ok(v) = decode_exact::<R>(&bytes) {
                assert_eq!(encode_to_vec(&v), Ok(bytes.to_vec()));
            }
        }
    }
}
