use {BigRational, BigInt, Unit};
use super::Base;
use prefix::Prefix;
#[cfg(test)] use quickcheck::{Arbitrary, Gen};

trait Distance {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meter {
  value: BigRational
}

impl Unit for Meter {
  fn new(value: BigRational) -> Self {
    Meter { 
      value: value 
    }
  }
  fn shortform() -> &'static str {
    "m"
  }
  fn longform() -> &'static str {
    "meter"
  }
  fn value(self) -> BigRational {
    self.value
  }
}

impl Base for Meter {
  fn scale<P>(self) -> P where P: Unit + Prefix<Meter> {
    unimplemented!();
  }
}

impl From<BigInt> for Meter {
  fn from(val: BigInt) -> Self {
    let fraction = BigRational::from_integer(val); 
    Meter::new(fraction)
  }
}

impl From<i64> for Meter {
  fn from(val: i64) -> Self {
    let num = BigInt::from(val);
    let fraction = BigRational::from_integer(num); 
    Meter::new(fraction)
  }
}

impl From<BigRational> for Meter {
  fn from(val: BigRational) -> Self {
    Meter::new(val)
  }
}

#[cfg(test)]
impl Arbitrary for Meter {
  fn arbitrary<G: Gen>(g: &mut G) -> Self {
    let (numerator, denominator) = (g.gen::<i64>(), g.gen::<i64>());
    let denominator = if denominator == 0 { 1 } else { denominator }; // The denominator cannot be zero.
    
    let (numerator, denominator) = (BigInt::from(numerator), BigInt::from(denominator));
    let rational = BigRational::new(numerator, denominator);
    Meter::new(rational)
  }
}