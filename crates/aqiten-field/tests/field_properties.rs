#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::similar_names,
    clippy::unreadable_literal,
    clippy::items_after_statements
)]
//! Property and known-answer tests for `aqiten-field`.
//!
//! Strategy: (1) exhaustive equality with a naive `u128 % P` reference over all
//! operand pairs for small primes; (2) pseudorandom comparison against the same
//! reference for the primes the project is likely to use, including the largest
//! allowed prime; (3) known-answer vectors computed independently in Python;
//! (4) canonical-encoding strictness.

use aqiten_codec::{decode_exact, encode_to_vec, Reader};
use aqiten_error::Error;
use aqiten_field::{Field, Fp};

/// Deterministic xorshift64* stream for reproducible tests.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

fn naive_inv(a: u64, p: u64) -> u64 {
    // Extended Euclid on i128, independent of the Fermat route in the crate.
    let (mut r0, mut r1) = (i128::from(p), i128::from(a));
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    assert_eq!(r0, 1, "a must be invertible");
    u64::try_from(t0.rem_euclid(i128::from(p))).expect("fits")
}

fn exhaustive<const P: u64>() {
    type F<const P: u64> = Fp<P>;
    let p = u128::from(P);
    for a in 0..P {
        let fa = F::<P>::from_canonical(a).expect("canonical");
        assert_eq!(fa.to_canonical(), a);
        assert_eq!((-fa).to_canonical(), ((p - u128::from(a)) % p) as u64);
        if a != 0 {
            let inv = fa.inv().expect("non-zero");
            assert_eq!(inv.to_canonical(), naive_inv(a, P), "inv of {a} mod {P}");
            assert_eq!(fa * inv, F::<P>::ONE);
        } else {
            assert_eq!(fa.inv(), Err(Error::NotInvertible));
        }
        for b in 0..P {
            let fb = F::<P>::from_canonical(b).expect("canonical");
            assert_eq!(
                (fa + fb).to_canonical(),
                ((u128::from(a) + u128::from(b)) % p) as u64
            );
            assert_eq!(
                (fa - fb).to_canonical(),
                ((u128::from(a) + p - u128::from(b)) % p) as u64
            );
            assert_eq!(
                (fa * fb).to_canonical(),
                ((u128::from(a) * u128::from(b)) % p) as u64
            );
        }
    }
}

#[test]
fn exhaustive_small_primes() {
    exhaustive::<3>();
    exhaustive::<5>();
    exhaustive::<7>();
    exhaustive::<13>();
    exhaustive::<97>();
    exhaustive::<257>();
}

fn randomized<const P: u64>(iterations: usize, seed: u64) {
    let p = u128::from(P);
    let mut rng = Rng(seed);
    // Edge values first: 0, 1, 2, P-1, P-2, (P-1)/2, (P+1)/2.
    let edges = [0, 1, 2, P - 1, P - 2, (P - 1) / 2, P.div_ceil(2)];
    let mut samples: Vec<u64> = edges.to_vec();
    for _ in 0..iterations {
        samples.push(rng.next() % P);
    }
    for (i, &a) in samples.iter().enumerate() {
        let fa = Fp::<P>::from_canonical(a).expect("canonical");
        let b = samples[(i * 7 + 3) % samples.len()];
        let fb = Fp::<P>::from_canonical(b).expect("canonical");
        assert_eq!(
            (fa + fb).to_canonical(),
            ((u128::from(a) + u128::from(b)) % p) as u64
        );
        assert_eq!(
            (fa - fb).to_canonical(),
            ((u128::from(a) + p - u128::from(b)) % p) as u64
        );
        assert_eq!(
            (fa * fb).to_canonical(),
            ((u128::from(a) * u128::from(b)) % p) as u64
        );
        assert_eq!(fa.square(), fa * fa);
        if a != 0 {
            assert_eq!(fa.inv().expect("non-zero").to_canonical(), naive_inv(a, P));
            // Fermat's little theorem.
            assert_eq!(fa.pow(P - 1), Fp::<P>::ONE);
        }
        // Field axioms on a triple.
        let c = samples[(i * 13 + 5) % samples.len()];
        let fc = Fp::<P>::from_canonical(c).expect("canonical");
        assert_eq!(fa * (fb + fc), fa * fb + fa * fc, "distributivity");
        assert_eq!((fa + fb) + fc, fa + (fb + fc), "additive associativity");
        assert_eq!(
            (fa * fb) * fc,
            fa * (fb * fc),
            "multiplicative associativity"
        );
        assert_eq!(fa + (-fa), Fp::<P>::ZERO);
        assert_eq!(fa * Fp::<P>::ONE, fa);
        assert_eq!(fa + Fp::<P>::ZERO, fa);
    }
}

#[test]
fn randomized_against_reference() {
    randomized::<65_537>(20_000, 1);
    randomized::<2_147_483_629>(20_000, 2); // 5 mod 8
    randomized::<4_294_967_197>(20_000, 3); // 5 mod 8
    randomized::<2_305_843_009_213_693_951>(20_000, 4); // 2^61 - 1
    randomized::<9_223_372_036_854_775_783>(20_000, 5); // largest allowed prime, 2^63 - 25
}

#[test]
fn known_answers_independent_python() {
    type F = Fp<4_294_967_197>;
    assert_eq!(
        F::from_u64(123_456_789).inv().expect("nz").to_canonical(),
        442_611_708
    );
    assert_eq!(
        F::from_u64(5).pow(1_000_000_000_000_000_000).to_canonical(),
        3_724_813_099
    );
    assert_eq!(F::from_u64(7).pow(4_294_967_196).to_canonical(), 1);
    assert_eq!(
        F::from_u64(u64::MAX).to_canonical(),
        u64::MAX % 4_294_967_197
    );

    type G = Fp<2_147_483_629>;
    assert_eq!(
        G::from_u64(3).inv().expect("nz").to_canonical(),
        1_431_655_753
    );
    assert_eq!(G::from_u64(2).pow(100).to_canonical(), 877_952);

    type M = Fp<2_305_843_009_213_693_951>;
    assert_eq!(
        M::from_u64((1 << 40) + 12_345)
            .inv()
            .expect("nz")
            .to_canonical(),
        1_356_555_169_007_482_819
    );
    assert_eq!(
        M::from_u64(3).pow((1 << 63) + 12_345).to_canonical(),
        864_701_107_312_448_298
    );

    type B = Fp<9_223_372_036_854_775_783>;
    assert_eq!(
        B::from_u64(987_654_321_987)
            .inv()
            .expect("nz")
            .to_canonical(),
        656_776_943_347_493_733
    );
}

#[test]
fn signed_conversions_match_reference() {
    type F = Fp<4_294_967_197>;
    let p = 4_294_967_197i128;
    let cases = [
        0i64,
        1,
        -1,
        2,
        -2,
        123_456_789,
        -123_456_789,
        i64::MAX,
        i64::MIN,
        i64::MIN + 1,
        4_294_967_197,
        -4_294_967_197,
        4_294_967_196,
        -4_294_967_196,
    ];
    for &x in &cases {
        let expected = i128::from(x).rem_euclid(p) as u64;
        assert_eq!(F::from_i64(x).to_canonical(), expected, "from_i64({x})");
    }
    // Spot values computed independently.
    assert_eq!(F::from_i64(i64::MIN).to_canonical(), 2_147_478_698);
    assert_eq!(F::from_i64(i64::MAX).to_canonical(), 2_147_488_498);
    assert_eq!(F::from_i64(-123_456_789).to_canonical(), 4_171_510_408);
}

#[test]
fn centered_lift_matches_reference_and_inverts_from_i64() {
    type F = Fp<4_294_967_197>;
    let p = 4_294_967_197i64;
    let half = (p - 1) / 2;
    for x in [
        0u64,
        1,
        2,
        100,
        2_147_483_597,
        2_147_483_598,
        2_147_483_599,
        4_294_967_195,
        4_294_967_196,
    ] {
        let f = F::from_canonical(x).expect("canonical");
        let c = f.to_centered_i64();
        let expected = if (x as i64) > half {
            x as i64 - p
        } else {
            x as i64
        };
        assert_eq!(c, expected, "centered({x})");
        assert!((-half..=half).contains(&c));
        assert_eq!(F::from_i64(c), f);
    }
    // Boundary stated in the design notes.
    assert_eq!(
        F::from_canonical(2_147_483_598)
            .expect("c")
            .to_centered_i64(),
        2_147_483_598
    );
    assert_eq!(
        F::from_canonical(2_147_483_599)
            .expect("c")
            .to_centered_i64(),
        -2_147_483_598
    );
    // Round trip over a window around zero.
    for v in -5_000i64..=5_000 {
        assert_eq!(F::from_i64(v).to_centered_i64(), v);
    }
}

#[test]
fn from_canonical_rejects_values_at_or_above_modulus() {
    type F = Fp<4_294_967_197>;
    assert!(F::from_canonical(4_294_967_196).is_ok());
    assert_eq!(
        F::from_canonical(4_294_967_197),
        Err(Error::NonCanonical {
            what: "field element"
        })
    );
    assert_eq!(
        F::from_canonical(u64::MAX),
        Err(Error::NonCanonical {
            what: "field element"
        })
    );
}

#[test]
fn zero_one_and_characteristic() {
    type F = Fp<4_294_967_197>;
    assert!(F::ZERO.is_zero());
    assert!(!F::ONE.is_zero());
    assert_eq!(F::ONE.to_canonical(), 1);
    assert_eq!(F::ZERO.to_canonical(), 0);
    assert_eq!(F::CHARACTERISTIC, 4_294_967_197);
    assert_eq!(F::from_u64(4_294_967_197), F::ZERO);
    assert_eq!(F::from_u64(4_294_967_198), F::ONE);
    assert_eq!(F::ZERO.pow(0), F::ONE); // 0^0 = 1 by convention
    assert_eq!(F::ZERO.pow(5), F::ZERO);
}

#[test]
fn constant_time_helpers() {
    type F = Fp<4_294_967_197>;
    let a = F::from_u64(11);
    let b = F::from_u64(22);
    assert_eq!(a.ct_eq_mask(a), u64::MAX);
    assert_eq!(a.ct_eq_mask(b), 0);
    assert_eq!(F::ZERO.ct_eq_mask(F::ZERO), u64::MAX);
    assert_eq!(F::ct_select(u64::MAX, a, b), a);
    assert_eq!(F::ct_select(0, a, b), b);
}

#[test]
fn encoding_width_and_canonicality() {
    type F = Fp<4_294_967_197>; // P - 1 needs 32 bits => 4 bytes
    assert_eq!(F::ENCODED_BYTES, 4);
    assert_eq!(Fp::<65_537>::ENCODED_BYTES, 3); // P - 1 = 2^16 needs 17 bits
    assert_eq!(Fp::<257>::ENCODED_BYTES, 2); // P - 1 = 256 needs 9 bits
    assert_eq!(Fp::<251>::ENCODED_BYTES, 1);
    assert_eq!(Fp::<9_223_372_036_854_775_783>::ENCODED_BYTES, 8);

    let x = F::from_u64(0x0102_0304);
    assert_eq!(encode_to_vec(&x), Ok(vec![4, 3, 2, 1]));
    assert_eq!(decode_exact::<F>(&[4, 3, 2, 1]), Ok(x));

    // 4_294_967_197 = 0xFFFF_FF9D is the smallest non-canonical word.
    assert_eq!(
        decode_exact::<F>(&[0x9D, 0xFF, 0xFF, 0xFF]),
        Err(Error::NonCanonical {
            what: "field element"
        })
    );
    assert_eq!(
        decode_exact::<F>(&[0x9C, 0xFF, 0xFF, 0xFF]).map(F::to_canonical),
        Ok(4_294_967_196)
    );
    assert_eq!(
        decode_exact::<F>(&[0xFF; 4]),
        Err(Error::NonCanonical {
            what: "field element"
        })
    );

    // Truncation and trailing bytes.
    assert!(matches!(
        decode_exact::<F>(&[1, 2, 3]),
        Err(Error::Truncated { .. })
    ));
    assert!(matches!(
        decode_exact::<F>(&[1, 2, 3, 4, 5]),
        Err(Error::TrailingBytes { .. })
    ));
}

#[test]
fn decode_then_encode_is_identity_for_all_accepted_two_byte_words() {
    type F = Fp<257>;
    for hi in 0u8..=255 {
        for lo in 0u8..=255 {
            let bytes = [lo, hi];
            if let Ok(v) = decode_exact::<F>(&bytes) {
                assert_eq!(encode_to_vec(&v), Ok(bytes.to_vec()));
            } else {
                // Rejected exactly when the little-endian value is >= 257.
                assert!(u16::from_le_bytes(bytes) >= 257);
            }
        }
    }
}

#[test]
fn decoders_do_not_panic_on_random_prefixes() {
    type F = Fp<4_294_967_197>;
    let mut rng = Rng(99);
    for _ in 0..5_000 {
        let buf: Vec<u8> = (0..12).map(|_| rng.next().to_le_bytes()[0]).collect();
        for n in 0..=buf.len() {
            let _ = decode_exact::<F>(&buf[..n]);
            let mut r = Reader::new(&buf[..n]);
            let _ = aqiten_codec::decode_vec::<F>(&mut r, 8);
        }
    }
}
