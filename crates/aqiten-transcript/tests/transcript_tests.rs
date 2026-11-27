//! Known answers from `tools/transcript_model.py` (independent hashlib model) plus
//! behavioural tests of domain separation, ratcheting, sampling and the ledger.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::format_collect
)]

use aqiten_field::Fp;
use aqiten_transcript::{Oracle, Shake256, Transcript};

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn shake256_known_answers_from_python() {
    let cases = [
        (
            1usize,
            "db4252337900d8ab7f609d170135d459a6798945d5a574d4a9db7b623c754d078fb2239762994969",
        ),
        (
            135,
            "0213fc98352f009fafdf8ee1ea36391485a85aa6f6c07a5cd81266d21eb17f9af3e3a6aa4a5e0379",
        ),
        (
            136,
            "c00f43811e5b4a38e14e3c06d8a5ce34115a19cd604ce5bac6c3823b76046d5c6b149f173bef3089",
        ),
        (
            137,
            "3c983983487bcbe74feba53b35bb1e05812379cb4116d9761f78d2ce3177866edc88e750bfaf43bd",
        ),
        (
            272,
            "fbb7df100461f5db3224c3b715603b5a52f92bd0f761d9361d4aa0613d47033bc8d8f05b01b30fe0",
        ),
        (
            1000,
            "980bf59987a720e516297296f92a27bba960e48a40bd01a0415b2e5dee26313d0a3f3ce47abe9e0f",
        ),
    ];
    for (n, expect) in cases {
        let m: Vec<u8> = (0..n).map(|i| ((i * 7 + 3) % 256) as u8).collect();
        let mut s = Shake256::new();
        s.absorb(&m);
        let mut out = [0u8; 40];
        s.finalize().squeeze(&mut out);
        assert_eq!(hex(&out), expect, "n = {n}");
    }
}

const P: u64 = 4_294_967_197;

fn reference_run() -> Transcript {
    Transcript::new(b"test-protocol").unwrap()
}

#[test]
fn transcript_matches_independent_model() {
    let mut t = reference_run();
    t.absorb_bytes(b"msg", b"hello").unwrap();
    t.begin_round(b"r").unwrap();
    let mut c1 = [0u8; 16];
    t.challenge_bytes(b"c1", &mut c1).unwrap();
    assert_eq!(hex(&c1), "94a629d3c0f3731ab3ffeadbbacfcea6");
    t.absorb_bytes(b"msg2", b"").unwrap();
    assert_eq!(t.challenge_below(b"c2", 1000).unwrap(), 311);
    assert_eq!(
        t.challenge_field::<P>(b"c3").unwrap(),
        Fp::<P>::from_canonical(1_274_082_562).unwrap()
    );
    let mut c4 = [0u8; 8];
    t.challenge_bytes(b"c4", &mut c4).unwrap();
    assert_eq!(hex(&c4), "0912eb6919771e2f");
}

#[test]
fn oracle_matches_independent_model_and_is_domain_separated() {
    let mut o = Oracle::new(b"seed", b"A").unwrap();
    let f = |o: &mut Oracle, c: &[u64]| o.field::<P>(c).unwrap().to_canonical();
    assert_eq!(f(&mut o, &[3, 7]), 1_394_824_652);
    assert_eq!(f(&mut o, &[7, 3]), 1_975_810_378); // coordinate order matters
    assert_eq!(f(&mut o, &[]), 1_378_357_464);
    assert_eq!(
        Oracle::new(b"seed", b"B")
            .unwrap()
            .field::<P>(&[3, 7])
            .unwrap()
            .to_canonical(),
        3_453_052_657
    );
    assert_eq!(
        Oracle::new(b"seed2", b"A")
            .unwrap()
            .field::<P>(&[3, 7])
            .unwrap()
            .to_canonical(),
        2_646_591_673
    );
    // query order does not matter: same coordinates, same value
    assert_eq!(f(&mut o, &[3, 7]), 1_394_824_652);
    assert_eq!(o.queries(), 4);
}

fn first_challenge(build: impl FnOnce(&mut Transcript)) -> [u8; 16] {
    let mut t = reference_run();
    build(&mut t);
    let mut out = [0u8; 16];
    t.challenge_bytes(b"c", &mut out).unwrap();
    out
}

#[test]
fn framing_is_injective_across_label_and_data_boundaries() {
    let a = first_challenge(|t| t.absorb_bytes(b"a", b"bc").unwrap());
    let b = first_challenge(|t| t.absorb_bytes(b"ab", b"c").unwrap());
    let c = first_challenge(|t| {
        t.absorb_bytes(b"a", b"b").unwrap();
        t.absorb_bytes(b"", b"c").unwrap();
    });
    let d = first_challenge(|t| t.absorb_bytes(b"a", b"b").unwrap());
    let all = [a, b, c, d];
    for i in 0..4 {
        for j in (i + 1)..4 {
            assert_ne!(all[i], all[j], "{i} vs {j}");
        }
    }
}

#[test]
fn order_label_protocol_and_round_markers_all_matter() {
    let x = first_challenge(|t| {
        t.absorb_bytes(b"p", b"1").unwrap();
        t.absorb_bytes(b"q", b"2").unwrap();
    });
    let y = first_challenge(|t| {
        t.absorb_bytes(b"q", b"2").unwrap();
        t.absorb_bytes(b"p", b"1").unwrap();
    });
    assert_ne!(x, y);
    let other_protocol = {
        let mut t = Transcript::new(b"test-protocol-2").unwrap();
        let mut out = [0u8; 16];
        t.challenge_bytes(b"c", &mut out).unwrap();
        out
    };
    assert_ne!(other_protocol, first_challenge(|_| {}));
    let with_round = first_challenge(|t| t.begin_round(b"r").unwrap());
    assert_ne!(with_round, first_challenge(|_| {}));
    // the challenge label is part of the derivation
    let mut t = reference_run();
    let mut o = [0u8; 16];
    t.challenge_bytes(b"other", &mut o).unwrap();
    assert_ne!(o, first_challenge(|_| {}));
}

#[test]
fn challenges_ratchet_and_clone_replays_identically() {
    let mut t = reference_run();
    let (mut a, mut b) = ([0u8; 16], [0u8; 16]);
    t.challenge_bytes(b"c", &mut a).unwrap();
    t.challenge_bytes(b"c", &mut b).unwrap(); // same label, later in the run
    assert_ne!(a, b);
    let mut t1 = reference_run();
    t1.absorb_bytes(b"m", b"x").unwrap();
    let mut t2 = t1.clone();
    let (mut u, mut v) = ([0u8; 32], [0u8; 32]);
    t1.challenge_bytes(b"c", &mut u).unwrap();
    t2.challenge_bytes(b"c", &mut v).unwrap();
    assert_eq!(u, v);
    // a longer request is a prefix-consistent extension of a shorter one
    let (mut s, mut l) = ([0u8; 8], [0u8; 24]);
    reference_run().challenge_bytes(b"c", &mut s).unwrap();
    reference_run().challenge_bytes(b"c", &mut l).unwrap();
    assert_eq!(s, l[..8]);
}

#[test]
fn sampling_is_in_range_and_close_to_uniform() {
    let mut t = reference_run();
    let mut counts = [0u32; 5];
    for _ in 0..20_000 {
        let x = t.challenge_field::<5>(b"f").unwrap().to_canonical();
        counts[x as usize] += 1;
    }
    for c in counts {
        assert!((3_600..=4_400).contains(&c), "counts {counts:?}"); // 4000 +- ~10% (> 6 sigma)
    }
    let mut counts = [0u32; 3];
    for _ in 0..12_000 {
        counts[t.challenge_below(b"b", 3).unwrap() as usize] += 1;
    }
    for c in counts {
        assert!((3_600..=4_400).contains(&c), "counts {counts:?}");
    }
    assert_eq!(t.challenge_below(b"one", 1).unwrap(), 0);
    assert!(t.challenge_below(b"zero", 0).is_err());
    // very large bounds work and stay in range
    for _ in 0..100 {
        assert!(t.challenge_below(b"big", u64::MAX).unwrap() < u64::MAX);
        assert!(t.challenge_below(b"pow", 1 << 63).unwrap() < (1 << 63));
    }
}

#[test]
fn ledger_counts_everything() {
    let mut t = reference_run();
    assert_eq!(t.ledger().permutations, 0);
    t.absorb_bytes(b"m", &[0u8; 10]).unwrap();
    t.begin_round(b"r").unwrap();
    t.begin_round(b"r").unwrap();
    let mut o = [0u8; 8];
    t.challenge_bytes(b"c", &mut o).unwrap();
    t.challenge_below(b"c", 10).unwrap();
    let l = t.ledger();
    assert_eq!(
        (l.rounds, l.absorbs, l.challenges, l.absorbed_bytes),
        (2, 1, 2, 10)
    );
    assert!(l.permutations >= 4, "{l:?}"); // >= one finalize per challenge, per clone
                                           // a 1000-byte message forces at least 7 absorb permutations on the main state
    let before = t.ledger().permutations;
    t.absorb_bytes(b"big", &[1u8; 1000]).unwrap();
    assert!(t.ledger().permutations >= before + 7);
}
