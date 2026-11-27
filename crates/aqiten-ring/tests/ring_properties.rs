//! Integration tests: the ring multiplication is checked against an independent
//! integer-convolution reference that shares no code with the crate.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::missing_docs_in_private_items,
    clippy::many_single_char_names,
    clippy::needless_range_loop,
    clippy::eq_op
)]

use aqiten_ring::RingElem;

/// Reference: schoolbook product in Z[X], reduced by X^D = -1, then mod p. Uses i128 only.
fn reference_mul(a: &[i64], b: &[i64], p: i64) -> Vec<i64> {
    let d = a.len();
    let mut out = vec![0i128; d];
    for i in 0..d {
        for j in 0..d {
            // reduce each product first: unreduced sums of 2^126-sized terms overflow i128
            let prod = (i128::from(a[i]) * i128::from(b[j])).rem_euclid(i128::from(p));
            let k = i + j;
            if k < d {
                out[k] += prod;
            } else {
                out[k - d] -= prod;
            }
        }
    }
    out.iter()
        .map(|&v| v.rem_euclid(i128::from(p)) as i64)
        .collect()
}

fn canon<const P: u64, const D: usize>(r: &RingElem<P, D>) -> Vec<i64> {
    r.coeffs().iter().map(|x| x.to_canonical() as i64).collect()
}

struct Xs(u64);
impl Xs {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

fn rand_vec(g: &mut Xs, d: usize, p: u64) -> Vec<i64> {
    (0..d).map(|_| (g.next() % p) as i64).collect()
}

fn elem<const P: u64, const D: usize>(v: &[i64]) -> RingElem<P, D> {
    RingElem::from_centered(v).unwrap()
}

#[test]
fn exhaustive_d2_p5_all_pairs() {
    for a0 in 0..5i64 {
        for a1 in 0..5 {
            for b0 in 0..5 {
                for b1 in 0..5 {
                    let (a, b) = (vec![a0, a1], vec![b0, b1]);
                    let got = elem::<5, 2>(&a) * elem::<5, 2>(&b);
                    assert_eq!(canon(&got), reference_mul(&a, &b, 5));
                }
            }
        }
    }
}

#[test]
fn exhaustive_d4_p3_all_pairs() {
    let all: Vec<Vec<i64>> = (0..81)
        .map(|n| (0..4).map(|i| (n / 3i64.pow(i)) % 3).collect())
        .collect();
    for a in &all {
        for b in &all {
            let got = elem::<3, 4>(a) * elem::<3, 4>(b);
            assert_eq!(canon(&got), reference_mul(a, b, 3));
        }
    }
}

fn randomized<const P: u64, const D: usize>(seed: u64) {
    let mut g = Xs(seed);
    for _ in 0..200 {
        let (a, b, c) = (
            rand_vec(&mut g, D, P),
            rand_vec(&mut g, D, P),
            rand_vec(&mut g, D, P),
        );
        let (ra, rb, rc) = (elem::<P, D>(&a), elem::<P, D>(&b), elem::<P, D>(&c));
        // against the independent reference
        assert_eq!(canon(&(ra * rb)), reference_mul(&a, &b, P as i64));
        // ring axioms
        assert_eq!(ra * rb, rb * ra);
        assert_eq!((ra * rb) * rc, ra * (rb * rc));
        assert_eq!(ra * (rb + rc), ra * rb + ra * rc);
        assert_eq!(ra * RingElem::one(), ra);
        assert_eq!(ra * RingElem::ZERO, RingElem::ZERO);
        assert_eq!(ra - ra, RingElem::ZERO);
        assert_eq!(ra + (-ra), RingElem::ZERO);
        assert_eq!((ra + rb) - rb, ra);
        // scale agrees with multiplication by a constant
        let s = aqiten_field::Fp::<P>::from_canonical(g.next() % P).unwrap();
        assert_eq!(ra.scale(s), ra * RingElem::constant(s));
        // monomial agrees with multiplication by X^k
        let k = (g.next() % (3 * D as u64)) as usize;
        let mut xk = RingElem::<P, D>::one();
        for _ in 0..k {
            xk = xk.mul_x();
        }
        assert_eq!(ra.mul_monomial(k), ra * xk);
    }
}

#[test]
fn randomized_small_prime() {
    randomized::<257, 8>(1);
    randomized::<5, 4>(2);
}

#[test]
fn randomized_q_equiv_5_mod_8() {
    // 4294967197 = 5 mod 8: the class for which a full negacyclic NTT does not exist.
    assert_eq!(4_294_967_197u64 % 8, 5);
    randomized::<4_294_967_197, 16>(3);
    randomized::<4_294_967_197, 64>(4);
}

#[test]
fn randomized_largest_modulus() {
    randomized::<9_223_372_036_854_775_783, 8>(5); // 2^63 - 25
    randomized::<2_305_843_009_213_693_951, 32>(6); // 2^61 - 1
}

#[test]
fn degree_two_is_the_gaussian_like_case() {
    // Z_5[X]/(X^2+1): X^2 = -1, (2+X)(2-X) = 4 - X^2 = 5 = 0 (zero divisors exist in R_q)
    let a = elem::<5, 2>(&[2, 1]);
    let b = elem::<5, 2>(&[2, -1]);
    assert_eq!(a * b, RingElem::ZERO);
}
