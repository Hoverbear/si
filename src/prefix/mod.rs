use {Fraction, Number};
use unit::Unit;

pub trait Prefix<U> where U: Unit {
  fn to_base(self) -> U;
}

macro_rules! generate_prefix {
  ($prefix:ident,$scale:expr) => (
    pub struct $prefix<U>(U) where U: Unit;
    
    impl<U> Prefix<U> for $prefix<U> where U: Unit {
      fn to_base(self) -> U {
        let scale = Fraction::from_integer(10_i64).pow($scale);
        U::from(self.0.take() * scale)
      }
    }

    impl<U> From<Number> for $prefix<U> where U: Unit {
      fn from(val: Number) -> Self {
        $prefix(U::from(Fraction::from_integer(val)))
      }
    }

    impl<U> From<Fraction> for $prefix<U> where U: Unit {
      fn from(val: Fraction) -> Self {
        $prefix(U::from(val))
      }
    }
  )
}

generate_prefix!(Yotta, 24);
generate_prefix!(Zetta, 21);
generate_prefix!(Exa, 18);
generate_prefix!(Peta, 15);
generate_prefix!(Tera, 12);
generate_prefix!(Giga, 9);
generate_prefix!(Mega, 6);
generate_prefix!(Kilo, 3);
generate_prefix!(Hecto, 2);
generate_prefix!(Deka, 1);
generate_prefix!(Deci, -1);
generate_prefix!(Centi, -2);
generate_prefix!(Milli, -3);
generate_prefix!(Micro, -6);
generate_prefix!(Nano, -9);
generate_prefix!(Pico, -12);
generate_prefix!(Femto, -15);
generate_prefix!(Atto, -18);
generate_prefix!(Zepto, -21);
generate_prefix!(Yocto, -24);