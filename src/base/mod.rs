use Unit;

#[macro_use] mod macros;

pub trait Base: Unit {}

generate_base! {
  name      = Meter,
  longform  = meter,
  shortform = m,
  dimension = Length,
  doc       = "A meter is a unit measuring distance.", 
}

generate_base! {
  name      = Gram,
  longform  = gram,
  shortform = g,
  dimension = Mass,
  doc       = "A gram is a unit measuring weight.", 
}

generate_base! {
  name      = Second,
  longform  = second,
  shortform = s,
  dimension = Time,
  doc       = "A second is a unit measuring time.", 
}

generate_base! {
  name      = Ampere,
  longform  = ampere,
  shortform = A,
  dimension = Current,
  doc       = "An ampere is a unit measuring electrical current.", 
}

generate_base! {
  name      = Kelvin,
  longform  = kelvin,
  shortform = K,
  dimension = Temperature,
  doc       = "A kelvin is a unit measuring thermodynamic temperature.", 
}

generate_base! {
  name      = Mole,
  longform  = mole,
  shortform = mol,
  dimension = Amount,
  doc       = "A mole is a unit measuring the amount of a substance.", 
}

generate_base! {
  name      = Candela,
  longform  = candela,
  shortform = cd,
  dimension = Intensity,
  doc       = "A candela is a unit measuring the amount of luminous intensity.", 
}