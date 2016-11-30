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
      use prefix::Prefix;
      use std::ops::*;
      #[cfg(test)] use quickcheck::{Arbitrary, Gen};
      #[cfg(test)] use prefix::Kilo;

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

      //
      // Operations on this type
      //

      impl Add<$name> for $name {
        type Output = Self;
        fn add(self, value: Self) -> Self {
          Self::new(self.value + value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() + second.clone().value();
          (first + second).value() == check
        }
      }

      impl AddAssign<$name> for $name {
        fn add_assign(&mut self, value: Self) {
          self.value = self.value.clone() + value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_assign_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() + second.clone().value();
          let mut first = first;
          first += second;
          first.value() == check
        }
      }

      impl Sub<$name> for $name {
        type Output = Self;
        fn sub(self, value: Self) -> Self {
          Self::new(self.value - value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() - second.clone().value();
          (first - second).value() == check
        }
      }

      impl SubAssign<$name> for $name {
        fn sub_assign(&mut self, value: Self) {
          self.value = self.value.clone() - value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_assign_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() - second.clone().value();
          let mut first = first;
          first -= second;
          first.value() == check
        }
      }

      impl Div<$name> for $name {
        type Output = Self;
        fn div(self, value: Self) -> Self {
          Self::new(self.value / value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
      }

      impl DivAssign<$name> for $name {
        fn div_assign(&mut self, value: Self) {
          self.value = self.value.clone() / value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_assign_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          let mut first = first;
          first /= second;
          first.value() == check
        }
      }

      impl Mul<$name> for $name {
        type Output = Self;
        fn mul(self, value: Self) -> Self {
          Self::new(self.value / value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
      }

      impl MulAssign<$name> for $name {
        fn mul_assign(&mut self, value: Self) {
          self.value = self.value.clone() * value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_assign_self(first: $name, second: $name) -> bool {
          let check = first.clone().value() * second.clone().value();
          let mut first = first;
          first *= second;
          first.value() == check
        }
      }

      //
      // Operations on prefixes
      //
      impl<P> Add<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn add(self, value: P) -> Self {
          Self::new(self.value() + value.base().value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() + second.clone().base().value();
          (first + second).value() == check
        }
      }

      impl<P> AddAssign<P> for $name where P: Prefix<$name> {
        fn add_assign(&mut self, value: P) {
          self.value = self.value.clone() + value.base().value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_assign_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() + second.clone().base().value();
          let mut first = first;
          first += second;
          first.value() == check
        }
      }

      impl<P> Sub<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn sub(self, value: P) -> Self {
          Self::new(self.value - value.base().value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() - second.clone().base().value();
          (first - second).value() == check
        }
      }

      impl<P> SubAssign<P> for $name where P: Prefix<$name> {
        fn sub_assign(&mut self, value: P) {
          self.value = self.value.clone() - value.base().value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_assign_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() - second.clone().base().value();
          let mut first = first;
          first -= second;
          first.value() == check
        }
      }

      impl<P> Div<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn div(self, value: P) -> Self {
          Self::new(self.value / value.base().value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() / second.clone().base().value();
          (first / second).value() == check
        }
      }

      impl<P> DivAssign<P> for $name where P: Prefix<$name> {
        fn div_assign(&mut self, value: P) {
          self.value = self.value.clone() / value.base().value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_assign_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() / second.clone().base().value();
          let mut first = first;
          first /= second;
          first.value() == check
        }
      }

      impl<P> Mul<P> for $name where P: Prefix<$name> {
        type Output = Self;
        fn mul(self, value: P) -> Self {
          Self::new(self.value / value.base().value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() / second.clone().base().value();
          (first / second).value() == check
        }
      }

      impl<P> MulAssign<P> for $name where P: Prefix<$name> {
        fn mul_assign(&mut self, value: P) {
          self.value = self.value.clone() * value.base().value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_assign_prefix(first: $name, second: Kilo<$name>) -> bool {
          let check = first.clone().value() * second.clone().base().value();
          let mut first = first;
          first *= second;
          first.value() == check
        }
      }
    }
    pub use self::$longform::$name;
  )
}
