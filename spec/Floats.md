# Float

Represents an floating-point value in 1, 2, 3, 4, 5, 6, 7, or 8 bytes.

## Binary representation

```plain
0b00001XXX <FLOAT>
  ├───┘├─┘ ├─────┘
  │    │   └─ Value
  │    └─ Width in bytes, subtracted by 1
  └─ Float Type
```

where

- `XXX` is a 3-bit integer which represents the network-endian, bit-packed number of bytes required to represent the value, subtracted by `1`.
- `<FLOAT>` is a bit-representation of the bit-packed value in network-endian, padded up to the next full byte.

## Float representation

Lilliput's float representations follow [IEEE-754](https://en.wikipedia.org/wiki/IEEE_754), but generalized to also cover 8-bit, 24-bit, 40-bit, 48-bit, and 56-bit values:

### 8-bit

A bit-level representation of a 8-bit floating-point number.

The bits are laid out as follows:

```plain
MSB   ...   LSB
┌─┬─┬─┬─┬─┬─┬─┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┘
 │ ├─────┘ ├───┘
 │ │       └ Significand (3 bits)
 │ └ Exponent (4 bits)
 └ Sign (1 bit)
```

### 16-bit

A bit-level representation of a 16-bit floating-point number.

The bits are laid out as follows:

```plain
MSB           ...           LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
 │ ├───────┘ ├─────────────────┘
 │ │         └ Significand (10 bits)
 │ └ Exponent (5 bits)
 └ Sign (1 bit)
```

### 24-bit

A bit-level representation of a 24-bit floating-point number.

The bits are laid out as follows:

```plain
MSB                      ...                     LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
 │ ├───────────┘ ├─────────────────────────────┘
 │ │             └ Significand (16 bits)
 │ └ Exponent (7 bits)
 └ Sign (1 bit)
```

### 32-bit

A bit-level representation of a 32-bit floating-point number.

The bits are laid out as follows:

```plain
MSB                              ...                             LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┘
 │ ├─────────────┘ ├───────────────────────────────────────────┘
 │ │               └ Significand (23 bits)
 │ └ Exponent (8 bits)
 └ Sign (1 bit)
```

### 40-bit

A bit-level representation of a 40-bit floating-point number.

The bits are laid out as follows:

```plain
MSB                              ...                             LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
 │ ├─────────────┘ ├───────────────────────────────────────╴╴╴─┘
 │ │               └ Significand (31 bits)
 │ └ Exponent (8 bits)
 └ Sign (1 bit)
```

### 48-bit

A bit-level representation of a 48-bit floating-point number.

The bits are laid out as follows:

```plain
MSB                              ...                             LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
 │ ├───────────────┘ ├─────────────────────────────────────╴╴╴─┘
 │ │                 └ Significand (38 bits)
 │ └ Exponent (9 bits)
 └ Sign (1 bit)
```

### 56-bit

A bit-level representation of a 56-bit floating-point number.

The bits are laid out as follows:

```plain
MSB                              ...                             LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
 │ ├─────────────────┘ ├───────────────────────────────────╴╴╴─┘
 │ │                   └ Significand (45 bits)
 │ └ Exponent (10 bits)
 └ Sign (1 bit)
```

### 64-bit

A bit-level representation of a 64-bit floating-point number.

The bits are laid out as follows:

```plain
MSB                           ...                           LSB
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬╴╴╴┬─┐
└─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴─┴╴╴╴┴─┘
 │ ├─────────────────┘ ├───────────────────────────────────╴╴╴─┘
 │ │                   └ Significand (53 bits)
 │ └ Exponent (11 bits)
 └ Sign (1 bit)
```
