#[macro_use]
macro_rules! generate_prefix {
  {
    name   = $name:ident,
    module = $module:ident,
    prefix = $prefix:ident,
    factor = $factor:expr,
    $doc:meta,
  } => (
    mod $module {
      use {BigRational, BigInt};
      use base::*;
      use prefix::*;
      use super::generate_prefix_factor;
      #[cfg(test)] use quickcheck::{Arbitrary, Gen};

      lazy_static! {
        static ref FACTOR: BigRational = generate_prefix_factor($factor);
        static ref PREFIX: &'static str = stringify!($prefix);
      }

      #[$doc]
      #[derive(Clone, Debug, Eq, PartialEq)]
      pub struct $name<B>(B) where B: Base;

      // Basic impl
      impl<B> Prefix<B> for $name<B> where B: Base {
        fn new(val: BigRational) -> Self {
          $name(B::new(val))
        }
        fn factor() -> &'static BigRational {
          &*FACTOR
        }
        fn prefix() -> &'static str {
          &*PREFIX
        }
        fn base(self) -> B {
          // Prefixes are just wrappers around the base value.
          self.0
        }
        fn value(self) -> BigRational {
          self.0.value() / Self::factor()
        }
        fn convert<P>(val: P) -> Self where P: Prefix<B> {
          // Prefixes are just wrappers around the base value.
          $name::from(val.base())
        }
      }

      // Create a prefix from a given base.
      impl<B> From<B> for $name<B> where B: Base {
        fn from(val: B) -> Self {
          // Prefixes are just wrappers around the base value.
          $name(val)
        }
      }

      // Create a prefix from a BigInt.
      impl<B> From<BigInt> for $name<B> where B: Base {
        fn from(val: BigInt) -> Self {
          // Prefixes are wrappers around the base value, so we need to scale
          // the base value appropriately.
          let factor = Self::factor().clone();
          let rational = BigRational::from_integer(val);
          $name(B::from(rational * factor))
        }
      }

      // Create a prefix from a i64.
      impl<B> From<i64> for $name<B> where B: Base {
        fn from(val: i64) -> Self {
          // Prefixes are wrappers around the base value, so we need to scale
          // the base value appropriately.
          let num = BigInt::from(val);
          let rational = BigRational::from_integer(num);
          let factor = Self::factor().clone();
          $name(B::from(rational * factor))
        }
      }

      // Create a prefix from a BigRational.
      impl<B> From<BigRational> for $name<B> where B: Base {
        fn from(val: BigRational) -> Self {
          // Prefixes are wrappers around the base value, so we need to scale
          // the base value appropriately.
          let factor = Self::factor().clone();
          $name(B::from(val * factor))
        }
      }

      #[cfg(test)]
      impl Arbitrary for $name<Meter> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
          let (numerator, denominator) = (BigInt::from(g.gen::<i64>()), BigInt::from(g.gen::<i64>()));
          let rational = BigRational::new(numerator, denominator);
          $name::<Meter>::new(rational)
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn into_prefix_and_back_to_base_is_equal(value: Meter) -> bool {
          let prefixed = $name::<Meter>::from(value.clone());
          prefixed.base() == value
        }
        fn ensure_factor_calculated_correctly(value: Meter) -> bool {
          let amount = value.clone().value() / $name::<Meter>::factor();
          let prefixed = $name::<Meter>::from(amount);
          prefixed.base() == value
        }
        fn value_functions_as_expected(value: $name<Meter>) -> bool {
          let expected_amount = value.clone().base().value() / $name::<Meter>::factor();
          value.value() == expected_amount
        }
      }
    }
    pub use self::$module::$name;
  )
}