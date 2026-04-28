module null_type

open common

-- Null: header byte 0x00 (0b00000000).
-- All eight bits are fixed to Zero; no variant fields.
sig NullHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = Zero
    b3 = Zero
    b2 = Zero
    b1 = Zero
    b0 = Zero
}

-- All Null headers share the same bit pattern.
assert NullPatternIsUniform {
    all h1, h2: NullHeader | samePattern[h1, h2]
}

run showNull { some NullHeader } for 2 expect 1
check NullPatternIsUniform for 4 expect 0
