#![cfg_attr(not(test), no_std)]

use core::{
    fmt,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};
use serde::{Deserialize, Serialize};

pub mod unit;

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixedPoint<S: Spec>(S::Repr);

type Float = f32;

pub trait Spec {
    type Repr;
    const SCALE: Float;
    const PRECISION: usize;
    const SYMBOL: &'static str;
}

impl<S> fmt::Debug for FixedPoint<S>
where
    S: Spec,
    S::Repr: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}/{} {}", self.0, S::SCALE, S::SYMBOL)
    }
}

#[cfg(feature = "defmt")]
impl<S> defmt::Format for FixedPoint<S>
where
    S: Spec,
    S::Repr: defmt::Format,
{
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "{}/{} {}", self.0, S::SCALE, S::SYMBOL)
    }
}

pub trait ConvertRepr {
    fn to_float(self) -> Float;
    fn from_float(value: Float) -> Self;
    fn parts(self, scale: usize) -> (bool, usize, usize);
}

impl ConvertRepr for u32 {
    fn from_float(value: Float) -> Self {
        value as Self
    }
    fn to_float(self) -> Float {
        self as Float
    }
    fn parts(self, scale: usize) -> (bool, usize, usize) {
        (false, self as usize / scale, self as usize % scale)
    }
}

impl ConvertRepr for i32 {
    fn from_float(value: Float) -> Self {
        value as Self
    }
    fn to_float(self) -> Float {
        self as Float
    }
    fn parts(self, scale: usize) -> (bool, usize, usize) {
        if self >= 0 {
            (false, self as usize / scale, self as usize % scale)
        } else {
            (true, (-self) as usize / scale, (-self) as usize % scale)
        }
    }
}

impl<S> fmt::Display for FixedPoint<S>
where
    S: Spec,
    S::Repr: ConvertRepr + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (negative, whole, frac) = S::Repr::parts(self.0.clone(), S::SCALE as usize);
        let sign = if negative { "-" } else { "" };
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
    S::Repr: ConvertRepr,
{
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = text.parse::<Float>() {
            Ok(FixedPoint(S::Repr::from_float(value)))
        } else {
            Err(ParseError)
        }
    }
}

impl<S> From<Float> for FixedPoint<S>
where
    S: Spec,
    S::Repr: ConvertRepr,
{
    fn from(value: Float) -> Self {
        Self(S::Repr::from_float(value * S::SCALE))
    }
}

impl<S> From<FixedPoint<S>> for Float
where
    S: Spec,
    S::Repr: ConvertRepr,
{
    fn from(value: FixedPoint<S>) -> Self {
        value.0.to_float() * (1.0 / S::SCALE)
    }
}

impl From<FixedPoint<unit::Watt>> for FixedPoint<unit::KiloWatt> {
    fn from(value: FixedPoint<unit::Watt>) -> Self {
        FixedPoint(<unit::KiloWatt as Spec>::Repr::from_float(
            <unit::Watt as Spec>::Repr::to_float(value.0) * 0.001,
        ))
    }
}

impl<S> Default for FixedPoint<S>
where
    S: Spec,
    S::Repr: Default,
{
    fn default() -> Self {
        FixedPoint(Default::default())
    }
}

impl<S> Add<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
    S::Repr: Add<S::Repr, Output = S::Repr>,
{
    type Output = FixedPoint<S>;

    fn add(self, rhs: FixedPoint<S>) -> Self::Output {
        FixedPoint(self.0 + rhs.0)
    }
}

impl<S> Sub<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
    S::Repr: Sub<S::Repr, Output = S::Repr>,
{
    type Output = FixedPoint<S>;

    fn sub(self, rhs: FixedPoint<S>) -> Self::Output {
        FixedPoint(self.0 - rhs.0)
    }
}

impl<S> Mul<Float> for FixedPoint<S>
where
    S: Spec,
    S::Repr: ConvertRepr,
{
    type Output = FixedPoint<S>;

    fn mul(self, rhs: Float) -> Self::Output {
        FixedPoint(S::Repr::from_float(S::Repr::to_float(self.0) * rhs))
    }
}

impl<S> Div<Float> for FixedPoint<S>
where
    S: Spec,
    S::Repr: ConvertRepr,
{
    type Output = FixedPoint<S>;

    fn div(self, rhs: Float) -> Self::Output {
        FixedPoint(S::Repr::from_float(S::Repr::to_float(self.0) / rhs))
    }
}

impl<S> Div<FixedPoint<S>> for FixedPoint<S>
where
    S: Spec,
    S::Repr: ConvertRepr,
{
    type Output = Float;

    fn div(self, rhs: FixedPoint<S>) -> Self::Output {
        S::Repr::to_float(self.0) / S::Repr::to_float(rhs.0)
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
