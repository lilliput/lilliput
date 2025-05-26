# Integer

Represents an integer value in 1, 2, 3, 4, 5, 6, 7, or 8 bytes.

## Signed

Values are represented as network-endian, bit-packed, zig-zag-encoded signed integers:

```plain
// pseudo-code

function pack(unpacked: signed int) -> bits {
    return bit_pack(zig_zag_encode(unpacked));
}

function unpack(packed: bits) -> signed int {
    return zig_zag_decode(bit_unpack(packed));
}
```

## Unsigned

Values are represented as network-endian, bit-packed, unsigned integers:

```plain
// pseudo-code

function pack(unpacked: unsigned int) -> bits {
    return bit_pack(unpacked);
}

function unpack(packed: bits) -> unsigned int {
    return bit_unpack(packed);
}
```

## Binary representation

Integers can be represented as two variants: compact, and extended.

```plain
0b1XYZZZZZ <INTEGER>?
  │││├───┘ ├────────┘
  ││││     Value
  │││└─ <depends on variant>
  ││└─ Signedness
  │└─ Variant
  └─ Integer type
```

where

- `X` is a single bit that specifies the variant (`1` = compact, `0` = extended).
- `Y` is a single bit that specifies the signedness (`1` = signed, `0` = unsigned).
- `ZZZZZ` is a 5-bit variant-specific binary value.

> ⚠️ An integer value MAY be encoded in the compact variant, if it fits the compact value range, but it is NOT REQUIRED.

### Compact representation

In the compact variant the entire value is represented by the type header itself.

```plain
0b11XYYYYY
  │││├───┘
  │││└─ Value
  ││└─ Signedness
  │└─ Compact variant
  └─ Integer type
```

where

- `X` is a single bit that specifies the signedness (`1` = signed, `0` = unsigned).
- `YYYYY` is a 5-bit integer which represents the integer value.
  - signed: a bit-packed, zig-zag-encoded integer value.
  - unsigned: a bit-packed integer value.

#### Signed representation

Signed integer values can be represented as compact variant, as long as they fall within the range of `[-(2^4), (2^4)-1]` (i.e. `[-16, 15]`).

#### Unsigned representation

Signed integer values can be represented as compact variant, as long as they fall within the range of `[0, (2^5)-1]` (i.e. `[0, 31]`).

### Extended representation

```plain
0b10X00YYY <INTEGER>
  │││├┘├─┘ ├───────┘
  ││││ │   └─ Value
  ││││ └─ Width in bytes, minus 1
  │││└─ Reserved bits
  ││└─ Signedness
  │└─ Extended variant
  └─ Integer type
```

where

- `X` is a single bit that specifies the signedness (`1` = signed, `0` = unsigned).
- `YYY` is a 3-bit integer which represents the network-endian, bit-packed number of bytes required to represent the value, subtracted by `1`.
- `<INTEGER>` is a variable-byte integer which represents the bit-packed value in network-endian, padded up to the next full byte.
