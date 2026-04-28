module map_type

open common

-- Compact map header: 0x18–0x1F (0b00011XXX).
-- Bits 7–5 = Zero (type prefix), b4 = One (map type),
-- b3 = One (compact variant), bits 2–0 = entry count in {0..7}.
sig CompactMapHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = One
    b3 = One
    -- b2..b0 encode entry count (0..7) — free
}

-- Extended map header: 0x10–0x17 (0b00010XXX).
-- Bits 7–5 = Zero, b4 = One (map type), b3 = Zero (extended),
-- bits 2–0 encode (countFieldWidth − 1) in {0..7}.
-- No reserved bits: all lower 3 bits carry the width encoding.
sig ExtendedMapHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = Zero
    b4 = One
    b3 = Zero
    -- b2..b0 encode (countFieldWidth − 1) — free
}

run showCompactMap { some CompactMapHeader } for 2 expect 1
run showExtendedMap { some ExtendedMapHeader } for 2 expect 1

-- Compact and extended differ in b3 (One vs Zero).
assert CompactExtendedMapDisjoint {
    no h1: CompactMapHeader, h2: ExtendedMapHeader | samePattern[h1, h2]
}

check CompactExtendedMapDisjoint for 4 expect 0
