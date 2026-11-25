"""Exhaustive finite-domain checks for the box-rejection lemma in TR-TEG-2026-AQT-002.

Run:
    python -m unittest research.test_box_rejection -v

This is a mathematical test harness, not cryptographic code. It checks the
finite counting identity and does not establish transcript-level zero knowledge.
"""
from collections import Counter
from fractions import Fraction
from itertools import product
import unittest


def accepted_outputs(z, bound, mask_radius):
    """Return accepted output counts for fixed z in the integer box."""
    if mask_radius < bound:
        raise ValueError("mask_radius must be >= bound")
    if any(abs(v) > bound for v in z):
        raise ValueError("z lies outside the declared bound")

    counts = Counter()
    mask_values = range(-mask_radius, mask_radius + 1)
    for y in product(mask_values, repeat=len(z)):
        out = tuple(a + b for a, b in zip(z, y))
        if all(abs(v) <= mask_radius - bound for v in out):
            counts[out] += 1
    return counts


def claimed_acceptance_probability(dimension, bound, mask_radius):
    numerator = 2 * (mask_radius - bound) + 1
    denominator = 2 * mask_radius + 1
    return Fraction(numerator, denominator) ** dimension


class BoxRejectionTests(unittest.TestCase):
    def test_exact_probability_and_uniformity_for_all_small_witnesses(self):
        for dimension in range(1, 4):
            for bound in range(0, 3):
                for radius in range(max(1, bound), bound + 4):
                    expected = claimed_acceptance_probability(
                        dimension, bound, radius
                    )
                    support_size = (2 * (radius - bound) + 1) ** dimension
                    expected_count_per_output = 1

                    for z in product(range(-bound, bound + 1), repeat=dimension):
                        counts = accepted_outputs(z, bound, radius)
                        self.assertEqual(sum(counts.values()), round(
                            expected * (2 * radius + 1) ** dimension
                        ))
                        self.assertEqual(len(counts), support_size)
                        self.assertTrue(
                            all(v == expected_count_per_output
                                for v in counts.values())
                        )
                        self.assertEqual(
                            Fraction(sum(counts.values()),
                                     (2 * radius + 1) ** dimension),
                            expected,
                        )

    def test_bernoulli_lower_bound(self):
        for dimension in range(1, 8):
            for bound in range(0, 5):
                for radius in range(max(1, bound), bound + 10):
                    exact = claimed_acceptance_probability(
                        dimension, bound, radius
                    )
                    lower = Fraction(
                        1, 1
                    ) - Fraction(
                        2 * dimension * bound, 2 * radius + 1
                    )
                    self.assertGreaterEqual(exact, lower)

    def test_y_equals_two_n_b_gives_at_least_half(self):
        for dimension in range(1, 20):
            for bound in range(1, 10):
                radius = 2 * dimension * bound
                exact = claimed_acceptance_probability(
                    dimension, bound, radius
                )
                self.assertGreaterEqual(exact, Fraction(1, 2))

    def test_rejects_invalid_bounds(self):
        with self.assertRaises(ValueError):
            accepted_outputs((2,), 1, 3)
        with self.assertRaises(ValueError):
            accepted_outputs((0,), 1, 0)


if __name__ == "__main__":
    unittest.main()
