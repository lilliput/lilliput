module unit_type

open common

-- Unit: header byte 0x01 (0b00000001).
-- b0 = One is the sole distinguishing bit; all others are Zero.
sig UnitHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = Zero
    b3 = Zero
    b2 = Zero
    b1 = Zero
    b0 = One
}

-- All Unit headers share the same bit pattern.
assert UnitPatternIsUniform {
    all h1, h2: UnitHeader | samePattern[h1, h2]
}

run showUnit { some UnitHeader } for 2 expect 1
check UnitPatternIsUniform for 4 expect 0
