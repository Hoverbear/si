use {Unit};
use prefix::Prefix;

mod meter;
pub use self::meter::Meter;

pub trait Base: Unit {
  /// Scale to a prefix.
  fn scale<P>(self) -> P where P: Unit + Prefix<Self>;
}