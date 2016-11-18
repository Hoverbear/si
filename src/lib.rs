extern crate num;
#[macro_use] extern crate lazy_static;
#[cfg(test)]#[macro_use] extern crate quickcheck;

pub mod prefix;
pub mod base; 

pub type Number = num::bigint::BigInt;
pub type Fraction = num::rational::BigRational;