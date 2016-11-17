use {Fraction, Number};

mod meter;
pub use self::meter::Meter;

pub trait Unit: From<Fraction> + From<Number> {
  fn take(self) -> Fraction;
  fn new(val: Fraction) -> Self;
}