import unittest
from research.ring_profile import audit, negacyclic_mul, exact_order_power_of_two_root


class RingProfileTests(unittest.TestCase):
    def test_q_5_mod_8_cannot_have_full_ntt_at_power_two_degree_ge_4(self):
        for q in (13, 29, 37, 53, 61, 101):
            if q % 8 == 5:
                for d in (4, 8, 16):
                    self.assertIsNone(exact_order_power_of_two_root(q, d))

    def test_valid_full_negacyclic_ntt_certificate(self):
        omega = exact_order_power_of_two_root(41, 4)
        self.assertIsNotNone(omega)
        self.assertEqual(pow(omega, 4, 41), 40)
        self.assertEqual(pow(omega, 8, 41), 1)

    def test_reference_negacyclic_wrap_sign(self):
        self.assertEqual(negacyclic_mul((0, 0, 0, 1), (0, 1, 0, 0), 41, 4),
                         (40, 0, 0, 0))

    def test_reference_multiplication(self):
        self.assertEqual(negacyclic_mul((1, 2, 0, 0), (3, 4, 0, 0), 41, 4),
                         (3, 10, 8, 0))

    def test_nonprime_is_rejected(self):
        self.assertFalse(audit(45, 4).prime)


if __name__ == "__main__":
    unittest.main()
