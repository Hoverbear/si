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
      use {BigRational, BigInt, Unit};
      use dimension::*;
      use base::Base;
      use std::ops::*;
      #[cfg(test)] use quickcheck::{Arbitrary, Gen};

      lazy_static! {
        static ref SHORTFORM: &'static str = stringify!($shortform);
        static ref LONGFORM: &'static str = stringify!($longform);
      }

      #[$doc]
      #[derive(Clone, Debug, Eq, PartialEq)]
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

      impl $dimension for $name {}

      impl Base for $name {}

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

      impl Add<$name> for $name {
        type Output = Self;
        fn add(self, value: Self) -> Self {
          Self::new(self.value + value.value())
        }
      }

      impl AddAssign<$name> for $name {
        fn add_assign(&mut self, value: Self) {
          self.value = self.value.clone() + value.value()
        }
      }

      impl Sub<$name> for $name {
        type Output = Self;
        fn sub(self, value: Self) -> Self {
          Self::new(self.value - value.value())
        }
      }

      impl SubAssign<$name> for $name {
        fn sub_assign(&mut self, value: Self) {
          self.value = self.value.clone() - value.value()
        }
      }

      impl Div<$name> for $name {
        type Output = Self;
        fn div(self, value: Self) -> Self {
          Self::new(self.value / value.value())
        }
      }

      impl DivAssign<$name> for $name {
        fn div_assign(&mut self, value: Self) {
          self.value = self.value.clone() / value.value()
        }
      }

      impl Mul<$name> for $name {
        type Output = Self;
        fn mul(self, value: Self) -> Self {
          Self::new(self.value / value.value())
        }
      }

      impl MulAssign<$name> for $name {
        fn mul_assign(&mut self, value: Self) {
          self.value = self.value.clone() * value.value()
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

      #[cfg(test)]
      quickcheck! {
        fn can_add_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() + second.clone().value();
          (first + second).value() == check
        }
        fn can_add_assign_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() + second.clone().value();
          let mut first = first;
          first += second;
          first.value() == check
        }
        fn can_sub_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() - second.clone().value();
          (first - second).value() == check
        }
        fn can_sub_assign_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() - second.clone().value();
          let mut first = first;
          first -= second;
          first.value() == check
        }
        fn can_div_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
        fn can_div_assign_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          let mut first = first;
          first /= second;
          first.value() == check
        }
        fn can_mul_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
        fn can_mul_assign_arbitrary(first: $name, second: $name) -> bool {
          let check = first.clone().value() * second.clone().value();
          let mut first = first;
          first *= second;
          first.value() == check
        }
      }
     
    }
    pub use self::$longform::$name;
  )
}
