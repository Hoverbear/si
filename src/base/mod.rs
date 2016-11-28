use {Unit};

mod meter;
pub use self::meter::Meter;

pub trait Base: Unit {}