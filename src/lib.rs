extern crate num;

pub mod prefix;
pub mod unit;

pub type Number = i64;
pub type Fraction = num::rational::Ratio<Number>;

