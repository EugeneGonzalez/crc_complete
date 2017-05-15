#![feature(test)]

extern crate crc_complete;
extern crate test;

const SM_SIZE: usize = 16;
const MD_SIZE: usize = 256;
const KILOBYTE: usize = 1_000;
const MEGABYTE: usize = 1_000_000;
static BYTES: [u8; MEGABYTE] = [0xAA; MEGABYTE];

macro_rules! benchmark_suite {
    ($hasher:ident) => (
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod $hasher {
            benchmark_suite!($hasher;
                             CRC_8,
                             CRC_8_MAXIM,
                             CRC_16,
                             CRC_16_XMODEM,
                             CRC_32,
                             CRC_32_C,
                             CRC_32_MPEG_2,
                             CRC_32_Q,
                             CRC_64,
                             CRC_64_XZ);
        }
    );
    ($hasher:ident; $($poly:ident),*) => ($(
        #[allow(non_snake_case)]
        mod $poly {
            use super::super::*;
            use crc_complete::polynomial::$poly;
            use crc_complete::hasher::{CrcHasher, $hasher};
            use test::Bencher;

            #[bench]
            fn small(b: &mut Bencher) {
                b.bytes = SM_SIZE as u64;

                let hasher = $hasher::with(&$poly);
                let bytes = &BYTES[..b.bytes as usize];
                b.iter(|| {
                    hasher.finalize_crc(hasher.update_crc(hasher.initial_value(), bytes))
                });
            }

            #[bench]
            fn medium(b: &mut Bencher) {
                b.bytes = MD_SIZE as u64;

                let hasher = $hasher::with(&$poly);
                let bytes = &BYTES[..b.bytes as usize];
                b.iter(|| {
                    hasher.finalize_crc(hasher.update_crc(hasher.initial_value(), bytes))
                });
            }

            #[bench]
            fn kilobyte(b: &mut Bencher) {
                b.bytes = KILOBYTE as u64;

                let hasher = $hasher::with(&$poly);
                let bytes = &BYTES[..b.bytes as usize];
                b.iter(|| {
                    hasher.finalize_crc(hasher.update_crc(hasher.initial_value(), bytes))
                });
            }

            #[bench]
            fn megabyte(b: &mut Bencher) {
                b.bytes = MEGABYTE as u64;

                let hasher = $hasher::with(&$poly);
                let bytes = &BYTES[..b.bytes as usize];
                b.iter(|| {
                    hasher.finalize_crc(hasher.update_crc(hasher.initial_value(), bytes))
                });
            }
        }
    )*)
}

benchmark_suite!(Bitwise);
benchmark_suite!(Table);