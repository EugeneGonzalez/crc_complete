use polynomial::{Reflect, CrcPoly, CrcPolynomial};

pub mod bitwise;
pub mod table_builder;
pub mod table;
//pub mod slicex4;

pub use self::bitwise::Bitwise;
pub use self::table::Table;

pub trait CrcHasher<'a, R, T>
    where R: Reflect,
    CrcPoly<R, T>: CrcPolynomial<T>
{
    fn with(polynomial: &'a CrcPoly<R, T>) -> Self;
    fn initial_value(&self) -> T;
    fn update_crc(&self, crc: T, bytes: &[u8]) -> T;
    fn finalize_crc(&self, crc: T) -> T;
}
