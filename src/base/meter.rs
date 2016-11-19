use {BigRational, BigInt};
use base::Base;
#[cfg(test)] use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Meter(BigRational);

impl Base for Meter {
  fn new(val: BigRational) -> Self {
    Meter(val)
  }
  fn unit(&self) -> &'static str {
    "m"
  }
  fn value(self) -> BigRational {
    self.0
  }
}

impl From<BigInt> for Meter {
  fn from(val: BigInt) -> Self {
    let fraction = BigRational::from_integer(val); 
    Meter(fraction)
  }
}

impl From<i64> for Meter {
  fn from(val: i64) -> Self {
    let num = BigInt::from(val);
    let fraction = BigRational::from_integer(num); 
    Meter(fraction)
  }
}

impl From<BigRational> for Meter {
  fn from(val: BigRational) -> Self {
    Meter(val)
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