//! This module is responsible for handling the different ways to represent a polynomial
//! in memory and how to convert between each form.

pub mod algorithm;

use {Crc8, Crc16, Crc32, Crc64};
use bit_reverse::ParallelReverse;
use std::convert::From;
use std::option::Option;
use std::marker::Sized;

/// Struct that holds a CRC polynomial in Most Significant Bit order.
///
/// The polynomial must contain the 0th degree (lowest bit must be set) and the
/// highest degree of the polynomial is implied. i.e. a 32 degree polynomial has the
///  +1 (0th bit set) and the x^32 implied (would be the 32nd bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Normal<T> (T);

/// Struct that holds a CRC polynomial in Least Significant Bit order.
///
/// The polynomial must contain the 0th degree (highest bit must be set) and the
/// highest degree of the polynomial is implied. i.e. a 32 degree polynomial has the
///  +1 (31st bit set) and the x^32 implied (would be the -1th bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reverse<T> (T);

/// Struct that holds a CRC polynomial in Most Significant Bit order and shows the highest degree
/// but not the lowest degree.
///
/// The polynomial must contain the highest degree (highest bit must be set) and the
/// 0th degree of the polynomial is implied. i.e. a 32 degree polynomial has the
///  x^32 (31st bit set) and the 0th degree implied (would be the -1th bit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Koopman<T> (T);

pub trait CrcPolynomial<T> where Self: Sized {
    fn with_polynomial(poly: T) -> Option<Self>;
    fn get_polynomial(&self) -> T;
}

macro_rules! doit {
    ($($ty:ty),*) => ($(
        impl CrcPolynomial<$ty> for Normal<$ty> {
            fn with_polynomial(poly: $ty) -> Option<Self> {
                if poly & 1 == 0 {
                    return None;
                }
                
                Some(Normal(poly))
            }

            fn get_polynomial(&self) -> $ty {
                self.0
            }
        }

        impl CrcPolynomial<$ty> for Reverse<$ty> {
            fn with_polynomial(polynomial: $ty) -> Option<Self> {
                if polynomial & (1 as $ty).rotate_right(1) == 0 {
                    return None;
                }
                
                Some(Reverse(polynomial))
            }

            fn get_polynomial(&self) -> $ty {
                self.0
            }
        }

        impl CrcPolynomial<$ty> for Koopman<$ty> {
            fn with_polynomial(polynomial: $ty) -> Option<Self> {
                if polynomial & (1 as $ty).rotate_right(1) == 0 {
                    return None;
                }
                
                Some(Koopman(polynomial))
            }

            fn get_polynomial(&self) -> $ty {
                self.0
            }
        }

        impl From<Reverse<$ty>> for Normal<$ty> {
            #[inline]
            fn from(polynomial: Reverse<$ty>) -> Self {
                Normal(polynomial.0.swap_bits())
            }
        }

        impl From<Koopman<$ty>> for Normal<$ty> {
            #[inline]
            fn from(polynomial: Koopman<$ty>) -> Self {
                Normal(polynomial.0 << 1 | (1 as $ty))
            }
        }

        impl From<Normal<$ty>> for Reverse<$ty> {
            #[inline]
            fn from(polynomial: Normal<$ty>) -> Self {
                Reverse(polynomial.0.swap_bits())
            }
        }

        impl From<Koopman<$ty>> for Reverse<$ty> {
            #[inline]
            fn from(polynomial: Koopman<$ty>) -> Self {
                Reverse((polynomial.0 << 1 | (1 as $ty)).swap_bits())
            }
        }

        impl From<Normal<$ty>> for Koopman<$ty> {
            #[inline]
            fn from(polynomial: Normal<$ty>) -> Self {
                Koopman(polynomial.0 >> 1 | (1 as $ty).rotate_right(1))
            }
        }

        impl From<Reverse<$ty>> for Koopman<$ty> {
            #[inline]
            fn from(polynomial: Reverse<$ty>) -> Self {
                Koopman(polynomial.0.swap_bits() >> 1 | (1 as $ty).rotate_right(1))
            }
        }
    )*)
}

doit!(Crc8, Crc16, Crc32, Crc64);

macro_rules! test_crc_order {
    ($name:ident, $size:ident) => (
        #[cfg(test)]
        mod $name {
            use super::*;
            use super::super::{$size};

            #[test]
            fn valid_normal_polynomial() {
                assert!(Normal::with_polynomial(!0 as $size).is_some());
            }

            #[test]
            #[should_panic]
            fn invalid_normal_polynomial() {
                assert!(Normal::with_polynomial(0 as $size).is_some());
            }

            #[test]
            fn valid_reverse_polynomial() {
                assert!(Reverse::with_polynomial(!0 as $size).is_some());
            }

            #[test]
            #[should_panic]
            fn invalid_reverse_polynomial() {
                assert!(Reverse::with_polynomial(0 as $size).is_some());
            }

            #[test]
            fn valid_koopman_polynomial() {
                assert!(Koopman::with_polynomial(!0 as $size).is_some());
            }

            #[test]
            #[should_panic]
            fn invalid_koopman_polynomial() {
                assert!(Koopman::with_polynomial(0 as $size).is_some());
            }

            #[test]
            fn normal_reverse_identity() {
                let poly = Normal::with_polynomial(!0 as $size).unwrap();
                let into_poly: Reverse<_> = poly.clone().into();
                assert_eq!(poly, into_poly.into());
            }

            #[test]
            fn normal_koopman_identity() {
                let poly = Normal::with_polynomial(!0 as $size).unwrap();
                let into_poly: Koopman<_> = poly.clone().into();
                assert_eq!(poly, into_poly.into());
            }

            #[test]
            fn reverse_normal_identity() {
                let poly = Reverse::with_polynomial(!0 as $size).unwrap();
                let into_poly: Normal<_> = poly.clone().into();
                assert_eq!(poly, into_poly.into());
            }

            #[test]
            fn reverse_koopman_identity() {
                let poly = Normal::with_polynomial(!0 as $size).unwrap();
                let into_poly: Koopman<_> = poly.clone().into();
                assert_eq!(poly, into_poly.into());
            }

            #[test]
            fn koopman_normal_identity() {
                let poly = Koopman::with_polynomial(!0 as $size).unwrap();
                let into_poly: Normal<_> = poly.clone().into();
                assert_eq!(poly, into_poly.into());
            }

            #[test]
            fn koopman_reverse_identity() {
                let poly = Koopman::with_polynomial(!0 as $size).unwrap();
                let into_poly: Reverse<_> = poly.clone().into();
                assert_eq!(poly, into_poly.into());
            }
        }
    )
}

test_crc_order!(crc8, Crc8);
test_crc_order!(crc16, Crc16);
test_crc_order!(crc32, Crc32);
test_crc_order!(crc64, Crc64);