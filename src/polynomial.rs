use {Crc8, Crc16, Crc32, Crc64};
use std::marker::PhantomData;

/// The trait that determines if a polynomial is to use the reflected algorithm or not.
pub trait Reflect {}

/// Marker for polynomials that are specified to use the reflected algorithm.
pub struct Yes;

/// Marker for polynomials that are specified to use the non-reflected algorithm.
pub struct No;

impl Reflect for Yes {}
impl Reflect for No {}

/// A struct that holds all the information needed to compute the CRC.
pub struct CrcPoly<R, T>
    where R: Reflect
{
    init_val: T,
    poly: T,
    xor_out: T,
    check: T,
    phantom: PhantomData<R>,
}

pub trait CrcPolynomial<T> {
    fn with(init: T, poly: T, xor: T, check: T) -> Self;
    fn init_val(&self) -> T;
    fn poly(&self) -> T;
    fn finalize_crc(&self, crc: T) -> T;
    fn check(&self) -> (&[u8], T);
}

/// The data used to compute the check value for all CRC polynomials given in this library.
pub static CHECK_MSG: &'static [u8] = b"123456789";

macro_rules! doit {
    ($($ty:ty),*) => ($(
        impl<R> CrcPolynomial<$ty> for CrcPoly<R, $ty> where R: Reflect {
            #[inline]
            fn with(init: $ty, poly: $ty, xor: $ty, check: $ty) -> Self {
                CrcPoly {
                    init_val: init,
                    poly: poly,
                    xor_out: xor,
                    check: check,
                    phantom: PhantomData,
                }
            }

            #[inline]
            fn init_val(&self) -> $ty {
                self.init_val
            }

            #[inline]
            fn poly(&self) -> $ty {
                self.poly
            }

            #[inline]
            fn finalize_crc(&self, crc: $ty) -> $ty {
                crc ^ self.xor_out
            }

            #[inline]
            fn check(&self) -> (&[u8], $ty) {
                (CHECK_MSG, self.check)
            }
        }
    )*)
}

doit!(Crc8, Crc16, Crc32, Crc64);

pub static CRC_8: CrcPoly<No, Crc8> = CrcPoly {
    init_val: 0x00,
    poly: 0x07,
    xor_out: 0x00,
    check: 0xf4,
    phantom: PhantomData,
};

pub static CRC_8_MAXIM: CrcPoly<Yes, Crc8> = CrcPoly {
    init_val: 0x00,
    poly: 0x31,
    xor_out: 0x00,
    check: 0xa1,
    phantom: PhantomData,
};

pub static CRC_16: CrcPoly<Yes, Crc16> = CrcPoly {
    init_val: 0,
    poly: 0x8005,
    xor_out: 0,
    check: 0xbb3d,
    phantom: PhantomData,
};

pub static CRC_16_XMODEM: CrcPoly<No, Crc16> = CrcPoly {
    init_val: 0,
    poly: 0x1021,
    xor_out: 0,
    check: 0x31c3,
    phantom: PhantomData,
};

pub static CRC_32: CrcPoly<Yes, Crc32> = CrcPoly {
    init_val: 0xffffffff,
    poly: 0x04c11db7,
    xor_out: 0xffffffff,
    check: 0xcbf43926,
    phantom: PhantomData,
};

pub static CRC_32_C: CrcPoly<Yes, Crc32> = CrcPoly {
    init_val: 0xffffffff,
    poly: 0x1edc6f41,
    xor_out: 0xffffffff,
    check: 0xe3069283,
    phantom: PhantomData,
};

pub static CRC_32_MPEG_2: CrcPoly<No, Crc32> = CrcPoly {
    init_val: 0xffffffff,
    poly: 0x04c11db7,
    xor_out: 0x00000000,
    check: 0x0376e6e7,
    phantom: PhantomData,
};

pub static CRC_32_Q: CrcPoly<No, Crc32> = CrcPoly {
    init_val: 0x00000000,
    poly: 0x814141ab,
    xor_out: 0x00000000,
    check: 0x3010bf7f,
    phantom: PhantomData,
};

pub static CRC_64: CrcPoly<No, Crc64> = CrcPoly {
    init_val: 0x0000000000000000,
    poly: 0x42f0e1eba9ea3693,
    xor_out: 0x0000000000000000,
    check: 0x6c40df5f0b497347,
    phantom: PhantomData,
};

pub static CRC_64_XZ: CrcPoly<Yes, Crc64> = CrcPoly {
    init_val: 0xffffffffffffffff,
    poly: 0x42f0e1eba9ea3693,
    xor_out: 0xffffffffffffffff,
    check: 0x995dc9bbdf1939fa,
    phantom: PhantomData,
};
