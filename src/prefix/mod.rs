use {BigRational, BigInt, Unit};
use base::Base;
use num::pow::pow;
use num::bigint::Sign::*;
use std::ops::Neg;

#[macro_use]
mod macros;

pub trait Prefix<B>: Unit where B: Base {
  // The factor amount. Eg Kilo is 1*10^3, or 1000.
  fn factor() -> &'static BigRational;
  /// Scale to a prefix.
  fn scale<P>(value: P) -> Self where P: Unit + Prefix<B>;
  fn base(self) -> B;
}

fn generate_prefix_factor(exp: isize) -> BigRational {
  let ten = BigInt::new(Plus, vec![10]);
  let one = 1.into();
  if exp >= 0 {
    // (1*10^exp) / 1
    BigRational::new(pow(ten, exp as usize), one)
  } else {
    // 1 / (1*10^exp)
    let exp = exp.neg(); // Invert to be positive.
    BigRational::new(one, pow(ten, exp as usize))
  }
}

// generate_prefix! {
//   name   = Yotta,
//   module = yotta,
//   shortform = Y,
//   factor = 24,
//   doc    = "A yotta is 10^24 of the base unit.", 
// }

// generate_prefix! {
//   name   = Zetta,
//   module = zetta,
//   shortform = Z,
//   factor = 21,
//   doc    = "A zetta is 10^21 of the base unit.", 
// }

// generate_prefix! {
//   name   = Exa,
//   module = exa,
//   shortform = E,
//   factor = 18,
//   doc    = "An exa is 10^18 of the base unit.", 
// }

// generate_prefix! {
//   name   = Peta,
//   module = peta,
//   shortform = P,
//   factor = 15,
//   doc    = "A peta is 10^15 of the base unit.", 
// }

// generate_prefix! {
//   name   = Tera,
//   module = tera,
//   shortform = T,
//   factor = 12,
//   doc    = "A tera is 10^12 of the base unit.", 
// }

// generate_prefix! {
//   name   = Giga,
//   module = giga,
//   shortform = G,
//   factor = 9,
//   doc    = "A giga is 10^9 of the base unit.", 
// }

// generate_prefix! {
//   name   = Mega,
//   module = mega,
//   shortform = M,
//   factor = 6,
//   doc    = "A mega is 10^6 of the base unit.", 
// }

generate_prefix! {
  name   = Kilo,
  module = kilo,
  shortform = k,
  factor = 3,
  doc    = "A kilo is 10^3 of the base unit.", 
}

// generate_prefix! {
//   name   = Hecto,
//   module = hecto,
//   shortform = h,
//   factor = 2,
//   doc    = "A hecto is 10^2 of the base unit.", 
// }

// generate_prefix! {
//   name   = Deca,
//   module = deca,
//   shortform = da,
//   factor = 1,
//   doc    = "A hecto is 10^1 of the base unit.", 
// }

// generate_prefix! {
//   name   = Deci,
//   module = deci,
//   shortform = d,
//   factor = -1,
//   doc    = "A deci is 10^-1 of the base unit.", 
// }

// generate_prefix! {
//   name   = Centi,
//   module = centi,
//   shortform = c,
//   factor = -2,
//   doc    = "A centi is 10^-2 of the base unit.", 
// }

// generate_prefix! {
//   name   = Milli,
//   module = milli,
//   shortform = m,
//   factor = -3,
//   doc    = "A milli is 10^-3 of the base unit.", 
// }

// generate_prefix! {
//   name   = Micro,
//   module = micro,
//   shortform = μ,
//   factor = -6,
//   doc    = "A micro is 10^-6 of the base unit.", 
// }

// generate_prefix! {
//   name   = Nano,
//   module = nano,
//   shortform = n,
//   factor = -9,
//   doc    = "A nano is 10^-9 of the base unit.", 
// }

// generate_prefix! {
//   name   = Pico,
//   module = pico,
//   shortform = p,
//   factor = -12,
//   doc    = "A pico is 10^-12 of the base unit.", 
// }

// generate_prefix! {
//   name   = Femto,
//   module = femto,
//   shortform = f,
//   factor = -15,
//   doc    = "A femto is 10^-15 of the base unit.", 
// }

// generate_prefix! {
//   name   = Atto,
//   module = atto,
//   shortform = a,
//   factor = -18,
//   doc    = "An atto is 10^-18 of the base unit.", 
// }

// generate_prefix! {
//   name   = Zepto,
//   module = zepto,
//   shortform = z,
//   factor = -21,
//   doc    = "A zepto is 10^-21 of the base unit.", 
// }

// generate_prefix! {
//   name   = Yocto,
//   module = yocto,
//   shortform = y,
//   factor = -24,
//   doc    = "A yocto is 10^-24 of the base unit.", 
// }