use {Crc8, Crc16, Crc32, Crc64};
use polynomial::{CrcPolynomial , Normal};
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
pub struct Algorithm<R, T>
    where R: Reflect,
{
    initial: T,
    polynomial: Normal<T>,
    xor_out: T,
    check: T,
    phantom: PhantomData<R>,
}

pub trait CrcAlgorithm<T> {
    fn with_parameters<P: Into<Normal<T>>>(init: T, polynomial: P, xor: T, check: T) -> Self;
    fn initial(&self) -> T;
    fn polynomial(&self) -> T;
    fn finalize_crc(&self, crc: T) -> T;
    fn check(&self) -> (&[u8], T);
}

/// The data used to compute the check value for all CRC polynomials given in this library.
pub static CHECK_MSG: &'static [u8] = b"123456789";

macro_rules! doit {
    ($($ty:ty),*) => ($(
        impl<R> CrcAlgorithm<$ty> for Algorithm<R, $ty> where R: Reflect {
            #[inline]
            fn with_parameters<P: Into<Normal<$ty>>>(init: $ty, polynomial: P, xor: $ty, check: $ty) -> Self {
                Algorithm {
                    initial: init,
                    polynomial: polynomial.into(),
                    xor_out: xor,
                    check: check,
                    phantom: PhantomData,
                }
            }

            #[inline]
            fn initial(&self) -> $ty {
                self.initial
            }

            #[inline]
            fn polynomial(&self) -> $ty {
                self.polynomial.get_polynomial()
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

pub static CRC_8: Algorithm<No, Crc8> = Algorithm {
    initial: 0x00,
    polynomial: Normal(0x07),
    xor_out: 0x00,
    check: 0xf4,
    phantom: PhantomData,
};

pub static CRC_8_MAXIM: Algorithm<Yes, Crc8> = Algorithm {
    initial: 0x00,
    polynomial: Normal(0x31),
    xor_out: 0x00,
    check: 0xa1,
    phantom: PhantomData,
};

pub static CRC_16: Algorithm<Yes, Crc16> = Algorithm {
    initial: 0,
    polynomial: Normal(0x8005),
    xor_out: 0,
    check: 0xbb3d,
    phantom: PhantomData,
};

pub static CRC_16_XMODEM: Algorithm<No, Crc16> = Algorithm {
    initial: 0,
    polynomial: Normal(0x1021),
    xor_out: 0,
    check: 0x31c3,
    phantom: PhantomData,
};

pub static CRC_32: Algorithm<Yes, Crc32> = Algorithm {
    initial: 0xffffffff,
    polynomial: Normal(0x04c11db7),
    xor_out: 0xffffffff,
    check: 0xcbf43926,
    phantom: PhantomData,
};

pub static CRC_32_C: Algorithm<Yes, Crc32> = Algorithm {
    initial: 0xffffffff,
    polynomial: Normal(0x1edc6f41),
    xor_out: 0xffffffff,
    check: 0xe3069283,
    phantom: PhantomData,
};

pub static CRC_32_MPEG_2: Algorithm<No, Crc32> = Algorithm {
    initial: 0xffffffff,
    polynomial: Normal(0x04c11db7),
    xor_out: 0x00000000,
    check: 0x0376e6e7,
    phantom: PhantomData,
};

pub static CRC_32_Q: Algorithm<No, Crc32> = Algorithm {
    initial: 0x00000000,
    polynomial: Normal(0x814141ab),
    xor_out: 0x00000000,
    check: 0x3010bf7f,
    phantom: PhantomData,
};

pub static CRC_64: Algorithm<No, Crc64> = Algorithm {
    initial: 0x0000000000000000,
    polynomial: Normal(0x42f0e1eba9ea3693),
    xor_out: 0x0000000000000000,
    check: 0x6c40df5f0b497347,
    phantom: PhantomData,
};

pub static CRC_64_XZ: Algorithm<Yes, Crc64> = Algorithm {
    initial: 0xffffffffffffffff,
    polynomial: Normal(0x42f0e1eba9ea3693),
    xor_out: 0xffffffffffffffff,
    check: 0x995dc9bbdf1939fa,
    phantom: PhantomData,
};
