"""Small, dependency-free audit for aQiten ring candidates.

Research tooling only. Trial-division primality is suitable only for small
candidate values; this is not cryptographic parameter validation.
"""
from __future__ import annotations
from dataclasses import dataclass
from typing import Sequence


def is_prime_trial(n: int) -> bool:
    if n < 2:
        return False
    if n % 2 == 0:
        return n == 2
    divisor = 3
    while divisor * divisor <= n:
        if n % divisor == 0:
            return False
        divisor += 2
    return True


def exact_order_power_of_two_root(q: int, d: int) -> int | None:
    """Return omega of exact order 2d, or None if a full negacyclic NTT is absent."""
    if not is_prime_trial(q) or d < 1:
        return None
    order = 2 * d
    if order & (order - 1):
        raise ValueError("2*d must be a power of two")
    if (q - 1) % order:
        return None
    exponent = (q - 1) // order
    for base in range(2, q):
        omega = pow(base, exponent, q)
        if pow(omega, d, q) == q - 1 and pow(omega, order, q) == 1:
            return omega
    return None


def negacyclic_mul(a: Sequence[int], b: Sequence[int], q: int, d: int) -> tuple[int, ...]:
    """Multiply degree-<d representatives in F_q[X]/(X^d+1)."""
    if q < 2 or d < 1 or len(a) != d or len(b) != d:
        raise ValueError("require q >= 2, d >= 1 and exactly d coefficients")
    acc = [0] * (2 * d - 1)
    for i, ai in enumerate(a):
        for j, bj in enumerate(b):
            acc[i + j] += ai * bj
    for k in range(d, 2 * d - 1):
        acc[k - d] -= acc[k]
    return tuple(x % q for x in acc[:d])


@dataclass(frozen=True)
class RingAudit:
    q: int
    d: int
    prime: bool
    q_mod_8_is_5: bool
    ntt_root: int | None

    @property
    def has_full_negacyclic_ntt(self) -> bool:
        return self.ntt_root is not None


def audit(q: int, d: int) -> RingAudit:
    prime = is_prime_trial(q)
    root = exact_order_power_of_two_root(q, d) if prime else None
    return RingAudit(q, d, prime, q % 8 == 5, root)
