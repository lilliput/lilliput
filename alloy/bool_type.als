module bool_type

open common

-- Bool: header byte 0x02 (false) or 0x03 (true).
-- Bits 7–2 are Zero; b1 = One is the type discriminator; b0 carries the value.
sig BoolHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = Zero
    b3 = Zero
    b2 = Zero
    b1 = One
    -- b0: Zero = false, One = true (free)
}

-- Two Bool headers with different b0 values have different bit patterns.
assert BoolPatternsDistinct {
    all h1, h2: BoolHeader | h1.b0 != h2.b0 implies not samePattern[h1, h2]
}

run showFalse { some h: BoolHeader | h.b0 = Zero } for 2 expect 1
run showTrue { some h: BoolHeader | h.b0 = One } for 2 expect 1
check BoolPatternsDistinct for 4 expect 0
