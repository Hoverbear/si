use {BigRational, BigInt};
use base::*;
use num::pow::pow;
use num::bigint::Sign::*;

#[macro_use]
mod macros;

pub trait Prefix<B> where B: Base {
  fn factor() -> &'static BigRational;
  fn prefix() -> &'static str;
  fn base(self) -> B;
  fn convert<P>(val: P) -> Self where P: Prefix<B>;
}

fn generate_prefix_factor(exp: isize) -> BigRational {
  let ten = BigInt::new(Plus, vec![10]);
  let one = 1.into();
  if exp >= 0 {
    // (1*10^exp) / 1
    BigRational::new(pow(ten, exp as usize), one)
  } else {
    // 1 / (1*10^exp)
    let exp = exp * -1; // Invert to be positive.
    BigRational::new(one, pow(ten, exp as usize))
  }
}

generate_prefix! {
  name   = Yotta,
  module = yotta,
  prefix = Y,
  factor = 24,
  doc    = "A yotta is 10^24 of the base unit.", 
}

generate_prefix! {
  name   = Zetta,
  module = zetta,
  prefix = Z,
  factor = 21,
  doc    = "A zetta is 10^21 of the base unit.", 
}

generate_prefix! {
  name   = Exa,
  module = exa,
  prefix = E,
  factor = 18,
  doc    = "An exa is 10^18 of the base unit.", 
}

generate_prefix! {
  name   = Peta,
  module = peta,
  prefix = P,
  factor = 15,
  doc    = "A peta is 10^15 of the base unit.", 
}

generate_prefix! {
  name   = Tera,
  module = tera,
  prefix = T,
  factor = 12,
  doc    = "A tera is 10^12 of the base unit.", 
}

generate_prefix! {
  name   = Giga,
  module = giga,
  prefix = G,
  factor = 9,
  doc    = "A giga is 10^9 of the base unit.", 
}

generate_prefix! {
  name   = Mega,
  module = mega,
  prefix = M,
  factor = 6,
  doc    = "A mega is 10^6 of the base unit.", 
}

generate_prefix! {
  name   = Kilo,
  module = kilo,
  prefix = k,
  factor = 3,
  doc    = "A kilo is 10^3 of the base unit.", 
}

generate_prefix! {
  name   = Hecto,
  module = hecto,
  prefix = h,
  factor = 2,
  doc    = "A hecto is 10^2 of the base unit.", 
}

generate_prefix! {
  name   = Deca,
  module = deca,
  prefix = da,
  factor = 1,
  doc    = "A hecto is 10^1 of the base unit.", 
}

generate_prefix! {
  name   = Deci,
  module = deci,
  prefix = d,
  factor = -1,
  doc    = "A deci is 10^-1 of the base unit.", 
}

generate_prefix! {
  name   = Centi,
  module = centi,
  prefix = c,
  factor = -2,
  doc    = "A centi is 10^-2 of the base unit.", 
}

generate_prefix! {
  name   = Milli,
  module = milli,
  prefix = m,
  factor = -3,
  doc    = "A milli is 10^-3 of the base unit.", 
}

generate_prefix! {
  name   = Micro,
  module = micro,
  prefix = μ,
  factor = -6,
  doc    = "A micro is 10^-6 of the base unit.", 
}

generate_prefix! {
  name   = Nano,
  module = nano,
  prefix = n,
  factor = -9,
  doc    = "A nano is 10^-9 of the base unit.", 
}

generate_prefix! {
  name   = Pico,
  module = pico,
  prefix = p,
  factor = -12,
  doc    = "A pico is 10^-12 of the base unit.", 
}

generate_prefix! {
  name   = Femto,
  module = femto,
  prefix = f,
  factor = -15,
  doc    = "A femto is 10^-15 of the base unit.", 
}

generate_prefix! {
  name   = Atto,
  module = atto,
  prefix = a,
  factor = -18,
  doc    = "An atto is 10^-18 of the base unit.", 
}

generate_prefix! {
  name   = Zepto,
  module = zepto,
  prefix = z,
  factor = -21,
  doc    = "A zepto is 10^-21 of the base unit.", 
}

generate_prefix! {
  name   = Yocto,
  module = yocto,
  prefix = y,
  factor = -24,
  doc    = "A yocto is 10^-24 of the base unit.", 
}