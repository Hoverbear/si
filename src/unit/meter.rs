use {Fraction, Number};
use unit::Unit;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Meter(Fraction);

impl Unit for Meter {
  fn new(val: Fraction) -> Self {
    Meter(val)
  }
  fn take(self) -> Fraction {
    self.0
  }
}

impl From<Number> for Meter {
  fn from(val: Number) -> Self {
    Meter(Fraction::from_integer(val))
  }
}

impl From<Fraction> for Meter {
  fn from(val: Fraction) -> Self {
    Meter(val)
  }
}

use prefix::*;
#[test]
fn kilo() {
  let km = Kilo::<Meter>::from(1);
  assert_eq!(km.to_base(), Meter::from(1000));
}