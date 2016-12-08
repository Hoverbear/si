extern crate si;

use si::prefix::*;
use si::base::*;

#[test]
fn check_equivalences() {
  assert!(Meter::from(1_000) == Meter::from(1_000));
  assert!(Meter::from(1_000) != Meter::from(1_001));
  assert!(Meter::from(1_000) == Kilo::<Meter>::from(1));
  assert!(Meter::from(1_000) != Kilo::<Meter>::from(1_000));
}

#[test]
fn check_addition() {
  assert!(Meter::from(1_000) + Kilo::<_>::from(1) == Kilo::<Meter>::from(2));
  assert!(Kilo::<Meter>::from(1) + Meter::from(1_000)  == Kilo::<Meter>::from(2));
}

#[test]
fn check_subtraction() {
  assert!(Meter::from(2000) - Kilo::<_>::from(1) == Kilo::<Meter>::from(1));
  assert!(Kilo::<Meter>::from(2) - Meter::from(1_000)  == Kilo::<Meter>::from(1));
}

#[test]
fn check_multiplication() {
  assert!(Meter::from(1_000) * 10 == Kilo::<Meter>::from(10));
  assert!(Kilo::<Meter>::from(1) * 10  == Kilo::<Meter>::from(10));
  assert!(Kilo::<Meter>::from(1) * 10  == Meter::from(10_000));
}

#[test]
fn check_division() {
  assert!(Meter::from(10_000) / 10 == Kilo::<Meter>::from(1));
  assert!(Kilo::<Meter>::from(10) / 10  == Kilo::<Meter>::from(1));
  assert!(Kilo::<Meter>::from(10) / 10  == Meter::from(1_000));
}