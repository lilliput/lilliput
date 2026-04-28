module value

open common
open null_type
open unit_type
open bool_type
open float_type
open bytes_type
open string_type
open integer_type
open sequence_type
open map_type

-- =========================================================================
-- Value type hierarchy
-- =========================================================================
-- Each Value variant holds one header that encodes its wire representation.
-- Sequence and Map reference the abstract Value sig for their elements,
-- giving the flat recursive structure bounded by Alloy's scope.

abstract sig Value {}

sig NullValue extends Value { header: one NullHeader }
sig UnitValue extends Value { header: one UnitHeader }
sig BoolValue extends Value { header: one BoolHeader }
sig FloatValue extends Value { header: one FloatHeader }
sig BytesValue extends Value { header: one BytesHeader }

-- String and Integer each have two wire variants; both are absorbed into
-- a single semantic Value kind.
sig StringValue extends Value { header: one (CompactStringHeader + ExtendedStringHeader) }
sig IntValue extends Value { header: one (CompactUnsignedIntHeader + CompactSignedIntHeader
                                            + ExtendedUnsignedIntHeader + ExtendedSignedIntHeader) }

-- Sequence: an ordered collection of arbitrary Values.
sig SeqValue extends Value {
    seqHeader: one (CompactSeqHeader + ExtendedSeqHeader),
    elements: set Value -- element order is abstracted away
}

-- Map: a key-value collection where both keys and values are arbitrary Values.
sig MapValue extends Value {
    mapHeader: one (CompactMapHeader + ExtendedMapHeader),
    entries: Value -> lone Value -- each key maps to at most one value
}

-- =========================================================================
-- Type-tag enumeration
-- =========================================================================
-- One atom per valid wire type, plus TInvalid for the header bytes that
-- carry reserved-bit violations.

abstract sig TypeTag {}
one sig TNull, TUnit, TBool, TBytes, TFloat,
        TMapExtended, TMapCompact,
        TSeqExtended, TSeqCompact,
        TStringExtended, TStringCompact,
        TIntExtUnsigned, TIntExtSigned,
        TIntCmpUnsigned, TIntCmpSigned,
        TInvalid
        extends TypeTag {}

-- =========================================================================
-- Complete decode trie
-- =========================================================================
-- typeOf implements the decoder's type-dispatch logic as a total function
-- over all 256 possible header byte patterns.
-- Reading from bit 7 (MSB) down to bit 0, each branch follows the actual
-- implementation constants in lilliput-core/src/header/*.rs.

fun typeOf[h: Header]: TypeTag {
    (h.b7 = One) =>
        (h.b6 = One) =>
            (h.b5 = One) => TIntCmpSigned
            else TIntCmpUnsigned
        else
            -- Extended integer: reserved bits b4 and b3 must be Zero
            (h.b4 = One or h.b3 = One) => TInvalid
            else
                (h.b5 = One) => TIntExtSigned
                else TIntExtUnsigned
    else
    (h.b6 = One) =>
        (h.b5 = One) => TStringCompact
        else
            -- Extended string: reserved bits b4 and b3 must be Zero
            (h.b4 = One or h.b3 = One) => TInvalid
            else TStringExtended
    else
    (h.b5 = One) =>
        (h.b4 = One) => TSeqCompact
        else
            -- Extended sequence: reserved bit b3 must be Zero
            (h.b3 = One) => TInvalid
            else TSeqExtended
    else
    (h.b4 = One) =>
        (h.b3 = One) => TMapCompact
        else TMapExtended
    else
    (h.b3 = One) => TFloat
    else
    (h.b2 = One) => TBytes
    else
    (h.b1 = One) => TBool
    else
    (h.b0 = One) => TUnit
    else TNull
}

-- =========================================================================
-- Soundness assertions
-- =========================================================================

-- (1) Disjoint opcodes:
-- Every header byte maps to exactly one TypeTag.
-- typeOf is a total function by construction, so the singleton check
-- verifies no branch returns an empty or multi-element set.
assert DisjointOpcodes {
    all h: Header | one typeOf[h]
}

-- (2) Consistency: typed-sig constraints agree with the decode trie.
-- If a header atom belongs to a typed sig, typeOf must return the
-- corresponding tag — verifying that the sig bit-constraints and the
-- trie are mutually consistent.
assert TypeOfConsistentWithSigs {
    all h: NullHeader | typeOf[h] = TNull
    all h: UnitHeader | typeOf[h] = TUnit
    all h: BoolHeader | typeOf[h] = TBool
    all h: BytesHeader | typeOf[h] = TBytes
    all h: FloatHeader | typeOf[h] = TFloat
    all h: CompactMapHeader | typeOf[h] = TMapCompact
    all h: ExtendedMapHeader | typeOf[h] = TMapExtended
    all h: CompactSeqHeader | typeOf[h] = TSeqCompact
    all h: ExtendedSeqHeader | typeOf[h] = TSeqExtended
    all h: CompactStringHeader | typeOf[h] = TStringCompact
    all h: ExtendedStringHeader | typeOf[h] = TStringExtended
    all h: CompactUnsignedIntHeader | typeOf[h] = TIntCmpUnsigned
    all h: CompactSignedIntHeader | typeOf[h] = TIntCmpSigned
    all h: ExtendedUnsignedIntHeader | typeOf[h] = TIntExtUnsigned
    all h: ExtendedSignedIntHeader | typeOf[h] = TIntExtSigned
}

-- (3) Reserved-byte invalidity:
-- No atom belonging to any typed sig is classified as TInvalid.
-- This confirms no valid header pattern accidentally lands in the
-- "reserved bits set" zones.
assert NoTypedHeaderIsInvalid {
    no h: Header |
        typeOf[h] = TInvalid and (
            h in NullHeader or h in UnitHeader or h in BoolHeader or
            h in BytesHeader or h in FloatHeader or
            h in CompactMapHeader or h in ExtendedMapHeader or
            h in CompactSeqHeader or h in ExtendedSeqHeader or
            h in CompactStringHeader or h in ExtendedStringHeader or
            h in CompactUnsignedIntHeader or h in CompactSignedIntHeader or
            h in ExtendedUnsignedIntHeader or h in ExtendedSignedIntHeader
        )
}

-- (4) Coverage: every header byte is either a valid type or explicitly invalid.
-- (There are no "unknown" byte patterns — TInvalid absorbs all gaps.)
assert FullCoverage {
    all h: Header | typeOf[h] != none
}

-- (5) Unique decoding (no lookahead):
-- Streaming decoders must identify the wire type from the header byte
-- alone. This asserts that typeOf is determined solely by the byte's
-- bit pattern — two headers with identical bits cannot disagree about
-- their classification, so context-free single-byte dispatch is sound.
assert UniqueDecoding {
    all h1, h2: Header | samePattern[h1, h2] => typeOf[h1] = typeOf[h2]
}

-- (6) Prefix-free dispatch (single-byte partition):
-- The typed sigs are pairwise disjoint and, together with the TInvalid
-- byte patterns, partition the entire 256-byte header space. No header
-- can satisfy two distinct typed-sig constraints simultaneously, which
-- is precisely what allows the decoder to commit to a wire type after
-- reading exactly one byte. Stated bidirectionally: a header belongs
-- to a typed sig iff typeOf returns the corresponding tag.
assert PrefixFreeDispatch {
    all h: Header | h in NullHeader <=> typeOf[h] = TNull
    all h: Header | h in UnitHeader <=> typeOf[h] = TUnit
    all h: Header | h in BoolHeader <=> typeOf[h] = TBool
    all h: Header | h in BytesHeader <=> typeOf[h] = TBytes
    all h: Header | h in FloatHeader <=> typeOf[h] = TFloat
    all h: Header | h in CompactMapHeader <=> typeOf[h] = TMapCompact
    all h: Header | h in ExtendedMapHeader <=> typeOf[h] = TMapExtended
    all h: Header | h in CompactSeqHeader <=> typeOf[h] = TSeqCompact
    all h: Header | h in ExtendedSeqHeader <=> typeOf[h] = TSeqExtended
    all h: Header | h in CompactStringHeader <=> typeOf[h] = TStringCompact
    all h: Header | h in ExtendedStringHeader <=> typeOf[h] = TStringExtended
    all h: Header | h in CompactUnsignedIntHeader <=> typeOf[h] = TIntCmpUnsigned
    all h: Header | h in CompactSignedIntHeader <=> typeOf[h] = TIntCmpSigned
    all h: Header | h in ExtendedUnsignedIntHeader <=> typeOf[h] = TIntExtUnsigned
    all h: Header | h in ExtendedSignedIntHeader <=> typeOf[h] = TIntExtSigned
}

-- =========================================================================
-- Run / check commands
-- =========================================================================

run showValue { some SeqValue and some MapValue } for 4 expect 1
run showNestedSeq { some s: SeqValue | some s.elements } for 4 expect 1

check DisjointOpcodes for 4 expect 0
check TypeOfConsistentWithSigs for 4 expect 0
check NoTypedHeaderIsInvalid for 4 expect 0
check FullCoverage for 4 expect 0
check UniqueDecoding for 4 expect 0
check PrefixFreeDispatch for 4 expect 0
