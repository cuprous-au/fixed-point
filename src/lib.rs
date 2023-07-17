#![cfg_attr(not(test), no_std)]

use core::{
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};
use serde::{Deserialize, Serialize};

pub mod phases;
pub mod unit;

/// A generic fixed point numeric type implemented as a tuple-struct that serializes cleanly.
/// Type parameter `S` effectively represents the units of the number, e.g. `FixedPoint<Volt>`
/// is a fixed point number representing a voltage.  
///
/// A trait `Spec` implemented for `S` gives the scaling and precision of the fixed point representation.
/// Several types implementing Spec are provided in module `unit`.  
///
/// Traits are implemented for:
///
/// - Conversions to and from Float (f32).
/// - Operations add, substract and scaling (ie a linear space).
/// - Equality and ordering.
/// - Debug, Display and defmt::Format.
/// - Parsing from strings.
/// - Serde.
///
#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize, Ord, PartialOrd)]
pub struct FixedPoint<S>(S);

/// The type of float for scaling and conversion.  
/// This is f32 for support on microcrontrollers.
type Float = f32;

/// The integer representation of a fixed point number.
/// This value is scaled to include the fractional part.
type Repr = i32;

/// The specification of a FixedPoint number.
///
/// The constant Self::SCALE indicates the
/// size of the fractional part.  A value
/// is multiplied by SCALE then truncated to Repr.
///
/// The trait requirements ensure those same traits
/// can be sucessfully derived for every FixedPoint type.
pub trait Spec
where
    Self: Clone + Copy + Eq + PartialEq + Serialize + for<'a> Deserialize<'a>,
{
    const SCALE: Float;
    const PRECISION: usize;
    const SYMBOL: &'static str;
    fn to_repr(self) -> Repr;
    fn from_repr(repr: Repr) -> Self;
}

impl<S> fmt::Debug for FixedPoint<S>
where
    S: Spec,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}/{} {}", self.0.to_repr(), S::SCALE, S::SYMBOL)
    }
}

#[cfg(feature = "defmt")]
impl<S> defmt::Format for FixedPoint<S>
where
    S: Spec,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}/{} {}", self.0.to_repr(), S::SCALE, S::SYMBOL)
    }
}

impl<S> fmt::Display for FixedPoint<S>
where
    S: Spec,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.0.to_repr();
        let sign = if repr < 0 { "-" } else { "" };
        let magn = repr.abs();
        let whole = magn / S::SCALE as Repr;
        let frac = magn % S::SCALE as Repr;
        if frac > 0 {
            write!(f, "{}{}.{:03$}", sign, whole, frac, S::PRECISION)
        } else {
            write!(f, "{}{}", sign, whole)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError;
impl<S> FromStr for FixedPoint<S>
where
    S: Spec,
{
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = text.parse::<Float>() {
            Ok(value.into())
        } else {
            Err(ParseError)
        }
    }
}

impl<S> From<Float> for FixedPoint<S>
where
    S: Spec,
{
    fn from(value: Float) -> Self {
        Self(S::from_repr((value * S::SCALE) as Repr))
    }
}

impl<S> From<FixedPoint<S>> for Float
where
    S: Spec,
{
    fn from(value: FixedPoint<S>) -> Self {
        value.0.to_repr() as Float * (1.0 / S::SCALE)
    }
}

impl From<FixedPoint<unit::Watt>> for FixedPoint<unit::KiloWatt> {
    fn from(value: FixedPoint<unit::Watt>) -> Self {
        let watts: Float = value.into();
        (watts * 0.001).into()
    }
}

impl<S> Add<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lhs = self.0.to_repr();
        let rhs = rhs.0.to_repr();
        Self(S::from_repr(lhs.saturating_add(rhs)))
    }
}

impl<S> Sub<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let lhs = self.0.to_repr();
        let rhs = rhs.0.to_repr();
        Self(S::from_repr(lhs.saturating_sub(rhs)))
    }
}

impl<S> Mul<Float> for FixedPoint<S>
where
    S: Spec,
{
    type Output = Self;

    fn mul(self, rhs: Float) -> Self {
        let lhs: Float = self.into();
        (lhs * rhs).into()
    }
}

impl<S> Div<Float> for FixedPoint<S>
where
    S: Spec,
{
    type Output = Self;

    fn div(self, rhs: Float) -> Self {
        let lhs: Float = self.into();
        (lhs / rhs).into()
    }
}

impl<S> Div<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
{
    type Output = Float;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs: Float = self.into();
        let rhs: Float = rhs.into();
        lhs / rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Example = FixedPoint<unit::Watt>;

    #[test]
    fn it_works() {
        fn json(e: Example) {
            println!("{}", serde_json::to_string_pretty(&e).unwrap());
        }
        let e1: Example = 5.01f32.into();
        let e2 = e1.clone();
        json(e2);
        println!("{e1:?} and {e2}");
        assert_eq!(e1, e2)
    }
}
