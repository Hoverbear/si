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

      lazy_static! {
        static ref FACTOR: BigRational = generate_prefix_factor($factor);
        static ref PREFIX: &'static str = stringify!($prefix);
      }

      #[$doc]
      #[derive(Clone, Debug, Eq, PartialEq)]
      pub struct $name<B>(B) where B: Base;

      // Basic impl
      impl<B> Prefix<B> for $name<B> where B: Base {
        fn factor() -> &'static BigRational {
          &*FACTOR
        }
        fn prefix() -> &'static str {
          &*PREFIX
        }
        fn base(self) -> B {
          self.0
        }
        fn convert<P>(val: P) -> Self where P: Prefix<B> {
          let base = val.base();
          Self::from(base)
        }
      }

      // Create a prefix from a given base.
      impl<B> From<B> for $name<B> where B: Base {
        fn from(val: B) -> Self {
          $name(val)
        }
      }

      // Create a prefix from a BigInt.
      impl<B> From<BigInt> for $name<B> where B: Base {
        fn from(val: BigInt) -> Self {
          let factor = Self::factor().clone();
          let rational = BigRational::from_integer(val);
          $name(B::from(rational * factor))
        }
      }

      // Create a prefix from a i64.
      impl<B> From<i64> for $name<B> where B: Base {
        fn from(val: i64) -> Self {
          let num = BigInt::from(val);
          let factor = Self::factor().clone();
          let fraction = BigRational::from_integer(num);
          $name(B::from(fraction * factor))
        }
      }

      // Create a prefix from a BigRational.
      impl<B> From<BigRational> for $name<B> where B: Base {
        fn from(val: BigRational) -> Self {
          let factor = Self::factor().clone();
          $name(B::from(val * factor))
        }
      }

      #[cfg(test)]
      quickcheck! {
        fn into_prefix_and_back_to_base(meter: Meter) -> bool {
          let prefixed = $name::<Meter>::from(meter.clone());
          prefixed.base() == meter
        }
        fn from_prefix_to_base(meter: Meter) -> bool {
          let val = meter.clone().take() / $name::<Meter>::factor();
          let prefixed = $name::<Meter>::from(val);
          prefixed.base() == meter
        }
      }
    }
    pub use self::$module::$name;
  )
}