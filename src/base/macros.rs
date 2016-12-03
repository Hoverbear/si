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

      impl From<i64> for $name {
        fn from(val: i64) -> Self {
          let num = BigInt::from(val);
          let fraction = BigRational::from_integer(num); 
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

      impl<P> Div<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn div(self, value: P) -> Self {
          self / value.base()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() / second.clone().base().value();
          (first / second).value() == check
        }
      }

      impl<P> Mul<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn mul(self, value: P) -> Self {
          self * value.base()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() * second.clone().base().value();
          (first * second).value() == check
        }
      }

      impl<P> PartialEq<P> for $name where P: IntoBase<$name> {
        fn eq(&self, other: &P) -> bool {
          self.value == other.clone().base().value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_eq_self(value: $name) -> bool {
          let duplicate = value.clone();
          value == duplicate
        }
        fn can_eq_prefix(value: $name) -> bool {
          let as_kilo = Kilo::scale(value.clone());
          value == as_kilo
        }
      }
    }
    pub use self::$longform::$name;
  )
}
