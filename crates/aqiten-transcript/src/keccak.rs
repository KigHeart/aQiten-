//! Keccak-f[1600] and SHAKE256 (FIPS 202), written out in full so the crate
//! has no external dependency. Verified against Python's `hashlib.shake_256`
//! (see the known-answer tests).
//!
//! Type-states prevent misuse: [`Shake256`] can only absorb, and
//! [`Shake256Reader`] (obtained by [`Shake256::finalize`]) can only squeeze.
//!
//! All array indices below are loop variables bounded by fixed array lengths
//! (`< 25`, `< 5`, `< 24`) and byte positions are reduced modulo the rate
//! `136`, so the module opts out of the workspace indexing/arithmetic lints.
#![allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]

/// SHAKE256 rate in bytes: `(1600 - 2*256) / 8`.
const RATE: usize = 136;

const RC: [u64; 24] = [
    0x0000_0000_0000_0001,
    0x0000_0000_0000_8082,
    0x8000_0000_0000_808a,
    0x8000_0000_8000_8000,
    0x0000_0000_0000_808b,
    0x0000_0000_8000_0001,
    0x8000_0000_8000_8081,
    0x8000_0000_0000_8009,
    0x0000_0000_0000_008a,
    0x0000_0000_0000_0088,
    0x0000_0000_8000_8009,
    0x0000_0000_8000_000a,
    0x0000_0000_8000_808b,
    0x8000_0000_0000_008b,
    0x8000_0000_0000_8089,
    0x8000_0000_0000_8003,
    0x8000_0000_0000_8002,
    0x8000_0000_0000_0080,
    0x0000_0000_0000_800a,
    0x8000_0000_8000_000a,
    0x8000_0000_8000_8081,
    0x8000_0000_0000_8080,
    0x0000_0000_8000_0001,
    0x8000_0000_8000_8008,
];
const ROTC: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];
const PILN: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

/// The Keccak-f[1600] permutation.
fn keccak_f(st: &mut [u64; 25]) {
    for rc in RC {
        // theta
        let mut bc = [0u64; 5];
        for i in 0..5 {
            bc[i] = st[i] ^ st[i + 5] ^ st[i + 10] ^ st[i + 15] ^ st[i + 20];
        }
        for i in 0..5 {
            let t = bc[(i + 4) % 5] ^ bc[(i + 1) % 5].rotate_left(1);
            for j in (0..25).step_by(5) {
                st[j + i] ^= t;
            }
        }
        // rho and pi
        let mut t = st[1];
        for i in 0..24 {
            let j = PILN[i];
            let next = st[j];
            st[j] = t.rotate_left(ROTC[i]);
            t = next;
        }
        // chi
        for j in (0..25).step_by(5) {
            bc.copy_from_slice(&st[j..j + 5]);
            for i in 0..5 {
                st[j + i] ^= (!bc[(i + 1) % 5]) & bc[(i + 2) % 5];
            }
        }
        // iota
        st[0] ^= rc;
    }
}

/// XORs one byte into the sponge state at byte position `pos < 200`.
fn xor_byte(st: &mut [u64; 25], pos: usize, b: u8) {
    st[pos / 8] ^= u64::from(b) << (8 * (pos % 8));
}

/// Reads the state byte at position `pos < 200`.
fn state_byte(st: &[u64; 25], pos: usize) -> u8 {
    (st[pos / 8] >> (8 * (pos % 8))).to_le_bytes()[0]
}

/// SHAKE256 in the absorbing phase.
#[derive(Clone)]
pub struct Shake256 {
    st: [u64; 25],
    pos: usize,
    perms: u64,
}

impl Default for Shake256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Shake256 {
    /// A fresh sponge.
    #[must_use]
    pub fn new() -> Self {
        Self {
            st: [0; 25],
            pos: 0,
            perms: 0,
        }
    }

    /// Absorbs `data` (any length, any chunking: the result depends only on the
    /// concatenation of everything absorbed).
    pub fn absorb(&mut self, data: &[u8]) {
        for &b in data {
            xor_byte(&mut self.st, self.pos, b);
            self.pos += 1;
            if self.pos == RATE {
                keccak_f(&mut self.st);
                self.perms += 1;
                self.pos = 0;
            }
        }
    }

    /// Number of Keccak-f permutations executed so far (the exact hash-query
    /// count used by the ledger; the final padding permutation is added by
    /// [`Self::finalize`]).
    #[must_use]
    pub fn permutations(&self) -> u64 {
        self.perms
    }

    /// Applies SHAKE padding (`0x1F ... 0x80`) and switches to squeezing.
    #[must_use]
    pub fn finalize(mut self) -> Shake256Reader {
        xor_byte(&mut self.st, self.pos, 0x1F);
        xor_byte(&mut self.st, RATE - 1, 0x80);
        keccak_f(&mut self.st);
        Shake256Reader {
            st: self.st,
            pos: 0,
            perms: self.perms + 1,
        }
    }
}

/// SHAKE256 in the squeezing phase: an unbounded output stream.
#[derive(Clone)]
pub struct Shake256Reader {
    st: [u64; 25],
    pos: usize,
    perms: u64,
}

impl Shake256Reader {
    /// Fills `out` with the next bytes of the stream.
    pub fn squeeze(&mut self, out: &mut [u8]) {
        for o in out.iter_mut() {
            if self.pos == RATE {
                keccak_f(&mut self.st);
                self.perms += 1;
                self.pos = 0;
            }
            *o = state_byte(&self.st, self.pos);
            self.pos += 1;
        }
    }

    /// Permutations executed so far, counting those done while absorbing.
    #[must_use]
    pub fn permutations(&self) -> u64 {
        self.perms
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::cast_possible_truncation)]
    use super::*;
    use alloc::vec::Vec;

    fn shake(input: &[u8], n: usize) -> Vec<u8> {
        let mut s = Shake256::new();
        s.absorb(input);
        let mut out = alloc::vec![0u8; n];
        s.finalize().squeeze(&mut out);
        out
    }

    fn hex(b: &[u8]) -> alloc::string::String {
        use core::fmt::Write;
        let mut s = alloc::string::String::new();
        for x in b {
            let _ = write!(s, "{x:02x}");
        }
        s
    }

    #[test]
    fn nist_empty_message() {
        assert_eq!(
            hex(&shake(b"", 64)),
            "46b9dd2b0ba88d13233b3feb743eeb243fcd52ea62b81b82b50c27646ed5762f\
             d75dc4ddd8c0f200cb05019d67b592f6fc821c49479ab48640292eacb3b7c4be"
        );
    }

    #[test]
    fn chunking_does_not_matter_and_squeeze_is_a_stream() {
        let msg: Vec<u8> = (0..1000u32).map(|i| (i * 7 + 3) as u8).collect();
        let whole = shake(&msg, 400);
        for chunk in [1usize, 7, 135, 136, 137, 500] {
            let mut s = Shake256::new();
            for c in msg.chunks(chunk) {
                s.absorb(c);
            }
            let mut r = s.finalize();
            let mut out = alloc::vec![0u8; 400];
            for piece in out.chunks_mut(13) {
                r.squeeze(piece);
            }
            assert_eq!(out, whole, "chunk {chunk}");
        }
    }

    #[test]
    fn permutation_count_is_exact() {
        // 136-byte message: one absorb permutation + one padding permutation.
        let mut s = Shake256::new();
        s.absorb(&[0u8; 136]);
        assert_eq!(s.permutations(), 1);
        let mut r = s.finalize();
        assert_eq!(r.permutations(), 2);
        let mut o = [0u8; 137];
        r.squeeze(&mut o);
        assert_eq!(r.permutations(), 3);
    }
}
