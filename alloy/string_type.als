module string_type

open common

-- Compact string header: 0x60–0x7F (0b011XXXXX).
-- b7 = Zero, b6 = One (string type), b5 = One (compact variant set),
-- bits 4–0 encode the string's byte-length in {0..31}.
sig CompactStringHeader extends Header {} {
    b7 = Zero
    b6 = One
    b5 = One
    -- b4..b0 encode byte-length (0..31) — free
}

-- Extended string header: 0x40–0x47 (0b01000XXX).
-- b7 = Zero, b6 = One (string type), b5 = Zero (compact variant not set),
-- b4 = Zero and b3 = Zero (reserved, must be Zero),
-- bits 2–0 encode (lengthFieldWidth − 1) in {0..7}.
sig ExtendedStringHeader extends Header {} {
    b7 = Zero
    b6 = One
    b5 = Zero
    b4 = Zero -- reserved
    b3 = Zero -- reserved
    -- b2..b0 encode (lengthFieldWidth − 1) — free
}

run showCompactString { some CompactStringHeader } for 2 expect 1
run showExtendedString { some ExtendedStringHeader } for 2 expect 1

-- Reserved bits on extended headers are always Zero.
assert ExtendedStringReservedZero {
    all h: ExtendedStringHeader | h.b4 = Zero and h.b3 = Zero
}

-- Compact and extended headers cannot share a bit pattern (b5 differs).
assert CompactExtendedStringDisjoint {
    no h1: CompactStringHeader, h2: ExtendedStringHeader | samePattern[h1, h2]
}

check ExtendedStringReservedZero for 4 expect 0
check CompactExtendedStringDisjoint for 4 expect 0
