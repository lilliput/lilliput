module common

-- A single bit.
abstract sig Bit {}
one sig Zero, One extends Bit {}

-- An 8-bit wire-format header byte.
-- Each field corresponds to one bit position (b7 = MSB, b0 = LSB).
sig Header {
    b7, b6, b5, b4, b3, b2, b1, b0: one Bit
}

-- True when two headers carry the same bit pattern.
pred samePattern[h1, h2: Header] {
    h1.b7 = h2.b7
    h1.b6 = h2.b6
    h1.b5 = h2.b5
    h1.b4 = h2.b4
    h1.b3 = h2.b3
    h1.b2 = h2.b2
    h1.b1 = h2.b1
    h1.b0 = h2.b0
}
