module bytes_type

open common

-- Bytes header: 0x04–0x07 (0b000001XX).
-- Bits 7–3 = Zero (type prefix), b2 = One (bytes type flag),
-- bits 1–0 encode the length-field width exponent e ∈ {0..3},
-- so the <INTEGER> length field is 2^e bytes wide (1, 2, 4, or 8 bytes).
sig BytesHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = Zero
    b3 = Zero
    b2 = One
    -- b1, b0 are free: they encode the length-field width exponent (0..3)
}

run showBytes { some BytesHeader } for 2 expect 1

-- The type prefix bits are fixed.
assert BytesTypeBitsFixed {
    all h: BytesHeader |
        h.b7=Zero and h.b6=Zero and h.b5=Zero and
        h.b4=Zero and h.b3=Zero and h.b2=One
}
check BytesTypeBitsFixed for 4 expect 0
