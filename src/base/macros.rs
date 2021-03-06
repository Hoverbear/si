#[macro_use]
macro_rules! base_from_primitives { 
  {
    $unit:ident, 
    [$($primitive:ty,)*]
  } => {
    $(
      impl From<$primitive> for $unit {
        fn from(value: $primitive) -> Self {
          Self::from(BigInt::from(value))
        }
      }
    )*
  }
}

#[macro_use]
macro_rules! base_div_and_mul_with_primitives { 
  {
    $unit:ident, 
    [$($primitive:ty,)*]
  } => {
    $(
      impl Div<$primitive> for $unit {
        type Output = Self;
        fn div(self, value: $primitive) -> Self {
          self / BigRational::from_integer(BigInt::from(value))
        }
      }

      impl Mul<$primitive> for $unit {
        type Output = Self;
        fn mul(self, value: $primitive) -> Self {
          self * BigRational::from_integer(BigInt::from(value))
        }
      }
    )*
  }
}

#[macro_use]
macro_rules! generate_base {
  {
    name   = $name:ident,
    longform = $longform:ident,
    shortform = $shortform:ident,
    dimension = $dimension:ty,
    $doc:meta,
  } => (
    mod $longform {
      #[cfg(test)] use prefix::Kilo;
      #[cfg(test)] use quickcheck::{Arbitrary, Gen};
      use {Unit, IntoBase};
      use base::Base;
      use dimension::*;
      use num::bigint::BigInt;
      use num::rational::BigRational;
      use prefix::Prefix;
      use std::cmp::*;
      use std::ops::*;

      lazy_static! {
        static ref SHORTFORM: &'static str = stringify!($shortform);
        static ref LONGFORM: &'static str = stringify!($longform);
      }

      #[$doc]
      #[derive(Clone, Debug, Eq)]
      pub struct $name {
        value: BigRational,
      }

      impl Unit for $name {
        fn new(value: BigRational) -> Self {
          $name { 
            value: value 
          }
        }
        fn shortform() -> String {
          (*SHORTFORM).into()
        }
        fn longform() -> String {
          (*LONGFORM).into()
        }
        fn value(self) -> BigRational {
          self.value
        }
        fn value_ref(&self) -> &BigRational {
          &self.value
        }
      }

      #[cfg(test)]
      impl Arbitrary for $name {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
          let (numerator, denominator) = (g.gen::<i64>(), g.gen::<i64>());
          let denominator = if denominator == 0 { 1 } else { denominator }; // The denominator cannot be zero.
          
          let (numerator, denominator) = (BigInt::from(numerator), BigInt::from(denominator));
          let rational = BigRational::new(numerator, denominator);
          Self::new(rational)
        }
      }

      #[test]
      fn has_right_shortform() {
        assert_eq!($name::shortform(), stringify!($shortform))
      }

      #[test]
      fn has_right_longform() {
        assert_eq!($name::longform(), stringify!($longform))
      }

      //
      // Markers
      //
      impl $dimension for $name {}

      impl Base for $name {}

      impl IntoBase<$name> for $name {
        fn base(self) -> Self {
          self
        }
      }

      //
      // Conversions
      //
      impl From<BigInt> for $name {
        fn from(val: BigInt) -> Self {
          let fraction = BigRational::from_integer(val); 
          Self::new(fraction)
        }
      }

      impl From<BigRational> for $name {
        fn from(val: BigRational) -> Self {
          Self::new(val)
        }
      }

      impl<P> From<P> for $name where P: Prefix<$name> {
        fn from(val: P) -> Self {
          val.base()
        }
      }

      base_from_primitives! { $name, [i64, u64, i32, u32, i16, u16, i8, u8, isize, usize,] }

      //
      // Operations on self
      //
      impl Add for $name {
        type Output = Self;
        fn add(self, other: Self) -> Self {
          Self::new(self.value + other.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() + second.clone().value();
          (first + second).value() == check
        }
      }

      impl Sub for $name {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
          Self::new(self.value - other.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() - second.clone().value();
          (first - second).value() == check
        }
      }

      impl Div for $name {
        type Output = Self;
        fn div(self, other: Self) -> Self {
          Self::new(self.value / other.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
      }

      impl Mul for $name {
        type Output = Self;
        fn mul(self, other: Self) -> Self {
          Self::new(self.value * other.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() * second.clone().value();
          (first * second).value() == check
        }
      }

      //
      // Operations on prefixes
      //
      impl<P> Add<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn add(self, other: P) -> Self {
          self + other.base()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() + second.clone().base().value();
          (first + second).value() == check
        }
      }

      impl<P> Sub<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn sub(self, value: P) -> Self {
          self - value.base()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() - second.clone().base().value();
          (first - second).value() == check
        }
      }

      //
      // Dividing and multiplication are defined on integral types.
      //
      impl Div<BigRational> for $name {
        type Output = Self;
        fn div(self, value: BigRational) -> Self {
          Self::from(self.value() / value)
        }
      }

      impl Div<BigInt> for $name {
        type Output = Self;
        fn div(self, value: BigInt) -> Self {
          Self::from(self.value() / BigRational::from_integer(value))
        }
      }

      impl Mul<BigRational> for $name {
        type Output = Self;
        fn mul(self, value: BigRational) -> Self {
          Self::from(self.value() * value)
        }
      }

      impl Mul<BigInt> for $name {
        type Output = Self;
        fn mul(self, value: BigInt) -> Self {
          Self::from(self.value() * BigRational::from_integer(value))
        }
      }

      base_div_and_mul_with_primitives! { $name, [i64, u64, i32, u32, i16, u16, i8, u8, isize, usize,] }

      //
      // Equals
      //
      impl PartialEq for $name {
        fn eq(&self, other: &$name) -> bool {
          self.value_ref() == other.value_ref()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_eq_self(value: $name) -> bool {
          let duplicate = value.clone();
          value == duplicate
        }
      }

      impl<P> PartialEq<P> for $name where P: Prefix<$name> {
        fn eq(&self, other: &P) -> bool {
          self.value_ref() == other.clone().base().value_ref()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_eq_prefix(value: $name) -> bool {
          let as_kilo = Kilo::scale(value.clone());
          value == as_kilo
        }
      }
    }
    pub use self::$longform::$name;
  )
}
