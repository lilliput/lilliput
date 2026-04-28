# Bytes

Represents a byte array.

## Binary representation

```plain
0b000001XX <INTEGER> <BYTE>*
  ├────┘├┘ ├───────┘ ├─────┘
  │     │  │         └─ Bytes
  │     │  └─ Number of bytes
  │     └─ Length width exponent
  └─ Bytes type
```

where

- `XX` is a 2-bit unsigned integer (`e`) used as the exponent for the width of the `<INTEGER>` field: the field is `2^e` bytes wide (giving widths of 1, 2, 4, or 8 bytes).
- `<INTEGER>` is a `2^e`-byte unsigned integer representing the byte array's length (i.e. number of bytes).
- `<BYTE>*` is a variable-length sequence of bytes, representing the byte array's contents.
