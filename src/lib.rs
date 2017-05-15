extern crate bit_reverse;

pub mod polynomial_order;
pub mod polynomial;
pub mod hasher;
pub mod crc_util;

pub type Crc8 = u8;
pub type Crc16 = u16;
pub type Crc32 = u32;
pub type Crc64 = u64;
