use {Fraction, Number};
use base::Base;
#[cfg(test)] use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meter(Fraction);

impl Base for Meter {
  fn new(val: Fraction) -> Self {
    Meter(val)
  }
  fn value(&self) -> &Fraction {
    &self.0
  }
  fn take(self) -> Fraction {
    self.0
  }
}

impl From<Number> for Meter {
  fn from(val: Number) -> Self {
    let fraction = Fraction::from_integer(val); 
    Meter(fraction)
  }
}

impl From<i64> for Meter {
  fn from(val: i64) -> Self {
    let num = Number::from(val);
    let fraction = Fraction::from_integer(num); 
    Meter(fraction)
  }
}

impl From<Fraction> for Meter {
  fn from(val: Fraction) -> Self {
    Meter(val)
  }
}

#[cfg(test)]
impl Arbitrary for Meter {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let (numerator, denominator) = (Number::from(g.gen::<i64>()), Number::from(g.gen::<i64>()));
    let fraction = Fraction::new(numerator, denominator);
    Meter::new(fraction)
  }
}