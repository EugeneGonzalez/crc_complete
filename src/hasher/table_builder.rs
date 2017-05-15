use {Crc8, Crc16, Crc32, Crc64};
use polynomial::{Reflect, Yes, No, CrcPoly, CrcPolynomial};
use bit_reverse::ParallelReverse;

pub trait CrcTableBuilder<R, T>
    where R: Reflect,
          CrcPoly<R, T>: CrcPolynomial<T>
{
    fn build_table(&mut self, polynomial: &CrcPoly<R, T>);
}

macro_rules! doit_table {
    ($($ty:ty),*) => ($(
        impl CrcTableBuilder<Yes, $ty> for [$ty; 256]
            where CrcPoly<Yes, $ty>: CrcPolynomial<$ty>
        {
            fn build_table(&mut self, polynomial: &CrcPoly<Yes, $ty>) {
                let polynomial = polynomial.poly().swap_bits();
                let mut value = polynomial;

                self[0] = 0;
                self[128] = value;

                let mut i = 64;
                while i > 0 {
                    value = (value >> 1) ^ ((value & (1 as $ty)).wrapping_neg() & polynomial);
                    self[i] = value;
                    i = i >> 1;
                }

                i = 2;
                while i < 256 {
                    let temp = self[i];
                    for j in 1..i {
                        self[i + j] = temp ^ self[j];
                    }

                    i = i << 1;
                }
            }
        }

        impl CrcTableBuilder<No, $ty> for [$ty; 256]
            where CrcPoly<No, $ty>: CrcPolynomial<$ty>
        {
            fn build_table(&mut self, polynomial: &CrcPoly<No, $ty>) {
                let polynomial = polynomial.poly();
                let mut value = polynomial;

                self[0] = 0;
                self[1] = value;

                let mut i = 2;
                while i < 256 {
                    value = (value << 1) ^
                            ((value & (1 as $ty).rotate_right(1)).rotate_left(1).wrapping_neg() &
                             polynomial);
                    self[i] = value;
                    i = i << 1;
                }

                i = 2;
                while i < 256 {
                    let temp = self[i];
                    for j in 1..i {
                        self[i + j] = temp ^ self[j];
                    }

                    i = i << 1;
                }
            }
        }
    )*)
}

doit_table!(Crc8, Crc16, Crc32, Crc64);

macro_rules! doit_slices_crc8 {
    ($($e:expr),*) => ($(
        impl CrcTableBuilder<Yes, Crc8> for [[Crc8; 256]; $e]
            where CrcPoly<Yes, Crc8>: CrcPolynomial<Crc8>
        {
            fn build_table(&mut self, polynomial: &CrcPoly<Yes, Crc8>) {
                self[0].build_table(polynomial);


                for byte in 0..256 {
                    for i in 1..$e {
                        let temp = self[i - 1][byte];
                        self[i][byte] = self[0][temp as usize];
                    }
                }
            }
        }

        impl CrcTableBuilder<No, Crc8> for [[Crc8; 256]; $e]
            where CrcPoly<No, Crc8>: CrcPolynomial<Crc8>
        {
            fn build_table(&mut self, polynomial: &CrcPoly<No, Crc8>) {
                self[0].build_table(polynomial);


                for byte in 0..256 {
                    for i in 1..$e {
                        let temp = self[i - 1][byte];
                        self[i][byte] = self[0][temp as usize];
                    }
                }
            }
        }
    )*)
}

doit_slices_crc8!(4, 8, 16);


macro_rules! doit_slices {
    ($($ty:ty),*) => ($(
        doit_slices!($ty; 4, 8, 16);
    )*);
    ($ty:ty; $($e:expr),*) => ($(
        impl CrcTableBuilder<Yes, $ty> for [[$ty; 256]; $e]
            where CrcPoly<Yes, $ty>: CrcPolynomial<$ty>
        {
            fn build_table(&mut self, polynomial: &CrcPoly<Yes, $ty>) {
                self[0].build_table(polynomial);


                for byte in 0..256 {
                    for i in 1..$e {
                        let temp = self[i - 1][byte];
                        self[i][byte] = (temp >> 8) ^ self[0][(temp & 0xFF) as usize];
                    }
                }
            }
        }

        impl CrcTableBuilder<No, $ty> for [[$ty; 256]; $e]
            where CrcPoly<No, $ty>: CrcPolynomial<$ty>
        {
            fn build_table(&mut self, polynomial: &CrcPoly<No, $ty>) {
                self[0].build_table(polynomial);


                for byte in 0..256 {
                    for i in 1..$e {
                        let temp = self[i - 1][byte];
                        self[i][byte] = (temp << 8) ^
                                        self[0][(temp & (0xFF as $ty).rotate_right(8))
                            .rotate_left(8) as usize];
                    }
                }
            }
        }
    )*)
}

doit_slices!(Crc16, Crc32, Crc64);

macro_rules! test_tables {
    ($($poly:ident),*) => ($(
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod $poly {
            use super::CrcTableBuilder;
            use hasher::CrcHasher;
            use hasher::bitwise::Bitwise;
            use polynomial::$poly;

            #[test]
            fn table_check() {
                let mut bytes = [0u8; 1];
                let hasher = Bitwise::with(&$poly);
                let mut table = [0; 256];
                table.build_table(&$poly);

                for i in 0..256 {
                    bytes[0] = i as u8;
                    assert!(table[i] == hasher.update_crc(0, &bytes));
                }
            }

            #[test]
            fn slicex4_check() {
                let mut bytes = [0u8; 4];
                let hasher = Bitwise::with(&$poly);
                let mut table = [[0; 256]; 4];
                table.build_table(&$poly);

                for i in 0..4 {
                    let index = bytes.len() - i - 1;
                    for j in 0..256 {
                        bytes[index] = j as u8;
                        assert!(table[i][j as usize] == hasher.update_crc(0, &bytes));
                    }

                    bytes[index] = 0;
                }
            }

            #[test]
            fn slicex8_check() {
                let mut bytes = [0u8; 8];
                let hasher = Bitwise::with(&$poly);
                let mut table = [[0; 256]; 8];
                table.build_table(&$poly);

                for i in 0..8 {
                    let index = bytes.len() - i - 1;
                    for j in 0..256 {
                        bytes[index] = j as u8;
                        assert!(table[i][j as usize] == hasher.update_crc(0, &bytes));
                    }

                    bytes[index] = 0;
                }
            }

            #[test]
            fn slicex16_check() {
                let mut bytes = [0u8; 16];
                let hasher = Bitwise::with(&$poly);
                let mut table = [[0; 256]; 16];
                table.build_table(&$poly);

                for i in 0..16 {
                    let index = bytes.len() - i - 1;
                    for j in 0..256 {
                        bytes[index] = j as u8;
                        assert!(table[i][j as usize] == hasher.update_crc(0, &bytes));
                    }

                    bytes[index] = 0;
                }
            }
        }
    )*)
}

test_tables!(CRC_8,
             CRC_8_MAXIM,
             CRC_16,
             CRC_16_XMODEM,
             CRC_32,
             CRC_32_C,
             CRC_32_MPEG_2,
             CRC_32_Q,
             CRC_64,
             CRC_64_XZ);
