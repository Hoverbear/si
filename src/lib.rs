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
  /// Get the value. 
  fn value(self) -> BigRational;
  /// Get a reference to the value.
  fn value_ref(&self) -> &BigRational;
  /// The full string for the unit. Eg `kilometer` for Kilometer.
  fn longform() -> String;
  /// The short hand for the unit. Eg `km` for Kilometer.
  fn shortform() -> String;
}

pub trait IntoBase<B>: Unit where B: Unit {
  /// Get the base unit.
  fn base(self) -> B;
}