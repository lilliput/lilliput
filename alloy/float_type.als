module float_type

open common

-- Float header: 0x08–0x0F (0b00001XXX).
-- Bits 7–4 = Zero (type prefix), b3 = One (float type flag),
-- bits 2–0 encode (byteWidth − 1) in {0..7}, giving byte widths 1–8.
sig FloatHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = Zero
    b3 = One
    -- b2, b1, b0 are free: they encode byteWidth−1 (0..7)
}

run showFloat { some FloatHeader } for 2 expect 1

-- The type prefix bits are fixed.
assert FloatTypeBitsFixed {
    all h: FloatHeader | h.b7=Zero and h.b6=Zero and h.b5=Zero and h.b4=Zero and h.b3=One
}
check FloatTypeBitsFixed for 4 expect 0

-- -------------------------------------------------------------------------
-- Float bit-field layout model
--
-- For each byte width w ∈ {1..8}, the IEEE-754-generalised layout is:
-- sign=1 exponent=E significand=S where 1 + E + S = w × 8
-- -------------------------------------------------------------------------

abstract sig FloatLayout {
    signBits: one Int,
    exponentBits: one Int,
    significandBits: one Int,
    totalBits: one Int
}

one sig FL8 extends FloatLayout {} { signBits=1 and exponentBits=4 and significandBits=3 and totalBits=8 }
one sig FL16 extends FloatLayout {} { signBits=1 and exponentBits=5 and significandBits=10 and totalBits=16 }
one sig FL24 extends FloatLayout {} { signBits=1 and exponentBits=7 and significandBits=16 and totalBits=24 }
one sig FL32 extends FloatLayout {} { signBits=1 and exponentBits=8 and significandBits=23 and totalBits=32 }
one sig FL40 extends FloatLayout {} { signBits=1 and exponentBits=8 and significandBits=31 and totalBits=40 }
one sig FL48 extends FloatLayout {} { signBits=1 and exponentBits=9 and significandBits=38 and totalBits=48 }
one sig FL56 extends FloatLayout {} { signBits=1 and exponentBits=10 and significandBits=45 and totalBits=56 }
one sig FL64 extends FloatLayout {} { signBits=1 and exponentBits=11 and significandBits=52 and totalBits=64 }

-- sign + exponent + significand = totalBits (the partition is exact).
assert FloatLayoutPartitionsExactly {
    all f: FloatLayout |
        f.signBits.plus[f.exponentBits].plus[f.significandBits] = f.totalBits
}

-- Sign field is always exactly 1 bit.
assert FloatSignIsOneBit {
    all f: FloatLayout | f.signBits = 1
}

-- Total width is a multiple of 8 and lies within [8, 64].
assert FloatTotalBitsIsWholeByte {
    all f: FloatLayout |
        f.totalBits >= 8 and f.totalBits <= 64 and f.totalBits.rem[8] = 0
}

-- Exponent grows monotonically with width (except 32-bit and 40-bit share exp=8).
assert FloatExponentNonDecreasing {
    FL8.exponentBits <= FL16.exponentBits and
    FL16.exponentBits <= FL24.exponentBits and
    FL24.exponentBits <= FL32.exponentBits and
    FL32.exponentBits <= FL40.exponentBits and
    FL40.exponentBits <= FL48.exponentBits and
    FL48.exponentBits <= FL56.exponentBits and
    FL56.exponentBits <= FL64.exponentBits
}

check FloatLayoutPartitionsExactly for 0 but 8 int expect 0
check FloatSignIsOneBit for 0 but 8 int expect 0
check FloatTotalBitsIsWholeByte for 0 but 8 int expect 0
check FloatExponentNonDecreasing for 0 but 8 int expect 0
