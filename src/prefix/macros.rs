#[macro_use]
macro_rules! prefix_from_primitives { 
  {
    $unit:ident, 
    [$($primitive:ty,)*]
  } => {
    $(
      impl<B> From<$primitive> for $unit<B> where B: Base {
        fn from(value: $primitive) -> Self {
          Self::from(BigInt::from(value))
        }
      }
    )*
  }
}

#[macro_use]
macro_rules! prefix_div_and_mul_with_primitives { 
  {
    $unit:ident, 
    [$($primitive:ty,)*]
  } => {
    $(
      impl<B> Div<$primitive> for $unit<B> where B: Base {
        type Output = Self;
        fn div(self, value: $primitive) -> Self {
          self / BigRational::from_integer(BigInt::from(value))
        }
      }

      impl<B> Mul<$primitive> for $unit<B> where B: Base {
        type Output = Self;
        fn mul(self, value: $primitive) -> Self {
          self * BigRational::from_integer(BigInt::from(value))
        }
      }
    )*
  }
}


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
      #[cfg(test)] use base::Meter;
      #[cfg(test)] use quickcheck::{Arbitrary, Gen};
      use {Unit, IntoBase};
      use base::Base;
      use dimension::*;
      use num::bigint::BigInt;
      use num::rational::BigRational;
      use prefix::*;
      use std::marker::PhantomData;
      use std::ops::*;
      use super::generate_prefix_factor;

      lazy_static! {
        static ref FACTOR: BigRational = generate_prefix_factor($factor);
        static ref SHORTFORM: &'static str = stringify!($shortform);
        static ref LONGFORM: &'static str = stringify!($longform);
      }

      #[$doc]
      #[derive(Clone, Debug, Eq)]
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
        fn value_ref(&self) -> &BigRational {
          &self.value
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
      impl<B> Arbitrary for $name<B> where B: Base + Send + 'static {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
          let (numerator, denominator) = (g.gen::<i64>(), g.gen::<i64>());
          let denominator = if denominator == 0 { 1 } else { denominator }; // The denominator cannot be zero.

          let rational = BigRational::new(BigInt::from(numerator), BigInt::from(denominator));
          $name::<B>::new(rational)
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
        fn scale<P>(value: P) -> Self where P: IntoBase<B> {
          Self::from(value.base())
        }
      }

      impl<B> IntoBase<B> for $name<B> where B: Base {
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
        fn scale_from_kilo_works(value: Kilo<Meter>) -> bool {
          let expected_amount = value.clone().base().value() / $name::<Meter>::factor();
          $name::scale(value).value() == expected_amount
        }
      }

      impl<B> Length for $name<B> where B: Base + Length {}
      impl<B> Mass for $name<B> where B: Base + Mass {}
      impl<B> Time for $name<B> where B: Base + Time {}
      impl<B> Current for $name<B> where B: Base + Current {}
      impl<B> Temperature for $name<B> where B: Base + Temperature{}
      impl<B> Amount for $name<B> where B: Base + Amount {}
      impl<B> Intensity for $name<B> where B: Base + Intensity {}

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

      prefix_from_primitives! { $name, [i64, u64, i32, u32, i16, u16, i8, u8, isize, usize,] }

      //
      // Operations on prefixes
      //
      impl<P,B> Add<P> for $name<B> where P: IntoBase<B>, B: Base {
        type Output = Self;
        fn add(self, value: P) -> Self {
          Self::from(self.base() + value.base())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_add_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().base().value() + second.clone().base().value();
          (first + second).base().value() == check
        }
        fn can_add_other(first: $name<Meter>, second: Kilo<Meter>) -> bool {
          let check = first.clone().base().value() + second.clone().base().value();
          (first + second).base().value() == check
        }
        fn can_add_base(first: $name<Meter>, second: Meter) -> bool {
          let check = first.clone().base().value() + second.clone().value();
          (first + second).base().value() == check
        }
      }

      impl<P,B> Sub<P> for $name<B> where P: IntoBase<B>, B: Base {
        type Output = Self;
        fn sub(self, value: P) -> Self {
          Self::from(self.base() - value.base())
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_sub_self(first: $name<Meter>, second: $name<Meter>) -> bool {
          let check = first.clone().base().value() - second.clone().base().value();
          (first - second).base().value() == check
        }
        fn can_sub_other(first: $name<Meter>, second: Kilo<Meter>) -> bool {
          let check = first.clone().base().value() - second.clone().base().value();
          (first - second).base().value() == check
        }
        fn can_sub_base(first: $name<Meter>, second: Meter) -> bool {
          let check = first.clone().base().value() - second.clone().value();
          (first - second).base().value() == check
        }
      }

      //
      // Dividing and multiplication are defined on integral types.
      //
      impl<B> Div<BigRational> for $name<B> where B: Base {
        type Output = Self;
        fn div(self, value: BigRational) -> Self {
          Self::from(self.value() / value)
        }
      }

      impl<B> Div<BigInt> for $name<B> where B: Base {
        type Output = Self;
        fn div(self, value: BigInt) -> Self {
          Self::from(self.value() / BigRational::from_integer(value))
        }
      }

      impl<B> Mul<BigRational> for $name<B> where B: Base {
        type Output = Self;
        fn mul(self, value: BigRational) -> Self {
          Self::from(self.value() * value)
        }
      }

      impl<B> Mul<BigInt> for $name<B> where B: Base {
        type Output = Self;
        fn mul(self, value: BigInt) -> Self {
          Self::from(self.value() * BigRational::from_integer(value))
        }
      }

      prefix_div_and_mul_with_primitives! { $name, [i64, u64, i32, u32, i16, u16, i8, u8, isize, usize,] }

      impl<P,B> PartialEq<P> for $name<B> where P: IntoBase<B>, B: Base {
        fn eq(&self, other: &P) -> bool {
          self.clone().base().value() == other.clone().base().value()
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn can_eq_self(value: $name<Meter>) -> bool {
          let duplicate = value.clone();
          value == duplicate
        }
        fn can_eq_other(value: $name<Meter>) -> bool {
          let as_kilo = Kilo::scale(value.clone());
          value == as_kilo
        }
      }
    }
    pub use self::$longform::$name;
  )
}
