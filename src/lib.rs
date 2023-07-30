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
/// Type parameter `R` is the representation of the number on the wire and in memory.  
///
/// A trait `Spec` implemented for `R` gives the scaling and precision of the number.
/// Several types implementing Spec are provided in module `unit`.  
///
/// For example, `Volt` defines an i32 representation of voltage.  `impl Spec for Volt`
/// gives the precision of this representation as one decimal place.
///
/// The traits defined on FixedPoint<R> provide all representations with:
///
/// - Conversions to and from Float (f32).
/// - Operations add, substract and scaling (ie a linear space).
/// - Equality and ordering.
/// - Debug, Display and defmt::Format.
/// - Parsing from strings.
/// - Serde.
///
#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize, Ord, PartialOrd)]
pub struct FixedPoint<R>(R);

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
/// is multiplied by SCALE then truncated to Fixed.
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
    fn from_fixed(fixed: Fixed) -> Self;
}

impl<R> FixedPoint<R>
where
    R: Spec,
{
    /// Construct from a float
    pub fn new(value: Float) -> Self {
        value.into()
    }

    /// Conversion to a float.
    pub fn to_float(self) -> Float {
        self.into()
    }

    /// Construct from a whole number, no fractional part.
    /// Specialised to a zero cost operation for SCALE=1.0
    pub fn new0(value: Fixed) -> Self {
        if R::SCALE == 1.0 {
            Self(R::from_fixed(value))
        } else {
            (value as Float).into()
        }
    }

    /// Construct from a integer interpreted at 10x scale.
    /// Specialised to a zero cost operation for SCALE=10.0
    pub fn new1(value: Fixed) -> Self {
        if R::SCALE == 10.0 {
            Self(R::from_fixed(value))
        } else {
            (value as Float * 0.1).into()
        }
    }

    /// Construct from a integer interpreted at 100x scale.
    /// Specialised to a zero cost operation for SCALE=100.0
    pub fn new2(value: Fixed) -> Self {
        if R::SCALE == 100.0 {
            Self(R::from_fixed(value))
        } else {
            (value as Float * 0.01).into()
        }
    }

    /// Construct from a integer interpreted at 1000x scale.
    /// Specialised to a zero cost operation for SCALE=1000.0
    pub fn new3(value: Fixed) -> Self {
        if R::SCALE == 1000.0 {
            Self(R::from_fixed(value))
        } else {
            (value as Float * 0.001).into()
        }
    }
}

impl<R> fmt::Debug for FixedPoint<R>
where
    R: Spec,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}/{} {}", self.0.to_fixed(), R::SCALE, R::SYMBOL)
    }
}

#[cfg(feature = "defmt")]
impl<R> defmt::Format for FixedPoint<R>
where
    R: Spec,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}/{} {}", self.0.to_fixed(), R::SCALE, R::SYMBOL)
    }
}

impl<R> fmt::Display for FixedPoint<R>
where
    R: Spec,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.0.to_fixed();
        let sign = if repr < 0 { "-" } else { "" };
        let magn = repr.abs();
        let whole = magn / R::SCALE as Fixed;
        let frac = magn % R::SCALE as Fixed;
        if frac > 0 {
            write!(f, "{}{}.{:03$}", sign, whole, frac, R::PRECISION)
        } else {
            write!(f, "{}{}", sign, whole)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError;
impl<R> FromStr for FixedPoint<R>
where
    R: Spec,
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

impl<R> From<Float> for FixedPoint<R>
where
    R: Spec,
{
    fn from(value: Float) -> Self {
        Self(R::from_fixed((value * R::SCALE) as Fixed))
    }
}

impl<R> From<FixedPoint<R>> for Float
where
    R: Spec,
{
    fn from(value: FixedPoint<R>) -> Self {
        value.0.to_fixed() as Float * (1.0 / R::SCALE)
    }
}

impl From<FixedPoint<unit::Watt>> for FixedPoint<unit::KiloWatt> {
    fn from(value: FixedPoint<unit::Watt>) -> Self {
        let watts: Float = value.into();
        (watts * 0.001).into()
    }
}

impl<R> Add<FixedPoint<R>> for FixedPoint<R>
where
    R: Spec,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let lhs = self.0.to_fixed();
        let rhs = rhs.0.to_fixed();
        Self(R::from_fixed(lhs.saturating_add(rhs)))
    }
}

impl<R> Sub<FixedPoint<R>> for FixedPoint<R>
where
    R: Spec,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let lhs = self.0.to_fixed();
        let rhs = rhs.0.to_fixed();
        Self(R::from_fixed(lhs.saturating_sub(rhs)))
    }
}

impl<R> Mul<Float> for FixedPoint<R>
where
    R: Spec,
{
    type Output = Self;

    fn mul(self, rhs: Float) -> Self {
        let lhs: Float = self.into();
        (lhs * rhs).into()
    }
}

impl<R> Div<Float> for FixedPoint<R>
where
    R: Spec,
{
    type Output = Self;

    fn div(self, rhs: Float) -> Self {
        let lhs: Float = self.into();
        (lhs / rhs).into()
    }
}

impl<R> Div<FixedPoint<R>> for FixedPoint<R>
where
    R: Spec,
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
