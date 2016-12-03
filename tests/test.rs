extern crate si;

use si::prefix::*;
use si::base::*;

#[test]
fn check_equivalences() {
  assert!(Meter::from(1000) == Meter::from(1000));
  assert!(Meter::from(1000) != Meter::from(1001));
  assert!(Meter::from(1000) == Kilo::<Meter>::from(1));
  assert!(Meter::from(1000) != Kilo::<Meter>::from(1000));
}

#[test]
fn check_operations() {
  assert!(Meter::from(1000) + Kilo::<_>::from(1) == Kilo::<Meter>::from(2));
  assert!(Kilo::<Meter>::from(1) + Meter::from(1000)  == Kilo::<Meter>::from(2));

  assert!(Meter::from(2000) - Kilo::<_>::from(1) == Kilo::<Meter>::from(1));
  assert!(Kilo::<Meter>::from(2) - Meter::from(1000)  == Kilo::<Meter>::from(1));

  assert!(Meter::from(10) * Kilo::<_>::from(1) == Kilo::<Meter>::from(10));
  assert!(Kilo::<Meter>::from(1) * Meter::from(10)  == Kilo::<Meter>::from(10));

  assert!(Meter::from(10_000) / Kilo::<_>::from(1) == Meter::from(10));
  assert!(Kilo::<Meter>::from(100) / Meter::from(10)  == Kilo::<Meter>::from(10));

  // assert!(Meter::from(10_000) / 10 == Kilo::<Meter>::from(1));
  // assert!(Kilo::<Meter>::from(10) / 10  == Kilo::<Meter>::from(1));
}