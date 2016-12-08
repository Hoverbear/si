extern crate num;

#[macro_use] extern crate lazy_static;

#[cfg(test)] #[macro_use] extern crate quickcheck;

pub mod prefix;
pub mod base;
pub mod dimension;

use std::ops::*;
use num::bigint::BigInt;
use num::rational::BigRational;

pub trait Unit: Clone + Eq 
+ From<BigRational> + From<BigInt>
+ From<i64> + From<u64> + From<i32> + From<u32> + From<usize> + From<isize>
+ From<i16> + From<u16> + From<i8> + From <u8>
+ Add<Self,Output=Self> + Sub<Self,Output=Self>
+ Mul<BigRational> + Mul<BigInt>
+ Mul<i64> + Mul<u64> + Mul<i32> + Mul<u32> + Mul<usize> + Mul<isize>
+ Mul<i16> + Mul<u16> + Mul<i8> + Mul <u8>
+ Div<BigRational> + Div<BigInt>
+ Div<i64> + Div<u64> + Div<i32> + Div<u32> + Div<usize> + Div<isize>
+ Div<i16> + Div<u16> + Div<i8> + Div <u8> {
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

pub trait IntoBase<B>: Unit + Add<Self,Output=Self> + Sub<Self,Output=Self> where B: Unit {
  /// Get the base unit.
  fn base(self) -> B;
}