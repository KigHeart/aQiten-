#!/usr/bin/env python3
"""Independent model of aqiten-transcript (hashlib SHAKE256). Prints the known answers
embedded in crates/aqiten-transcript/tests/known_answers.rs. Shares no code with the crate."""
import hashlib, struct

def frame(tag, label, data):
    return bytes([tag]) + struct.pack("<I", len(label)) + label + struct.pack("<Q", len(data)) + data

class T:
    def __init__(self, proto):
        self.s = hashlib.shake_256(frame(1, b"aqiten.transcript.v1", proto))
        self.rounds = 0; self.chal = 0
    def absorb(self, label, data): self.s.update(frame(2, label, data))
    def round(self, label):
        self.s.update(frame(4, label, struct.pack("<Q", self.rounds))); self.rounds += 1
    def stream(self, label):
        c = self.s.copy(); c.update(frame(3, label, struct.pack("<Q", self.chal)))
        buf = c.digest(1 << 16); self.chal += 1
        self.s.update(frame(5, label, buf[:32]))
        return iter(buf[32:])
    def bytes(self, label, n):
        it = self.stream(label); return bytes(next(it) for _ in range(n))
    def below(self, label, bound):
        it = self.stream(label); return sample_below(it, bound)

def sample_below(it, bound):
    bits = (bound - 1).bit_length(); mask = (1 << bits) - 1
    for _ in range(128):
        v = int.from_bytes(bytes(next(it) for _ in range(8)), "little") & mask
        if v < bound: return v
    raise RuntimeError

def oracle_field(seed, label, coords, p):
    s = hashlib.shake_256(frame(6, label, seed))
    s.update(struct.pack("<I", len(coords)) + b"".join(struct.pack("<Q", c) for c in coords))
    return sample_below(iter(s.digest(1 << 12)), p)

if __name__ == "__main__":
    for n in (1, 135, 136, 137, 272, 1000):
        m = bytes((i * 7 + 3) % 256 for i in range(n))
        print("shake", n, hashlib.shake_256(m).hexdigest(40))
    t = T(b"test-protocol")
    t.absorb(b"msg", b"hello")
    t.round(b"r")
    a = t.bytes(b"c1", 16)
    t.absorb(b"msg2", b"")
    b = t.below(b"c2", 1000)
    p = 4294967197
    f = t.below(b"c3", p)
    d = t.bytes(b"c4", 8)
    print("c1", a.hex()); print("c2", b); print("c3", f); print("c4", d.hex())
    print("oracle", oracle_field(b"seed", b"A", [3, 7], p), oracle_field(b"seed", b"A", [7, 3], p),
          oracle_field(b"seed", b"B", [3, 7], p), oracle_field(b"seed2", b"A", [3, 7], p), oracle_field(b"seed", b"A", [], p))
