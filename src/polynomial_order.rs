// //! This module is responsible for handling the different ways to represent a polynomial
// //! in memory and how to convert between each form.

// use {Crc8, Crc16, Crc32, Crc64};
// use bit_reverse::ParallelReverse;

// /// Trait that converts between the different representations of a polynomial.
// pub trait CrcPolyOrder<T> {
//     /// Converts the current representation to Normal form. This is Most Significant Bit
//     /// Order with the highest degree implied and the 0th degree set.
//     fn to_normal(&self) -> Normal<T>;

//     /// Converts the current representation to Reverse form. This is Least Significant Bit
//     /// Order with the highest degree implied and the 0th degree set.
//     fn to_reverse(&self) -> Reverse<T>;

//     /// Converts the current representation to Reverse form. This is Most Significant Bit
//     /// Order with the highest degree set and the 0th degree implied.
//     fn to_koopman(&self) -> Koopman<T>;
// }

// /// Struct that holds a CRC polynomial in Most Significant Bit order.
// ///
// /// The polynomial must contain the 0th degree (lowest bit must be set) and the
// /// highest degree of the polynomial is implied. i.e. a 32 degree polynomial has the
// ///  +1 (0th bit set) and the x^32 implied (would be the 32nd bit).
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Normal<T> (pub T);

// /// Struct that holds a CRC polynomial in Least Significant Bit order.
// ///
// /// The polynomial must contain the 0th degree (highest bit must be set) and the
// /// highest degree of the polynomial is implied. i.e. a 32 degree polynomial has the
// ///  +1 (31st bit set) and the x^32 implied (would be the -1th bit).
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Reverse<T> (pub T);

// /// Struct that holds a CRC polynomial in Most Significant Bit order and shows the highest degree
// /// but not the lowest degree.
// ///
// /// The polynomial must contain the highest degree (highest bit must be set) and the
// /// 0th degree of the polynomial is implied. i.e. a 32 degree polynomial has the
// ///  x^32 (31st bit set) and the 0th degree implied (would be the -1th bit).
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Koopman<T> (pub T);

// macro_rules! doit {
//     ($($ty:ty),*) => ($(
//         impl CrcPolyOrder<$ty> for Normal<$ty> {
//             #[inline]
//             fn to_normal(&self) -> Normal<$ty> {
//                 Normal(self.0)
//             }

//             #[inline]
//             fn to_reverse(&self) -> Reverse<$ty> {
//                 Reverse(self.0.swap_bits())
//             }

//             #[inline]
//             fn to_koopman(&self) -> Koopman<$ty> {
//                 Koopman(self.0 >> 1 | (1 as $ty).rotate_right(1))
//             }
//         }

//         impl CrcPolyOrder<$ty> for Reverse<$ty> {
//             #[inline]
//             fn to_normal(&self) -> Normal<$ty> {
//                 Normal(self.0.swap_bits())
//             }

//             #[inline]
//             fn to_reverse(&self) -> Reverse<$ty> {
//                 Reverse(self.0)
//             }

//             #[inline]
//             fn to_koopman(&self) -> Koopman<$ty> {
//                 Koopman(self.0.swap_bits() >> 1 | (1 as $ty).rotate_right(1))
//             }
//         }

//         impl CrcPolyOrder<$ty> for Koopman<$ty> {
//             #[inline]
//             fn to_normal(&self) -> Normal<$ty> {
//                 Normal(self.0 << 1 | (1 as $ty))
//             }

//             #[inline]
//             fn to_reverse(&self) -> Reverse<$ty> {
//                 Reverse((self.0 << 1 | (1 as $ty)).swap_bits())
//             }

//             #[inline]
//             fn to_koopman(&self) -> Koopman<$ty> {
//                 Koopman(self.0)
//             }
//         }
//     )*)
// }

// doit!(Crc8, Crc16, Crc32, Crc64);


// macro_rules! test_crc_order {
//     ($name:ident, $size:ident) => (
//         #[cfg(test)]
//         mod $name {
//             use super::*;
//             use super::super::{$size};

//             #[test]
//             fn normal_normal_identity() {
//                 let poly = Normal(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_normal().to_normal());
//             }

//             #[test]
//             fn normal_reverse_identity() {
//                 let poly = Normal(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_reverse().to_normal());
//             }

//             #[test]
//             fn normal_koopman_identity() {
//                 let poly = Normal(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_koopman().to_normal());
//             }

//             #[test]
//             fn reverse_normal_identity() {
//                 let poly = Reverse(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_normal().to_reverse());
//             }

//             #[test]
//             fn reverse_reverse_identity() {
//                 let poly = Reverse(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_reverse().to_reverse());
//             }

//             #[test]
//             fn reverse_koopman_identity() {
//                 let poly = Reverse(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_koopman().to_reverse());
//             }

//             #[test]
//             fn koopman_normal_identity() {
//                 let poly = Koopman(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_normal().to_koopman());
//             }

//             #[test]
//             fn koopman_reverse_identity() {
//                 let poly = Koopman(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_reverse().to_koopman());
//             }

//             #[test]
//             fn koopman_koopman_identity() {
//                 let poly = Koopman(!0 ^ 2 as $size);
//                 assert_eq!(poly, poly.to_koopman().to_koopman());
//             }
//         }
//     )
// }

// test_crc_order!(crc8, Crc8);
// test_crc_order!(crc16, Crc16);
// test_crc_order!(crc32, Crc32);
// test_crc_order!(crc64, Crc64);