# Map

Represents a map of lilliput-encoded values.

## Binary representation

```plain
0b0001XXXX <INTEGER>? (<ENCODED>,<ENCODED>)*
  ├──┘│├─┘ ├───────┘  ├────────────────────┘
  │   ││   └─ Length? └─ Key-value pairs
  │   │└─ <depends on variant>
  │   └─ Variant
  └─ Map type
```

where

- `X` is a single bit that specifies the variant (`1` = compact, `0` = extended).
- `YYY` is a 3-bit variant-specific binary value.

> ⚠️ An integer value MAY be encoded in the compact variant, if it fits the compact value range, but it is NOT REQUIRED.

### Compact representation

```plain
0b00011XXX (<ENCODED>,<ENCODED>)*
  ├──┘│├─┘ ├────────────────────┘
  │   ││   └─ Key-value pairs
  │   │└─ Number of elements
  │   └─ Compact variant
  └─ Map type
```

where

- `XXX` is a 3-bit unsigned integer which represents the number of items in the map, if within the range of `[0, (2^3)-1]`.
- `<ENCODED>*` is a variable-length map of lilliput-encoded values, representing the items of the map value.

### Extended representation

```plain
0b00010XXX <INTEGER> (<ENCODED>,<ENCODED>)*
  ├──┘│├─┘ ├───────┘ ├────────────────────┘
  │   ││   └─ Length └─ Key-value pairs
  │   │└─ Number of bytes in length
  │   └─ Extended variant
  └─ Map type
```

where

- `XXX` is a 3-bit unsigned integer which represents the network-endian, bit-packed number of bytes required to represent the value, subtracted by `1`.
- `<INTEGER>` is a byte-packed unsigned integer, representing the map's length (i.e. its number of items).
- `<ENCODED>*` is a variable-length map of lilliput-encoded values, representing the items of the map value.
