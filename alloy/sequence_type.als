module sequence_type

open common

-- Compact sequence header: 0x30–0x3F (0b0011XXXX).
-- Bits 7–6 = Zero (type prefix), b5 = One (sequence type),
-- b4 = One (compact variant), bits 3–0 = element count in {0..15}.
sig CompactSeqHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = One
    b4 = One
    -- b3..b0 encode element count (0..15) — free
}

-- Extended sequence header: 0x20–0x27 (0b00100XXX).
-- Bits 7–6 = Zero, b5 = One (sequence type), b4 = Zero (extended),
-- b3 = Zero (reserved, must be Zero),
-- bits 2–0 encode (countFieldWidth − 1) in {0..7}.
sig ExtendedSeqHeader extends Header {} {
    b7 = Zero
    b6 = Zero
    b5 = One
    b4 = Zero
    b3 = Zero -- reserved
    -- b2..b0 encode (countFieldWidth − 1) — free
}

run showCompactSeq { some CompactSeqHeader } for 2 expect 1
run showExtendedSeq { some ExtendedSeqHeader } for 2 expect 1

-- Reserved bit on extended header is always Zero.
assert ExtendedSeqReservedZero {
    all h: ExtendedSeqHeader | h.b3 = Zero
}

-- Compact and extended differ in b4 (One vs Zero).
assert CompactExtendedSeqDisjoint {
    no h1: CompactSeqHeader, h2: ExtendedSeqHeader | samePattern[h1, h2]
}

check ExtendedSeqReservedZero for 4 expect 0
check CompactExtendedSeqDisjoint for 4 expect 0
