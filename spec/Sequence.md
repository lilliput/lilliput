# Sequence

Represents a sequence of lilliput-encoded values.

## Binary representation

```plain
0b001XYYYY <INTEGER>? <ENCODED>*
  ├─┘│├──┘ ├───────┘  ├────────┘
  │  ││    └─ Length? └─ Values*
  │  │└─ <depends on variant>
  │  └─ Variant
  └─ Sequence type
```

where

- `X` is a single bit that specifies the variant (`1` = compact, `0` = extended).
- `YYYY` is a 4-bit variant-specific binary value.

> ⚠️ An integer value MAY be encoded in the compact variant, if it fits the compact value range, but it is NOT REQUIRED.

### Compact representation

```plain
0b0011XXXX <ENCODED>*
  ├─┘│├──┘ ├────────┘
  │  ││    └─ Values
  │  │└─ Number of elements
  │  └─ Compact variant
  └─ Sequence type
```

where

- `XXXX` is a 4-bit unsigned integer which represents the number of items in the sequence, if within the range of `[0, (2^4)-1]`.
- `<ENCODED>*` is a variable-length sequence of lilliput-encoded values, representing the items of the sequence value.

### Extended representation

```plain
0b00100XXX <INTEGER> <ENCODED>*
  ├─┘││├─┘ ├───────┘ ├────────┘
  │  │││   └─ Length └─ Values
  │  ││└─ Width of length in bytes
  │  │└─ Reserved bit
  │  └─ Extended variant
  └─ Sequence type
```

where

- `XXX` is a 3-bit unsigned integer which represents the network-endian, bit-packed number of bytes required to represent the value, subtracted by `1`.
- `<INTEGER>` is a byte-packed unsigned integer, representing the sequence's length (i.e. its number of items).
- `<ENCODED>*` is a variable-length sequence of lilliput-encoded values, representing the items of the sequence value.
