extern crate num;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod prefix;
pub mod base; 

pub use num::bigint::BigInt;
pub use num::rational::BigRational;
pub use num::bigint::Sign;