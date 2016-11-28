#[macro_use]
macro_rules! generate_prefix {
  {
    name   = $name:ident,
    longform = $longform:ident,
    shortform = $shortform:ident,
    factor = $factor:expr,
    $doc:meta,
  } => (
    mod $longform {
      use {BigRational, BigInt};
      use prefix::*;
      use dimension::*;
      use base::Base;
      #[cfg(test)]use base::Meter;
      use super::generate_prefix_factor;
      #[cfg(test)] use quickcheck::{Arbitrary, Gen};
      use std::marker::PhantomData;
      use std::ops::*;

      lazy_static! {
        static ref FACTOR: BigRational = generate_prefix_factor($factor);
        static ref SHORTFORM: &'static str = stringify!($shortform);
        static ref LONGFORM: &'static str = stringify!($longform);
      }

      #[$doc]
      #[derive(Clone, Debug, Eq, PartialEq)]
      pub struct $name<B> where B: Base {
        value: BigRational,
        base: PhantomData<B>,
      }

      // Basic impl
      impl<B> Unit for $name<B> where B: Base {
        fn new(value: BigRational) -> Self {
          $name {
            value: value,
            base: PhantomData,
          }
        }
        fn shortform() -> String {
          format!("{}{}", *SHORTFORM, B::shortform())
        }
        fn longform() -> String {
          format!("{}{}", *LONGFORM, B::longform())
        }
        fn value(self) -> BigRational {
          self.value
        }
      }

      #[test]
      fn has_right_shortform() {
        assert_eq!($name::<Meter>::shortform(), format!("{}{}", stringify!($shortform), Meter::shortform()))
      }
      #[test]
      fn has_right_longform() {
        assert_eq!($name::<Meter>::longform(), format!("{}{}", stringify!($longform), Meter::longform()))
      }

      #[cfg(test)]
      quickcheck! {
        fn value_functions_as_expected(value: $name<Meter>) -> bool {
          let expected_amount = value.clone().base().value() / $name::<Meter>::factor();
          value.value() == expected_amount
        }
      }

      #[cfg(test)]
      impl Arbitrary for $name<Meter> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
          let (numerator, denominator) = (g.gen::<i64>(), g.gen::<i64>());
          let denominator = if denominator == 0 { 1 } else { denominator }; // The denominator cannot be zero.

          let rational = BigRational::new(BigInt::from(numerator), BigInt::from(denominator));
          $name::<Meter>::new(rational)
        }
      }

      //
      // Markers
      //

      // Enable it to be a prefix.
      impl<B> Prefix<B> for $name<B> where B: Base {
        fn factor() -> &'static BigRational {
          &*FACTOR
        }
        fn scale<P>(value: P) -> Self where P: Unit + Prefix<B> {
          Self::from(value.base())
        }
        fn base(self) -> B {
          B::new(self.value * Self::factor())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn ensure_factor_calculated_correctly(value: Meter) -> bool {
          let amount = value.clone().value() / $name::<Meter>::factor();
          let prefixed = $name::<Meter>::from(amount);
          prefixed.base() == value
        }
        // This is mostly done just to test that convert works. All implementations of convert should similarly work.
        fn scale_to_kilo_works(value: $name<Meter>) -> bool {
          let expected_amount = value.clone().base().value() / Kilo::<Meter>::factor();
          Kilo::scale(value).value() == expected_amount
        }
      }

      impl<B> Distance for $name<B> where B: Base + Distance {}

      //
      // Conversions
      //

      // Create a prefix from a given base.
      impl<B> From<B> for $name<B> where B: Base {
        fn from(value: B) -> Self {
          Self::new(value.value() / Self::factor())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn into_prefix_and_back_to_base_is_equal(value: Meter) -> bool {
          let prefixed = $name::<Meter>::from(value.clone());
          prefixed.base() == value
        }
      }

      // Create a prefix from a BigInt.
      impl<B> From<BigInt> for $name<B> where B: Base {
        fn from(value: BigInt) -> Self {
          let fraction = BigRational::from_integer(value);
          Self::new(fraction)
        }
      }

      #[cfg(test)]
      quickcheck! {
        // Unfortunately BigInt doesn't implement Arbitrary
        fn from_bigint_returns_provided_value(value: i64) -> bool {
          let bigint = BigInt::from(value);
          let prefixed = $name::<Meter>::from(bigint.clone());
          let expected_rational = BigRational::from_integer(bigint);
          prefixed.value() == expected_rational
        }
      }

      // Create a prefix from a i64.
      impl<B> From<i64> for $name<B> where B: Base {
        fn from(value: i64) -> Self {
          let number = BigInt::from(value);
          let fraction = BigRational::from_integer(number);
          Self::new(fraction)
        }
      }

      // Create a prefix from a BigRational.
      impl<B> From<BigRational> for $name<B> where B: Base {
        fn from(value: BigRational) -> Self {
          Self::new(value)
        }
      }

      #[cfg(test)]
      quickcheck! {
        // Unfortunately BigRational doesn't implement Arbitrary
        fn from_rational_returns_provided_value(numerator: i64, denominator: i64) -> bool {
          let denominator = if denominator == 0 { 1 } else { denominator }; // The denominator cannot be zero.
          let rational = BigRational::new(BigInt::from(numerator), BigInt::from(denominator));
          let prefixed = $name::<Meter>::from(rational.clone());
          prefixed.value() == rational
        }
      }

      //
      // Operations
      //
      impl<B> Add<$name<B>> for $name<B> where B: Base {
        type Output = Self;
        fn add(self, value: Self) -> Self {
          Self::new(self.value + value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() + second.clone().value();
          (first + second).value() == check
        }
      }

      impl<B> AddAssign<$name<B>> for $name<B> where B: Base {
        fn add_assign(&mut self, value: Self) {
          self.value = self.value.clone() + value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_assign_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() + second.clone().value();
          let mut first = first;
          first += second;
          first.value() == check
        }
      }

      impl<B> Sub<$name<B>> for $name<B> where B: Base {
        type Output = Self;
        fn sub(self, value: Self) -> Self {
          Self::new(self.value - value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() - second.clone().value();
          (first - second).value() == check
        }
      }

      impl<B> SubAssign<$name<B>> for $name<B> where B: Base {
        fn sub_assign(&mut self, value: Self) {
          self.value = self.value.clone() - value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_assign_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() - second.clone().value();
          let mut first = first;
          first -= second;
          first.value() == check
        }
      }

      impl<B> Div<$name<B>> for $name<B> where B: Base {
        type Output = Self;
        fn div(self, value: Self) -> Self {
          Self::new(self.value / value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
      }

      impl<B> DivAssign<$name<B>> for $name<B> where B: Base {
        fn div_assign(&mut self, value: Self) {
          self.value = self.value.clone() / value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_div_assign_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() / second.clone().value();
          let mut first = first;
          first /= second;
          first.value() == check
        }
      }

      impl<B> Mul<$name<B>> for $name<B> where B: Base {
        type Output = Self;
        fn mul(self, value: Self) -> Self {
          Self::new(self.value / value.value())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().value() / second.clone().value();
          (first / second).value() == check
        }
      }

      impl<B> MulAssign<$name<B>> for $name<B> where B: Base {
        fn mul_assign(&mut self, value: Self) {
          self.value = self.value.clone() * value.value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_mul_assign_arbitrary(first: $name<Meter>, second: $name<Meter>) -> bool {
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
