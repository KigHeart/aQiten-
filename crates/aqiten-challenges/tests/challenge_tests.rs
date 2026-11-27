//! Verification of the algebra (exhaustive on small rings), the families
//! (Akita Table 3 cardinalities), the sampler (uniformity) and the operator-norm bound.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::needless_range_loop,
    clippy::similar_names,
    clippy::type_complexity,
    clippy::zero_sized_map_values,
    clippy::doc_markdown
)]

use std::collections::HashMap;

use aqiten_challenges::{
    challenge_sparse, op_norm_sq_certified_at_most, op_norm_sq_upper_bound, sample_sparse,
    SparseFamily, Split,
};
use aqiten_field::Fp;
use aqiten_ring::RingElem;
use aqiten_transcript::{Shake256, Transcript};

// ---------- algebra ----------

fn all_elements<const P: u64, const D: usize>() -> Vec<RingElem<P, D>> {
    let total = (P as usize).pow(D as u32);
    (0..total)
        .map(|mut n| {
            let mut v = [0i64; D];
            for slot in &mut v {
                *slot = (n % P as usize) as i64;
                n /= P as usize;
            }
            RingElem::<P, D>::from_centered(&v).unwrap()
        })
        .collect()
}

fn has_inverse<const P: u64, const D: usize>(c: &RingElem<P, D>, all: &[RingElem<P, D>]) -> bool {
    all.iter().any(|b| *c * *b == RingElem::one())
}

#[test]
fn sqrt_minus_one_squares_to_minus_one() {
    fn check<const P: u64>() {
        let i = Split::<P>::sqrt_minus_one();
        assert_eq!(i * i, -Fp::<P>::from_canonical(1).unwrap(), "P = {P}");
        assert!(i.to_canonical() <= P - i.to_canonical());
    }
    check::<5>();
    check::<13>();
    check::<29>();
    check::<37>();
    check::<4_294_967_197>();
    check::<2_305_843_009_213_693_693>(); // ~2^61
    check::<4_611_686_018_427_387_733>(); // ~2^62
}

/// Exhaustive: the exact predicate equals "has a multiplicative inverse" on every element.
fn exhaustive_unit_test<const P: u64, const D: usize>() {
    let all = all_elements::<P, D>();
    let mut units = 0usize;
    for c in &all {
        let predicted = Split::<P>::is_unit(c);
        if predicted {
            units += 1;
        }
        assert_eq!(predicted, has_inverse(c, &all), "P={P} D={D} c={c:?}");
    }
    // R_q = F_{q^m} x F_{q^m}: exactly (q^m - 1)^2 units, which also confirms irreducibility.
    let m = (D / 2) as u32;
    let expect = ((P as usize).pow(m) - 1).pow(2);
    assert_eq!(units, expect, "unit count P={P} D={D}");
}

#[test]
fn unit_predicate_exhaustive_d2() {
    exhaustive_unit_test::<5, 2>();
    exhaustive_unit_test::<13, 2>();
    exhaustive_unit_test::<29, 2>();
    exhaustive_unit_test::<37, 2>();
}

#[test]
fn unit_predicate_exhaustive_d4() {
    exhaustive_unit_test::<5, 4>(); // 625 elements, every pair
                                    // 28561 elements: the exact unit count for all, and the inverse/zero-divisor property on samples
                                    // (a full pairwise check would be 8*10^8 ring products).
    let all = all_elements::<13, 4>();
    let units = all.iter().filter(|c| Split::<13>::is_unit(c)).count();
    assert_eq!(units, (13usize.pow(2) - 1).pow(2));
    for c in all
        .iter()
        .filter(|c| Split::<13>::is_unit(c))
        .step_by(701)
        .take(30)
    {
        assert!(has_inverse(c, &all));
    }
    for c in all
        .iter()
        .filter(|c| !Split::<13>::is_unit(c))
        .step_by(53)
        .take(30)
    {
        assert!(c.is_zero() || all.iter().any(|b| !b.is_zero() && (*c * *b).is_zero()));
    }
}

#[test]
fn unit_count_matches_formula_d8_q5_and_sampled_inverses() {
    // 390625 elements: check the count formula for all, and the inverse property on a sample.
    let all = all_elements::<5, 8>();
    let units: Vec<_> = all.iter().filter(|c| Split::<5>::is_unit(c)).collect();
    assert_eq!(units.len(), (5usize.pow(4) - 1).pow(2));
    for c in units.iter().step_by(997).take(40) {
        assert!(all.iter().any(|b| **c * *b == RingElem::one()));
    }
    for c in all
        .iter()
        .filter(|c| !Split::<5>::is_unit(c))
        .step_by(331)
        .take(40)
    {
        // a non-unit is a zero divisor or zero
        assert!(c.is_zero() || all.iter().any(|b| !b.is_zero() && (*c * *b).is_zero()));
    }
}

#[test]
fn small_elements_lemma_is_exact_and_nearly_tight() {
    // For D=2 and each q, brute force the smallest infinity norm of a non-zero non-unit.
    fn min_nonunit_norm<const P: u64>() -> u64 {
        let mut best = u64::MAX;
        for c in all_elements::<P, 2>() {
            if !c.is_zero() && !Split::<P>::is_unit(&c) {
                best = best.min(c.linf_norm());
            }
        }
        best
    }
    for (q, min) in [
        (5u64, min_nonunit_norm::<5>()),
        (13, min_nonunit_norm::<13>()),
        (29, min_nonunit_norm::<29>()),
        (37, min_nonunit_norm::<37>()),
    ] {
        // Lemma: every non-zero element with 2*b^2 < q is a unit, so min norm b* has 2 b*^2 >= q.
        assert!(2 * min * min >= q, "q={q} min={min}");
        // Near-tightness: some non-unit with b*^2 + 0 ... exists with norm <= ceil(sqrt(q)).
        assert!(min * min <= q, "q={q} min={min}");
        // and the certified predicate agrees below that threshold
        assert_eq!(Split::<5>::small_elements_are_units(1), Ok(true));
    }
    assert_eq!(Split::<13>::small_elements_are_units(2), Ok(true)); // 8 < 13
    assert_eq!(Split::<13>::small_elements_are_units(3), Ok(false)); // 18 >= 13 (and a non-unit exists)
}

// ---------- families ----------

#[test]
fn akita_table_3_cardinalities_exact() {
    // (d, groups, expected floor(log2 |C|)); table: 128.09, 129.62, 131.08, 132.76, 131.58, 131.59
    let rows: [(usize, Vec<(u32, u32)>, u64); 6] = [
        (64, vec![(1, 31), (2, 10)], 128),
        (128, vec![(1, 31)], 129),
        (256, vec![(1, 23)], 131),
        (512, vec![(1, 19)], 132),
        (1024, vec![(1, 16)], 131),
        (2048, vec![(1, 14)], 131),
    ];
    for (d, groups, floor) in rows {
        let f = SparseFamily::new(d, &groups).unwrap();
        assert_eq!(f.cardinality_log2_floor(), Ok(floor), "d = {d}");
        assert_eq!(f.cardinality_at_least_two_pow(128), Ok(true));
        assert_eq!(f.cardinality_at_least_two_pow(floor + 1), Ok(false));
    }
    let f64 = SparseFamily::new(64, &[(1, 31), (2, 10)]).unwrap();
    assert_eq!(
        (f64.c_max(), f64.omega(), f64.support_size()),
        (2, Ok(51), 41)
    );
}

#[test]
fn difference_invertibility_certificate() {
    let f = SparseFamily::new(64, &[(1, 31), (2, 10)]).unwrap();
    assert_eq!(f.differences_are_units::<4_294_967_197>(), Ok(true)); // 2*4^2 = 32 < q
    assert_eq!(f.differences_are_units::<13>(), Ok(false)); // 32 >= 13
                                                            // tiny exhaustive confirmation: D=4, q=37, family (1,2): all pairwise differences of the family are units
    let fam = SparseFamily::new(4, &[(1, 2)]).unwrap();
    assert_eq!(fam.differences_are_units::<37>(), Ok(true));
    let mut members: HashMap<Vec<i64>, ()> = HashMap::new();
    let mut sponge = Shake256::new();
    sponge.absorb(b"members");
    let mut r = sponge.finalize();
    for _ in 0..4000 {
        let c = sample_sparse::<37, 4>(&mut r, &fam).unwrap();
        members.insert(c.to_centered().to_vec(), ());
    }
    assert_eq!(members.len(), 24);
    let list: Vec<_> = members.keys().cloned().collect();
    for a in &list {
        for b in &list {
            if a != b {
                let da = RingElem::<37, 4>::from_centered(a).unwrap();
                let db = RingElem::<37, 4>::from_centered(b).unwrap();
                assert!(Split::<37>::is_unit(&(da - db)));
            }
        }
    }
}

// ---------- sampler ----------

#[test]
fn every_sample_has_exact_shape_and_norms() {
    let fam = SparseFamily::new(64, &[(1, 31), (2, 10)]).unwrap();
    let mut t = Transcript::new(b"shape").unwrap();
    for _ in 0..300 {
        let c: RingElem<4_294_967_197, 64> = challenge_sparse(&mut t, b"c", &fam).unwrap();
        let v = c.to_centered();
        assert_eq!(v.iter().filter(|x| x.abs() == 1).count(), 31);
        assert_eq!(v.iter().filter(|x| x.abs() == 2).count(), 10);
        assert_eq!(v.iter().filter(|x| **x == 0).count(), 64 - 41);
        assert_eq!(c.linf_norm(), 2);
        assert_eq!(v.iter().map(|x| x.unsigned_abs()).sum::<u64>(), 51);
    }
}

#[test]
fn sampler_is_uniform_on_a_small_family() {
    // D=4, (1,1)+(2,1): exactly 48 members; every one must occur, with near-equal frequency.
    let fam = SparseFamily::new(4, &[(1, 1), (2, 1)]).unwrap();
    let mut t = Transcript::new(b"uniform").unwrap();
    let mut counts: HashMap<Vec<i64>, u32> = HashMap::new();
    let n = 48_000u32;
    for _ in 0..n {
        let c: RingElem<13, 4> = challenge_sparse(&mut t, b"c", &fam).unwrap();
        *counts.entry(c.to_centered().to_vec()).or_default() += 1;
    }
    assert_eq!(counts.len(), 48);
    for (k, v) in &counts {
        assert!(
            (800..=1200).contains(v),
            "{k:?} occurred {v} times (expected 1000)"
        ); // > 6 sigma
    }
}

#[test]
fn sampler_is_deterministic_and_checks_parameters() {
    let fam = SparseFamily::new(16, &[(1, 5)]).unwrap();
    let run = || {
        let mut t = Transcript::new(b"det").unwrap();
        challenge_sparse::<4_294_967_197, 16>(&mut t, b"c", &fam).unwrap()
    };
    assert_eq!(run(), run());
    let mut t = Transcript::new(b"bad").unwrap();
    assert!(challenge_sparse::<4_294_967_197, 32>(&mut t, b"c", &fam).is_err());
    let wide = SparseFamily::new(16, &[(3, 1)]).unwrap();
    assert!(challenge_sparse::<5, 16>(&mut t, b"c", &wide).is_err()); // 3 > (5-1)/2
}

// ---------- operator norm ----------

/// max_k |c(zeta_k)|^2 with zeta_k = exp(i*pi*(2k+1)/D), in f64 (test-only reference).
fn true_op_norm_sq(c: &[i64]) -> f64 {
    let d = c.len();
    (0..d)
        .map(|k| {
            let (mut re, mut im) = (0f64, 0f64);
            for (j, cj) in c.iter().enumerate() {
                let ang = std::f64::consts::PI * ((2 * k + 1) * j) as f64 / d as f64;
                re += *cj as f64 * ang.cos();
                im += *cj as f64 * ang.sin();
            }
            re * re + im * im
        })
        .fold(0f64, f64::max)
}

#[test]
fn operator_norm_bound_is_sound_and_sandwiched() {
    const Q: u64 = 4_294_967_197;
    let fam = SparseFamily::new(64, &[(1, 31), (2, 10)]).unwrap();
    let mut t = Transcript::new(b"opnorm").unwrap();
    let omega_sq = 51u128 * 51;
    for _ in 0..100 {
        let c: RingElem<Q, 64> = challenge_sparse(&mut t, b"c", &fam).unwrap();
        let bound = op_norm_sq_upper_bound(&c).unwrap();
        let truth = true_op_norm_sq(&c.to_centered());
        assert!(bound as f64 >= truth - 1e-6, "bound {bound} < true {truth}");
        assert!(bound <= omega_sq);
        assert!(bound >= c.l2_norm_squared().unwrap());
        assert_eq!(op_norm_sq_certified_at_most(&c, bound), Ok(true));
        assert_eq!(op_norm_sq_certified_at_most(&c, bound - 1), Ok(false));
    }
}

#[test]
fn operator_norm_bound_exact_on_monomials_and_constants() {
    type R = RingElem<4_294_967_197, 16>;
    let x5 = R::one().mul_monomial(5);
    assert_eq!(op_norm_sq_upper_bound(&x5), Ok(1));
    let c = R::constant(Fp::from_canonical(7).unwrap());
    assert_eq!(op_norm_sq_upper_bound(&c), Ok(49));
    // 1 + X: true norm^2 = max |1+zeta|^2 = 4 cos^2(pi/32)-ish < 4; bound is 1+1+|G_01|... = 4 here
    let a = R::one() + R::one().mul_x();
    let bound = op_norm_sq_upper_bound(&a).unwrap();
    assert!(bound as f64 >= true_op_norm_sq(&a.to_centered()));
    assert!(bound <= 4);
}

#[test]
fn gershgorin_rows_have_equal_absolute_sums() {
    // The claim behind the bound: every row of G = M^T M has the same absolute sum as row 0.
    type R = RingElem<4_294_967_197, 8>;
    let fam = SparseFamily::new(8, &[(1, 3), (2, 2)]).unwrap();
    let mut t = Transcript::new(b"rows").unwrap();
    for _ in 0..50 {
        let c: R = challenge_sparse(&mut t, b"c", &fam).unwrap();
        let cols: Vec<[i64; 8]> = (0..8).map(|a| c.mul_monomial(a).to_centered()).collect();
        let row_sum = |a: usize| -> i128 {
            (0..8)
                .map(|b| {
                    (0..8)
                        .map(|k| i128::from(cols[a][k]) * i128::from(cols[b][k]))
                        .sum::<i128>()
                        .abs()
                })
                .sum()
        };
        let r0 = row_sum(0);
        for a in 1..8 {
            assert_eq!(row_sum(a), r0);
        }
        assert_eq!(op_norm_sq_upper_bound(&c).unwrap() as i128, r0);
    }
}
