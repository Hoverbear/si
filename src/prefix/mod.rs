use {BigRational, BigInt};
use base::Base;

#[cfg(test)]mod test;

pub trait Prefix<B> where B: Base {
  fn factor() -> &'static BigInt;
  fn prefix() -> &'static str;
  fn base(self) -> B;
  fn convert<P>(val: P) -> Self where P: Prefix<B>;
}

macro_rules! generate_prefix {
  ($name:ident, $prefix:expr, $factor:expr) => (
    
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct $name<B>(B) where B: Base;
    
    impl<B> Prefix<B> for $name<B> where B: Base {
      fn factor() -> &'static BigInt {
        &*$factor
      }
      fn prefix() -> &'static str {
        $prefix
      }
      fn base(self) -> B {
        let factor = BigRational::from_integer(Self::factor().clone());
        B::from(self.0.value() * factor)
      }
      fn convert<P>(val: P) -> Self where P: Prefix<B> {
        let base = val.base();
        Self::from(base)
      }
    }

    impl<B> From<B> for $name<B> where B: Base {
      fn from(val: B) -> Self {
        let factor = BigRational::from_integer(Self::factor().clone());
        $name(B::from(val.value() / factor))
      }
    }

    impl<B> From<BigInt> for $name<B> where B: Base {
      fn from(val: BigInt) -> Self {
        $name(B::from(BigRational::from_integer(val)))
      }
    }

    impl<B> From<i64> for $name<B> where B: Base {
      fn from(val: i64) -> Self {
        let num = BigInt::from(val);
        let fraction = BigRational::from_integer(num);
        $name(B::from(fraction))
      }
    }

    impl<B> From<BigRational> for $name<B> where B: Base {
      fn from(val: BigRational) -> Self {
        $name(B::from(val))
      }
    }
  )
}

lazy_static! {
  static ref YOTTA: BigInt = BigInt::parse_bytes(b"1000000000000000000000000", 10).unwrap();
  static ref ZETTA: BigInt = BigInt::parse_bytes(b"1000000000000000000000", 10).unwrap();
  static ref EXA:   BigInt = BigInt::parse_bytes(b"1000000000000000000", 10).unwrap();
  static ref PETA:  BigInt = BigInt::parse_bytes(b"1000000000000000", 10).unwrap();
  static ref TERA:  BigInt = BigInt::parse_bytes(b"1000000000000", 10).unwrap();
  static ref GIGA:  BigInt = BigInt::parse_bytes(b"1000000000", 10).unwrap();
  static ref MEGA:  BigInt = BigInt::parse_bytes(b"1000000", 10).unwrap();
  static ref KILO:  BigInt = BigInt::parse_bytes(b"1000", 10).unwrap();
  static ref HECTO: BigInt = BigInt::parse_bytes(b"100", 10).unwrap();
  static ref DEKA:  BigInt = BigInt::parse_bytes(b"10", 10).unwrap();

  static ref DECI:  BigInt = BigInt::parse_bytes(b"-10", 10).unwrap();
  static ref CENTI: BigInt = BigInt::parse_bytes(b"-100", 10).unwrap();
  static ref MILLI: BigInt = BigInt::parse_bytes(b"-1000", 10).unwrap();
  static ref MICRO: BigInt = BigInt::parse_bytes(b"-1000000", 10).unwrap();
  static ref NANO:  BigInt = BigInt::parse_bytes(b"-1000000000", 10).unwrap();
  static ref PICO:  BigInt = BigInt::parse_bytes(b"-1000000000000", 10).unwrap();
  static ref FEMTO: BigInt = BigInt::parse_bytes(b"-1000000000000000", 10).unwrap();
  static ref ATTO:  BigInt = BigInt::parse_bytes(b"-1000000000000000000", 10).unwrap();
  static ref ZEPTO: BigInt = BigInt::parse_bytes(b"-1000000000000000000000", 10).unwrap();
  static ref YOCTO: BigInt = BigInt::parse_bytes(b"-1000000000000000000000000", 10).unwrap();
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