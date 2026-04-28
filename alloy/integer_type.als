module integer_type

open common

-- Compact unsigned integer: 0xC0–0xDF (0b110XXXXX).
-- b7 = One (integer type), b6 = One (compact variant), b5 = Zero (unsigned),
-- bits 4–0 encode the packed unsigned value in {0..31}.
sig CompactUnsignedIntHeader extends Header {} {
    b7 = One
    b6 = One
    b5 = Zero
    -- b4..b0 encode the packed unsigned value — free
}

-- Compact signed integer: 0xE0–0xFF (0b111XXXXX).
-- b7 = One, b6 = One (compact), b5 = One (signed),
-- bits 4–0 encode the zig-zag-encoded value in {0..31}.
sig CompactSignedIntHeader extends Header {} {
    b7 = One
    b6 = One
    b5 = One
    -- b4..b0 encode the zig-zag packed value — free
}

-- Extended unsigned integer: 0x80–0x87 (0b10000XXX).
-- b7 = One, b6 = Zero (extended), b5 = Zero (unsigned),
-- b4 = Zero and b3 = Zero (reserved, must be Zero),
-- bits 2–0 encode (byteWidth − 1) in {0..7}.
sig ExtendedUnsignedIntHeader extends Header {} {
    b7 = One
    b6 = Zero
    b5 = Zero
    b4 = Zero -- reserved
    b3 = Zero -- reserved
    -- b2..b0 encode (byteWidth − 1) — free
}

-- Extended signed integer: 0xA0–0xA7 (0b10100XXX).
-- b7 = One, b6 = Zero (extended), b5 = One (signed),
-- b4 = Zero and b3 = Zero (reserved, must be Zero),
-- bits 2–0 encode (byteWidth − 1) in {0..7}.
sig ExtendedSignedIntHeader extends Header {} {
    b7 = One
    b6 = Zero
    b5 = One
    b4 = Zero -- reserved
    b3 = Zero -- reserved
    -- b2..b0 encode (byteWidth − 1) — free
}

run showCompactUnsigned { some CompactUnsignedIntHeader } for 2 expect 1
run showCompactSigned { some CompactSignedIntHeader } for 2 expect 1
run showExtendedUnsigned { some ExtendedUnsignedIntHeader } for 2 expect 1
run showExtendedSigned { some ExtendedSignedIntHeader } for 2 expect 1

-- Reserved bits on extended headers are always Zero.
assert ExtendedUnsignedIntReservedZero {
    all h: ExtendedUnsignedIntHeader | h.b4 = Zero and h.b3 = Zero
}
assert ExtendedSignedIntReservedZero {
    all h: ExtendedSignedIntHeader | h.b4 = Zero and h.b3 = Zero
}

-- Compact and extended share b7=One but differ in b6 (One vs Zero).
assert CompactExtendedIntDisjoint {
    no h1: CompactUnsignedIntHeader, h2: ExtendedUnsignedIntHeader | samePattern[h1, h2]
    no h1: CompactSignedIntHeader, h2: ExtendedSignedIntHeader | samePattern[h1, h2]
}

-- Signed and unsigned compact variants differ in b5.
assert CompactSignedUnsignedDisjoint {
    no h1: CompactUnsignedIntHeader, h2: CompactSignedIntHeader | samePattern[h1, h2]
}

-- Signed and unsigned extended variants differ in b5.
assert ExtendedSignedUnsignedDisjoint {
    no h1: ExtendedUnsignedIntHeader, h2: ExtendedSignedIntHeader | samePattern[h1, h2]
}

check ExtendedUnsignedIntReservedZero for 4 expect 0
check ExtendedSignedIntReservedZero for 4 expect 0
check CompactExtendedIntDisjoint for 4 expect 0
check CompactSignedUnsignedDisjoint for 4 expect 0
check ExtendedSignedUnsignedDisjoint for 4 expect 0
