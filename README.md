memsize
========

Rust constants and functions for dealing with memory unit sizes.

## Constants

Includes the following constants representing the "traditional" memory unit sizes,
*e.g.* 1 kilobyte == 1024 bytes.

```
BYTE_SIZE
KILOBYTE_SIZE
MEGABYTE_SIZE
GIGABYTE_SIZE
TERABYTE_SIZE
PETABYTE_SIZE
EXABYTE_SIZE
ZETTABYTE_SIZE
YOTTABYTE_SIZE
```

Additionally, the following constants are also included for the metric decimal interpretations
of memory unit sizes, *e.g.* 1 kilobyte == 1000 bytes.

```
DECIMAL_BYTE_SIZE
DECIMAL_KILOBYTE_SIZE
DECIMAL_MEGABYTE_SIZE
DECIMAL_GIGABYTE_SIZE
DECIMAL_TERABYTE_SIZE
DECIMAL_PETABYTE_SIZE
DECIMAL_EXABYTE_SIZE
DECIMAL_ZETTABYTE_SIZE
DECIMAL_YOTTABYTE_SIZE
```

## Enums

```
pub enum MemoryUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
    Exabyte,
    Zettabyte,
    Yottabyte,
}
```

