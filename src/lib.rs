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
/// Type parameter `S` is the representation of the number on the wire and in memory.  
///
/// A trait `Spec` implemented for `S` gives the scaling and precision of the number.
/// Several types implementing Spec are provided in module `unit`.  
///
/// For example, `Volt` defines an i32 representation of voltage.  `impl Spec for Volt`
/// gives the precision of this representation as one decimal place.
///
/// Additional traits are implemented for:
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
/// This is f32 for support on microcontrollers.
type Float = f32;

/// A common integer type for all fixed point representations.
/// This i32 to fit in a microcontroller register.
/// It follows that all representations are less than
/// 32 bits in size (signed) or 31 bits (unsigned).
type Fixed = i32;

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
    fn to_fixed(self) -> Fixed;
    fn from_fixed(repr: Fixed) -> Self;
}

impl<S> fmt::Debug for FixedPoint<S>
where
    S: Spec,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}/{} {}", self.0.to_fixed(), S::SCALE, S::SYMBOL)
    }
}

#[cfg(feature = "defmt")]
impl<S> defmt::Format for FixedPoint<S>
where
    S: Spec,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}/{} {}", self.0.to_fixed(), S::SCALE, S::SYMBOL)
    }
}

impl<S> fmt::Display for FixedPoint<S>
where
    S: Spec,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.0.to_fixed();
        let sign = if repr < 0 { "-" } else { "" };
        let magn = repr.abs();
        let whole = magn / S::SCALE as Fixed;
        let frac = magn % S::SCALE as Fixed;
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
        Self(S::from_fixed((value * S::SCALE) as Fixed))
    }
}

impl<S> From<FixedPoint<S>> for Float
where
    S: Spec,
{
    fn from(value: FixedPoint<S>) -> Self {
        value.0.to_fixed() as Float * (1.0 / S::SCALE)
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
        let lhs = self.0.to_fixed();
        let rhs = rhs.0.to_fixed();
        Self(S::from_fixed(lhs.saturating_add(rhs)))
    }
}

impl<S> Sub<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let lhs = self.0.to_fixed();
        let rhs = rhs.0.to_fixed();
        Self(S::from_fixed(lhs.saturating_sub(rhs)))
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
    fn cloning_and_equality() {
        let e1: Example = 5.01f32.into();
        let e2 = e1.clone();
        assert_eq!(e1, e2);
    }

    #[test]
    fn ordering() {
        let e1: Example = 5.01f32.into();
        let e2: Example = 5.11f32.into();
        assert!(e2 > e1);
    }

    #[test]
    fn serialization() {
        let e1: Example = 5.01f32.into();
        assert_eq!(serde_json::to_string(&e1).unwrap(), "501");
    }

    #[test]
    fn display() {
        let e1: Example = 5.01f32.into();
        assert_eq!(format!("{}", e1), "5.01");
        assert_eq!(format!("{:?}", e1), "501/100 W");
        assert_eq!(format!("{}", Example::default() - e1), "-5.01");
    }
}
