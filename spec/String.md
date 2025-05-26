# String

Represents a [UTF-8](https://en.wikipedia.org/wiki/UTF-8)-encoded textual value.

## Binary representation

```plain
0b01XYYYYY
  ├┘│├───┘
  │ │└─ <depends on variant>
  │ └─ Compact variant / Extended variant
  └─ String Type
```

where

- `X` is a single bit that specifies the variant (`1` = compact, `0` = extended).
- `YYYYY` is a 5-bit variant-specific binary value.

> ⚠️ An integer value MAY be encoded in the compact variant, if it fits the compact value range, but it is NOT REQUIRED.

### Compact representation

```plain
0b010XXXXX <BYTE>*
  ├┘│├───┘ ├─────┘
  │ ││     └─ Characters
  │ │└─ Length
  │ └─ Compact variant
  └─ String type
```

where

- `XXXXX` is a 5-bit unsigned integer which represents the byte length of the string, if within the range of `[0, (2^5)-1]`.
- `<BYTE>*` is a variable-length sequence of bytes, representing the string value.

### Extended representation

```plain
0b01100XXX <LENGTH> <BYTE>*
  ├┘│├┘├─┘ ├──────┘ ├─────┘
  │ ││ │   └─ Length └─ Characters
  │ ││ └─ Number of bytes in <Length> - 1
  │ │└─ Empty padding bits
  │ └─ Extended variant
  └─ String type
```

where

- `XXX` is a 3-bit unsigned integer which represents the network-endian, bit-packed number of bytes required to represent the value, subtracted by `1`.
- `<INTEGER>` is a byte-packed unsigned integer, representing the string's length.
- `<BYTE>*` is a variable-length sequence of bytes, representing the string value.
