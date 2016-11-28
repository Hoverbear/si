use {Unit};

#[macro_use]
mod macros;
// pub use self::meter::Meter;

pub trait Base: Unit {}

generate_base! {
  name      = Meter,
  longform  = meter,
  shortform = m,
  dimension = Distance,
  doc       = "A meter is a unit measuring distance.", 
}

generate_base! {
  name      = Gram,
  longform  = gram,
  shortform = g,
  dimension = Weight,
  doc       = "A gram is a unit measuring weight.", 
}