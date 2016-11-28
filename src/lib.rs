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
use prefix::Prefix;
use base::Base;

pub trait Unit: From<BigRational> + From<BigInt> + Clone + Eq {
  /// Create a new unit from a numeric value.
  fn new(val: BigRational) -> Self;
  /// Consume the unit and return its internal numeric value. 
  fn value(self) -> BigRational;
  /// The full string for the unit. Eg `kilometer` for Kilometer.
  fn longform() -> &'static str;
  /// The short hand for the unit. Eg `km` for Kilometer.
  fn shortform() -> &'static str;
}