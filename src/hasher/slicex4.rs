use {Crc8, Crc16, Crc32, Crc64};
use polynomial::{Reflect, Yes, No, CrcPoly, CrcPolynomial};
use hasher::CrcHasher;
use hasher::table_builder::CrcTableBuilder;
use std::mem;


pub struct Slicex4<'a, R, T>
    where R: 'a + Reflect,
          T: 'a
{
    polynomial: &'a CrcPoly<R, T>,
    table: [[T; 256]; 4],
}


impl<'a> CrcHasher<'a, Yes, Crc8> for Slicex4<'a, Yes, Crc8>
    where CrcPoly<Yes, Crc8>: CrcPolynomial<Crc8>
{
    fn with(polynomial: &'a CrcPoly<Yes, Crc8>) -> Self {
        let mut hasher = Slicex4 {
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

        let mut ptr = bytes.as_ptr();
        let mut len = bytes.len();

        while len >= 4 {
            let one = unsafe { *ptr } ^ crc;
            let two = unsafe { *ptr.offset(1) };
            let three = unsafe { *ptr.offset(2) };
            let four = unsafe { *ptr.offset(3) };

            crc = self.table[0][four as usize] ^ self.table[1][three as usize] ^
                  self.table[2][two as usize] ^ self.table[3][one as usize];

            len -= 4;
            ptr = unsafe { ptr.offset(4) };
        }

        while len > 0 {
            crc = self.table[0][(crc ^ unsafe { *ptr }) as usize];

            len -= 1;
            ptr = unsafe { ptr.offset(1) };
        }

        crc
    }

    #[inline]
    fn finalize_crc(&self, crc: Crc8) -> Crc8 {
        self.polynomial.finalize_crc(crc)
    }
}

impl<'a> CrcHasher<'a, No, Crc8> for Slicex4<'a, No, Crc8>
    where CrcPoly<No, Crc8>: CrcPolynomial<Crc8>
{
    fn with(polynomial: &'a CrcPoly<No, Crc8>) -> Self {
        let mut hasher = Slicex4 {
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

        let mut ptr = bytes.as_ptr();
        let mut len = bytes.len();

        while len >= 4 {
            let one = unsafe { *ptr } ^ crc;
            let two = unsafe { *ptr.offset(1) };
            let three = unsafe { *ptr.offset(2) };
            let four = unsafe { *ptr.offset(3) };

            crc = self.table[0][four as usize] ^ self.table[1][three as usize] ^
                  self.table[2][two as usize] ^ self.table[3][one as usize];

            len -= 4;
            ptr = unsafe { ptr.offset(4) };
        }

        while len > 0 {
            crc = self.table[0][(crc ^ unsafe { *ptr }) as usize];

            len -= 1;
            ptr = unsafe { ptr.offset(1) };
        }

        crc
    }

    #[inline]
    fn finalize_crc(&self, crc: Crc8) -> Crc8 {
        self.polynomial.finalize_crc(crc)
    }
}







impl<'a> CrcHasher<'a, Yes, Crc16> for Slicex4<'a, Yes, Crc16>
    where CrcPoly<Yes, Crc16>: CrcPolynomial<Crc16>
{
    fn with(polynomial: &'a CrcPoly<Yes, Crc16>) -> Self {
        let mut hasher = Slicex4 {
            polynomial: polynomial,
            table: unsafe { mem::uninitialized() },
        };

        hasher.table.build_table(polynomial);
        hasher
    }

    #[inline]
    fn initial_value(&self) -> Crc16 {
        self.polynomial.init_val()
    }

    fn update_crc(&self, crc: Crc16, bytes: &[u8]) -> Crc16 {
        let mut crc = crc;

        let mut ptr = bytes.as_ptr() as *const Crc16;
        let mut len = bytes.len();

        while len >= 4 {
            let one = unsafe { *ptr } ^ crc;
            let two = unsafe { *ptr.offset(1) };

            crc = self.table[0][four as usize] ^ self.table[1][three as usize] ^
                  self.table[2][two as usize] ^ self.table[3][one as usize];

            len -= 4;
            ptr = unsafe { ptr.offset(2) };
        }

        while len > 0 {
            crc = self.table[0][(crc ^ unsafe { *ptr }) as usize];

            len -= 1;
            ptr = unsafe { ptr.offset(1) };
        }

        crc
    }

    #[inline]
    fn finalize_crc(&self, crc: Crc16) -> Crc16 {
        self.polynomial.finalize_crc(crc)
    }
}

impl<'a> CrcHasher<'a, No, Crc16> for Slicex4<'a, No, Crc16>
    where CrcPoly<No, Crc16>: CrcPolynomial<Crc16>
{
    fn with(polynomial: &'a CrcPoly<No, Crc16>) -> Self {
        let mut hasher = Slicex4 {
            polynomial: polynomial,
            table: unsafe { mem::uninitialized() },
        };

        hasher.table.build_table(polynomial);
        hasher
    }

    #[inline]
    fn initial_value(&self) -> Crc16 {
        self.polynomial.init_val()
    }

    fn update_crc(&self, crc: Crc16, bytes: &[u8]) -> Crc16 {
        let mut crc = crc;

        let mut ptr = bytes.as_ptr();
        let mut idx = 0;
        let len = bytes.len();

        while idx + 4 <= len {
            let one = unsafe { *ptr } ^ crc;
            let two = unsafe { *ptr.offset(1) };
            let three = unsafe { *ptr.offset(2) };
            let four = unsafe { *ptr.offset(3) };

            crc = self.table[0][four as usize] ^ self.table[1][three as usize] ^
                  self.table[2][two as usize] ^ self.table[3][one as usize];

            idx += 4;
            ptr = unsafe { ptr.offset(4) };
        }

        while idx < len {
            crc = self.table[0][(crc ^ unsafe { *ptr }) as usize];

            idx += 1;
            ptr = unsafe { ptr.offset(1) };
        }

        crc
    }

    #[inline]
    fn finalize_crc(&self, crc: Crc16) -> Crc16 {
        self.polynomial.finalize_crc(crc)
    }
}

macro_rules! test_slicex4_hasher {
    ($($poly:ident),*) => ($(
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod $poly {
            use super::Slicex4;
            use hasher::CrcHasher;
            use polynomial::{CrcPolynomial, $poly};

            #[test]
            fn check() {
                let (bytes, check_val) = $poly.check();
                let hasher = Slicex4::with(&$poly);
                let mut crc = hasher.initial_value();
                crc = hasher.update_crc(crc, bytes);
                crc = hasher.finalize_crc(crc);
                assert!(check_val == crc);
            }
        }
    )*)
}

test_slicex4_hasher!(CRC_8,
                     CRC_8_MAXIM /* CRC_16,
                                  * CRC_16_XMODEM,
                                  * CRC_32,
                                  * CRC_32_C,
                                  * CRC_32_MPEG_2,
                                  * CRC_32_Q,
                                  * CRC_64,
                                  * CRC_64_XZ */);
