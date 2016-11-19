use {BigRational, BigInt};

mod meter;
pub use self::meter::Meter;

pub trait Base: From<BigRational> + From<BigInt> + Clone + Eq + PartialEq {
  /// The name of the unit. Such as `m` for 'Meter'.
  fn unit(&self) -> &'static str;
  /// Consume the Base unit and return its internal numeric value. 
  fn value(self) -> BigRational;
  /// Create a new base unit from a numeric value.
  fn new(val: BigRational) -> Self;
}