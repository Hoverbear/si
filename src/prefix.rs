use {Fraction, Number};
use base::Base;

pub trait Prefix<B> where B: Base {
  fn factor() -> &'static Number;
  fn base(self) -> B;
  fn convert<P>(val: P) -> Self where P: Prefix<B>;
}

macro_rules! generate_prefix {
  ($name:ident, $prefix:expr, $factor:expr) => (
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct $name<B>(B) where B: Base;
    
    impl<B> Prefix<B> for $name<B> where B: Base {
      fn factor() -> &'static Number {
        &*$factor
      }
      fn base(self) -> B {
        let factor = Fraction::from_integer(Self::factor().clone());
        B::from(self.0.value() * factor)
      }
      fn convert<P>(val: P) -> Self where P: Prefix<B> {
        let base = val.base();
        Self::from(base)
      }
    }

    impl<B> From<B> for $name<B> where B: Base {
      fn from(val: B) -> Self {
        let factor = Fraction::from_integer(Self::factor().clone());
        $name(B::from(val.value() / factor))
      }
    }

    impl<B> From<Number> for $name<B> where B: Base {
      fn from(val: Number) -> Self {
        $name(B::from(Fraction::from_integer(val)))
      }
    }

    impl<B> From<i64> for $name<B> where B: Base {
      fn from(val: i64) -> Self {
        let num = Number::from(val);
        let fraction = Fraction::from_integer(num);
        $name(B::from(fraction))
      }
    }

    impl<B> From<Fraction> for $name<B> where B: Base {
      fn from(val: Fraction) -> Self {
        $name(B::from(val))
      }
    }
  )
}

lazy_static! {
  static ref YOTTA: Number = Number::parse_bytes(b"1_000_000_000_000_000_000_000_000", 10).unwrap();
  static ref ZETTA: Number = Number::parse_bytes(b"1_000_000_000_000_000_000_000", 10).unwrap();
  static ref EXA:   Number = Number::parse_bytes(b"1_000_000_000_000_000_000", 10).unwrap();
  static ref PETA:  Number = Number::parse_bytes(b"1_000_000_000_000_000", 10).unwrap();
  static ref TERA:  Number = Number::parse_bytes(b"1_000_000_000_000", 10).unwrap();
  static ref GIGA:  Number = Number::parse_bytes(b"1_000_000_000", 10).unwrap();
  static ref MEGA:  Number = Number::parse_bytes(b"1_000_000", 10).unwrap();
  static ref KILO:  Number = Number::parse_bytes(b"1_000", 10).unwrap();
  static ref HECTO: Number = Number::parse_bytes(b"100", 10).unwrap();
  static ref DEKA:  Number = Number::parse_bytes(b"10", 10).unwrap();

  static ref DECI:  Number = Number::parse_bytes(b"-10", 10).unwrap();
  static ref CENTI: Number = Number::parse_bytes(b"-100", 10).unwrap();
  static ref MILLI: Number = Number::parse_bytes(b"-1_000", 10).unwrap();
  static ref MICRO: Number = Number::parse_bytes(b"-1_000_000", 10).unwrap();
  static ref NANO:  Number = Number::parse_bytes(b"-1_000_000_000", 10).unwrap();
  static ref PICO:  Number = Number::parse_bytes(b"-1_000_000_000_000", 10).unwrap();
  static ref FEMTO: Number = Number::parse_bytes(b"-1_000_000_000_000_000", 10).unwrap();
  static ref ATTO:  Number = Number::parse_bytes(b"-1_000_000_000_000_000_000", 10).unwrap();
  static ref ZEPTO: Number = Number::parse_bytes(b"-1_000_000_000_000_000_000_000", 10).unwrap();
  static ref YOCTO: Number = Number::parse_bytes(b"-1_000_000_000_000_000_000_000_000", 10).unwrap();
}

generate_prefix!(Yotta, "Y", YOTTA);
generate_prefix!(Zetta, "Z", ZETTA);
generate_prefix!(Exa,   "E", EXA);
generate_prefix!(Peta,  "P", PETA);
generate_prefix!(Tera,  "T", TERA);
generate_prefix!(Giga,  "G", GIGA);
generate_prefix!(Mega,  "M", MEGA);
generate_prefix!(Kilo,  "k", KILO);
generate_prefix!(Hecto, "h", HECTO);
generate_prefix!(Deka,  "da", DEKA);

generate_prefix!(Deci,  "d", DECI);  
generate_prefix!(Centi, "c", CENTI); 
generate_prefix!(Milli, "m", MILLI); 
generate_prefix!(Micro, "μ", MICRO); 
generate_prefix!(Nano,  "n", NANO);  
generate_prefix!(Pico,  "p", PICO);  
generate_prefix!(Femto, "f", FEMTO); 
generate_prefix!(Atto,  "a", ATTO);  
generate_prefix!(Zepto, "z", ZEPTO); 
generate_prefix!(Yocto, "y", YOCTO); 

#[cfg(test)]
mod tests {
  use base::*;
  use prefix::*;
  use {Number, Fraction};


}