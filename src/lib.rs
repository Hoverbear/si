extern crate num;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod prefix;
pub mod base;
pub mod dimension;

use std::ops::*;
use num::bigint::BigInt;
use num::rational::BigRational;

pub trait Unit: From<BigRational> + From<BigInt> + Clone + Eq + 
Add<Self,Output=Self> + Sub<Self,Output=Self> + Mul<Self,Output=Self> + Div<Self,Output=Self> {
  /// Create a new unit from a numeric value.
  fn new(val: BigRational) -> Self;
  /// Consume the unit and return its internal numeric value. 
  fn value(self) -> BigRational;
  /// The full string for the unit. Eg `kilometer` for Kilometer.
  fn longform() -> String;
  /// The short hand for the unit. Eg `km` for Kilometer.
  fn shortform() -> String;
}