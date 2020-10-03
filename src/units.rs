/// Memory units
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

pub const BYTE_SIZE:      u32  = 1;                                 // 2 ^ 0
pub const KILOBYTE_SIZE:  u32  = 1_024;                             // 2 ^ 10
pub const MEGABYTE_SIZE:  u32  = 1_048_576;                         // 2 ^ 20
pub const GIGABYTE_SIZE:  u32  = 1_073_741_824;                     // 2 ^ 30
pub const TERABYTE_SIZE:  u64  = 1_099_511_627_776;                 // 2 ^ 40
pub const PETABYTE_SIZE:  u64  = 1_125_899_906_842_624;             // 2 ^ 50
pub const EXABYTE_SIZE:   u64  = 1_152_921_504_606_846_976;         // 2 ^ 60
pub const ZETTABYTE_SIZE: u128 = 1_180_591_620_717_411_303_424;     // 2 ^ 70
pub const YOTTABYTE_SIZE: u128 = 1_208_925_819_614_629_174_706_176; // 2 ^ 80


pub const DECIMAL_BYTE_SIZE:      u32  =  1;                                 // 10 ^ 0
pub const DECIMAL_KILOBYTE_SIZE:  u32  =  1_000;                             // 10 ^ 3
pub const DECIMAL_MEGABYTE_SIZE:  u32  =  1_000_000;                         // 10 ^ 6
pub const DECIMAL_GIGABYTE_SIZE:  u32  =  1_000_000_000;                     // 10 ^ 9
pub const DECIMAL_TERABYTE_SIZE:  u64  =  1_000_000_000_000;                 // 10 ^ 12
pub const DECIMAL_PETABYTE_SIZE:  u64  =  1_000_000_000_000_000;             // 10 ^ 15
pub const DECIMAL_EXABYTE_SIZE:   u64  =  1_000_000_000_000_000_000;         // 10 ^ 18
pub const DECIMAL_ZETTABYTE_SIZE: u128 =  1_000_000_000_000_000_000_000;     // 10 ^ 21
pub const DECIMAL_YOTTABYTE_SIZE: u128 =  1_000_000_000_000_000_000_000_000; // 10 ^ 24


/// Returns the multiplier using the "traditional" binary interpretation.
pub fn binary_multiplier(m: &MemoryUnit) -> u128 {
    match m {
        MemoryUnit::Byte      => BYTE_SIZE.into(),
        MemoryUnit::Kilobyte  => KILOBYTE_SIZE.into(),
        MemoryUnit::Megabyte  => MEGABYTE_SIZE.into(),
        MemoryUnit::Gigabyte  => GIGABYTE_SIZE.into(),
        MemoryUnit::Terabyte  => TERABYTE_SIZE.into(),
        MemoryUnit::Petabyte  => PETABYTE_SIZE.into(),
        MemoryUnit::Exabyte   => EXABYTE_SIZE.into(),
        MemoryUnit::Zettabyte => ZETTABYTE_SIZE.into(),
        MemoryUnit::Yottabyte => YOTTABYTE_SIZE.into(),
    }
}

/// Returns the multiplier using the metric decimal interpretation
pub fn decimal_multiplier(m: &MemoryUnit) -> u128 {
    match m {
        MemoryUnit::Byte      => DECIMAL_BYTE_SIZE.into(),   
        MemoryUnit::Kilobyte  => DECIMAL_KILOBYTE_SIZE.into(),
        MemoryUnit::Megabyte  => DECIMAL_MEGABYTE_SIZE.into(),
        MemoryUnit::Gigabyte  => DECIMAL_GIGABYTE_SIZE.into(),
        MemoryUnit::Terabyte  => DECIMAL_TERABYTE_SIZE.into(),
        MemoryUnit::Petabyte  => DECIMAL_PETABYTE_SIZE.into(),
        MemoryUnit::Exabyte   => DECIMAL_EXABYTE_SIZE.into(), 
        MemoryUnit::Zettabyte => DECIMAL_ZETTABYTE_SIZE.into(),
        MemoryUnit::Yottabyte => DECIMAL_YOTTABYTE_SIZE.into(),
    }
}
