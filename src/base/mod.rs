use {Fraction, Number};

mod meter;
pub use self::meter::Meter;

pub trait Base: From<Fraction> + From<Number> + Clone + Eq + PartialEq {
  fn value(&self) -> &Fraction;
  fn take(self) -> Fraction;
  fn new(val: Fraction) -> Self;
}