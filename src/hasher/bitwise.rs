use {Crc8, Crc16, Crc32, Crc64};
use hasher::CrcHasher;
use polynomial::{Reflect, Yes, No, CrcPoly, CrcPolynomial};
use bit_reverse::ParallelReverse;

pub struct Bitwise<'a, R, T>
    where R: 'a + Reflect,
          T: 'a
{
    polynomial: &'a CrcPoly<R, T>,
}

macro_rules! doit {
    ($($ty:ty),*) => ($(
        impl<'a> CrcHasher<'a, Yes, $ty> for Bitwise<'a, Yes, $ty>
            where CrcPoly<Yes, $ty>: CrcPolynomial<$ty>
        {
            #[inline]
            fn with(polynomial: &'a CrcPoly<Yes, $ty>) -> Self {
                Bitwise {
                    polynomial: polynomial
                }
            }

            #[inline]
            fn initial_value(&self) -> $ty {
                self.polynomial.init_val()
            }

            fn update_crc(&self, crc: $ty, bytes: &[u8]) -> $ty {
                let mut crc = crc;
                let polynomial = self.polynomial.poly().swap_bits();

                for &byte in bytes {
                    crc ^= byte as $ty;

                    for _ in 0..8u8 {
                        crc = (crc >> 1) ^ ((crc & 1).wrapping_neg() & polynomial);
                    }
                }

                crc
            }

            #[inline]
            fn finalize_crc(&self, crc: $ty) -> $ty {
                self.polynomial.finalize_crc(crc)
            }
        }

        impl<'a> CrcHasher<'a, No, $ty> for Bitwise<'a, No, $ty>
            where CrcPoly<No, $ty>: CrcPolynomial<$ty>
        {
            #[inline]
            fn with(polynomial: &'a CrcPoly<No, $ty>) -> Self {
                Bitwise {
                    polynomial: polynomial
                }
            }

            #[inline]
            fn initial_value(&self) -> $ty {
                self.polynomial.init_val()
            }

            fn update_crc(&self, crc: $ty, bytes: &[u8]) -> $ty {
                let mut crc = crc;
                let polynomial = self.polynomial.poly();

                for &byte in bytes {
                    crc ^=  (byte as $ty).rotate_right(8);
                    for _ in 0..8 {
                        crc = (crc << 1) ^
                              ((crc & (1 as $ty).rotate_right(1)).rotate_left(1)
                              .wrapping_neg() & polynomial);
                    }
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

doit!(Crc8, Crc16, Crc32, Crc64);

macro_rules! test_bitwise_hasher {
    ($($poly:ident),*) => ($(
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod $poly {
            use super::Bitwise;
            use hasher::CrcHasher;
            use polynomial::{CrcPolynomial, $poly};

            #[test]
            fn check() {
                let (bytes, check_val) = $poly.check();
                let hasher = Bitwise::with(&$poly);
                let mut crc = hasher.initial_value();
                crc = hasher.update_crc(crc, bytes);
                crc = hasher.finalize_crc(crc);
                assert!(check_val == crc);
            }
        }
    )*)
}

test_bitwise_hasher!(CRC_8,
                     CRC_8_MAXIM,
                     CRC_16,
                     CRC_16_XMODEM,
                     CRC_32,
                     CRC_32_C,
                     CRC_32_MPEG_2,
                     CRC_32_Q,
                     CRC_64,
                     CRC_64_XZ);
