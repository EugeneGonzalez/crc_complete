use {Crc8, Crc16, Crc32, Crc64};
use polynomial::{Reflect, Yes, No, CrcPoly, CrcPolynomial};
use hasher::CrcHasher;
use hasher::table_builder::CrcTableBuilder;
use std::mem;


pub struct Table<'a, R, T>
    where R: 'a + Reflect,
          T: 'a
{
    polynomial: &'a CrcPoly<R, T>,
    table: [T; 256],
}


impl<'a> CrcHasher<'a, Yes, Crc8> for Table<'a, Yes, Crc8>
    where CrcPoly<Yes, Crc8>: CrcPolynomial<Crc8>
{
    fn with(polynomial: &'a CrcPoly<Yes, Crc8>) -> Self {
        let mut hasher = Table {
            polynomial: polynomial,
            table: unsafe { mem::uninitialized() },
        };

        hasher.table.build_table(polynomial);
        hasher
    }

    #[inline]
    fn initial_value(&self) -> Crc8 {
        self.polynomial.init_val()
    }

    fn update_crc(&self, crc: Crc8, bytes: &[u8]) -> Crc8 {
        let mut crc = crc;

        for &byte in bytes {
            crc = self.table[((crc as u8) ^ byte) as usize];
        }

        crc
    }

    #[inline]
    fn finalize_crc(&self, crc: Crc8) -> Crc8 {
        self.polynomial.finalize_crc(crc)
    }
}

impl<'a> CrcHasher<'a, No, Crc8> for Table<'a, No, Crc8>
    where CrcPoly<No, Crc8>: CrcPolynomial<Crc8>
{
    fn with(polynomial: &'a CrcPoly<No, Crc8>) -> Self {
        let mut hasher = Table {
            polynomial: polynomial,
            table: unsafe { mem::uninitialized() },
        };

        hasher.table.build_table(polynomial);
        hasher
    }

    #[inline]
    fn initial_value(&self) -> Crc8 {
        self.polynomial.init_val()
    }

    fn update_crc(&self, crc: Crc8, bytes: &[u8]) -> Crc8 {
        let mut crc = crc;

        for &byte in bytes {
            crc = self.table[((crc.rotate_left(8) as u8) ^ byte) as usize];
        }

        crc
    }

    #[inline]
    fn finalize_crc(&self, crc: Crc8) -> Crc8 {
        self.polynomial.finalize_crc(crc)
    }
}

macro_rules! doit {
    ($($ty:ty),*) => ($(
        impl<'a> CrcHasher<'a, Yes, $ty> for Table<'a, Yes, $ty>
            where CrcPoly<Yes, $ty>: CrcPolynomial<$ty>
        {
            fn with(polynomial: &'a CrcPoly<Yes, $ty>) -> Self {
                let mut hasher = Table {
                    polynomial: polynomial,
                    table: unsafe { mem::uninitialized() },
                };

                hasher.table.build_table(polynomial);
                hasher
            }

            #[inline]
            fn initial_value(&self) -> $ty {
                self.polynomial.init_val()
            }

            fn update_crc(&self, crc: $ty, bytes: &[u8]) -> $ty {
                let mut crc = crc;

                for &byte in bytes {
                    crc = (crc >> 8) ^ self.table[((crc as u8) ^ byte) as usize];
                }

                crc
            }

            #[inline]
            fn finalize_crc(&self, crc: $ty) -> $ty {
                self.polynomial.finalize_crc(crc)
            }
        }

        impl<'a> CrcHasher<'a, No, $ty> for Table<'a, No, $ty>
            where CrcPoly<No, $ty>: CrcPolynomial<$ty>
        {
            fn with(polynomial: &'a CrcPoly<No, $ty>) -> Self {
                let mut hasher = Table {
                    polynomial: polynomial,
                    table: unsafe { mem::uninitialized() },
                };

                hasher.table.build_table(polynomial);
                hasher
            }

            #[inline]
            fn initial_value(&self) -> $ty {
                self.polynomial.init_val()
            }

            fn update_crc(&self, crc: $ty, bytes: &[u8]) -> $ty {
                let mut crc = crc;

                for &byte in bytes {
                    crc = (crc << 8) ^ self.table[((crc.rotate_left(8) as u8) ^ byte) as usize];
                }

                crc
            }

            #[inline]
            fn finalize_crc(&self, crc: $ty) -> $ty {
                self.polynomial.finalize_crc(crc)
            }
        }
    )*)
}

doit!(Crc16, Crc32, Crc64);

macro_rules! test_table_hasher {
    ($($poly:ident),*) => ($(
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod $poly {
            use super::Table;
            use hasher::CrcHasher;
            use polynomial::{CrcPolynomial, $poly};

            #[test]
            fn check() {
                let (bytes, check_val) = $poly.check();
                let hasher = Table::with(&$poly);
                let mut crc = hasher.initial_value();
                crc = hasher.update_crc(crc, bytes);
                crc = hasher.finalize_crc(crc);
                assert!(check_val == crc);
            }
        }
    )*)
}

test_table_hasher!(CRC_8,
                   CRC_8_MAXIM,
                   CRC_16,
                   CRC_16_XMODEM,
                   CRC_32,
                   CRC_32_C,
                   CRC_32_MPEG_2,
                   CRC_32_Q,
                   CRC_64,
                   CRC_64_XZ);
