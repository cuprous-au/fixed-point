#![cfg_attr(not(test), no_std)]

use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
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
        // special treat ment for these decimal point scales
        if R::SCALE == 10.0 || R::SCALE == 100.0 || R::SCALE == 1000.0 {
            let fixed = self.0.to_fixed();
            let sign = if fixed < 0 { "-" } else { "" };
            let magn = fixed.abs();
            let whole = magn / R::SCALE as Fixed;
            let frac = magn % R::SCALE as Fixed;

            if frac > 0 {
                if R::SCALE == 10.0 {
                    write!(f, "{sign}{whole}.{frac}")
                } else if R::SCALE == 100.0 {
                    if frac % 10 == 0 {
                        write!(f, "{sign}{whole}.{}", frac / 10)
                    } else {
                        write!(f, "{sign}{whole}.{:02}", frac)
                    }
                } else if frac % 100 == 0 {
                    write!(f, "{sign}{whole}.{}", frac / 100)
                } else if frac % 10 == 0 {
                    write!(f, "{sign}{whole}.{:02}", frac / 10)
                } else {
                    write!(f, "{sign}{whole}.{:03}", frac)
                }
            } else {
                write!(f, "{sign}{whole}")
            }
        } else if R::SCALE == 1.0 {
            // scale 1.0 is displayed without conversions
            write!(f, "{}", self.0.to_fixed())
        } else {
            // Every other scale including non decimal
            write!(f, "{}", self.to_float())
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

impl<R> AddAssign<FixedPoint<R>> for FixedPoint<R>
where
    R: Spec,
{
    fn add_assign(&mut self, rhs: FixedPoint<R>) {
        *self = *self + rhs;
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

impl<R> SubAssign<FixedPoint<R>> for FixedPoint<R>
where
    R: Spec,
{
    fn sub_assign(&mut self, rhs: FixedPoint<R>) {
        *self = *self - rhs;
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

impl<R> MulAssign<Float> for FixedPoint<R>
where
    R: Spec,
{
    fn mul_assign(&mut self, rhs: Float) {
        *self = *self * rhs;
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

impl<R> DivAssign<Float> for FixedPoint<R>
where
    R: Spec,
{
    fn div_assign(&mut self, rhs: Float) {
        *self = *self / rhs;
    }
}

// constants and const constructors for the representations defined in unit
impl FixedPoint<unit::Amp> {
    pub const ZERO: Self = Self::with_fix1(0);

    /// Construct from a integer interpreted at 10x scale.
    pub const fn with_fix1(value: Fixed) -> Self {
        Self(unit::Amp(value))
    }

    /// Extract an integer at 10x scale
    pub const fn fix1(self) -> Fixed {
        self.0 .0
    }
}

impl FixedPoint<unit::Volt> {
    pub const ZERO: Self = Self::with_fix1(0);

    /// Construct from a integer interpreted at 10x scale.
    pub const fn with_fix1(value: Fixed) -> Self {
        Self(unit::Volt(value))
    }

    /// Extract an integer at 10x scale
    pub const fn fix1(self) -> Fixed {
        self.0 .0
    }
}

impl FixedPoint<unit::PreciseVolt> {
    pub const ZERO: Self = Self::with_fix3(0);

    /// Construct from a integer interpreted at 1000x scale.
    pub const fn with_fix3(value: Fixed) -> Self {
        Self(unit::PreciseVolt(value))
    }

    /// Extract an integer at 1000x scale
    pub const fn fix3(self) -> Fixed {
        self.0 .0
    }
}

impl FixedPoint<unit::Watt> {
    pub const ZERO: Self = Self::with_fix0(0);

    /// Construct from a integer interpreted at 1x scale.
    pub const fn with_fix0(value: Fixed) -> Self {
        Self(unit::Watt(value))
    }

    /// Extract an integer at 100x scale
    pub const fn fix0(self) -> Fixed {
        self.0 .0
    }
}

impl FixedPoint<unit::KiloWatt> {
    pub const ZERO: Self = Self::with_fix1(0);

    /// Construct from a integer interpreted at 10x scale.
    pub const fn with_fix1(value: Fixed) -> Self {
        Self(unit::KiloWatt(value))
    }

    /// Extract an integer at 10x scale
    pub const fn fix1(self) -> Fixed {
        self.0 .0
    }
}

impl FixedPoint<unit::KiloWattHour> {
    pub const ZERO: Self = Self::with_fix2(0);

    /// Construct from a integer interpreted at 100x scale.
    pub const fn with_fix2(value: Fixed) -> Self {
        Self(unit::KiloWattHour(value))
    }

    /// Extract an integer at 100x scale
    pub const fn fix2(self) -> Fixed {
        self.0 .0
    }
}

// special case energy conversion
impl From<FixedPoint<unit::Watt>> for FixedPoint<unit::KiloWatt> {
    fn from(value: FixedPoint<unit::Watt>) -> Self {
        let watts: Float = value.into();
        (watts * 0.001).into()
    }
}

impl FixedPoint<unit::Watt> {
    /// Convenience function helps handle and display larger energy values
    pub fn kwh(self) -> FixedPoint<unit::KiloWatt> {
        self.into()
    }
}

impl FixedPoint<unit::Celsius> {
    pub const ZERO: Self = Self::with_fix2(0);

    /// Construct from a integer interpreted at 10x scale.
    pub const fn with_fix2(value: Fixed) -> Self {
        Self(unit::Celsius(value))
    }

    /// Extract an integer at 10x scale
    pub const fn fix2(self) -> Fixed {
        self.0 .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Voltage = FixedPoint<unit::Volt>;
    type Current = FixedPoint<unit::Amp>;
    type Energy = FixedPoint<unit::KiloWattHour>;
    type Power = FixedPoint<unit::Watt>;
    type LowVoltage = FixedPoint<unit::PreciseVolt>;

    #[test]
    fn cloning_and_equality() {
        let e1: Energy = 5.01f32.into();
        #[allow(clippy::clone_on_copy)]
        let e2 = e1.clone();
        assert_eq!(e1, e2);
    }

    #[test]
    fn ordering() {
        let e1: Energy = 5.01f32.into();
        let e2: Energy = 5.11f32.into();
        assert!(e2 > e1);
    }

    #[test]
    fn serialization() {
        let e1: Energy = 5.01f32.into();
        assert_eq!(serde_json::to_string(&e1).unwrap(), "501");
    }

    #[test]
    fn construction() {
        assert_eq!(LowVoltage::new(1.705).to_float(), 1.705);
        assert_eq!(LowVoltage::with_fix3(1705).to_float(), 1.705);
    }

    #[test]
    fn fixing() {
        assert_eq!(LowVoltage::new(1.705).fix3(), 1705);
        assert_eq!(Energy::new(1.705).fix2(), 170);
        assert_eq!(Current::new(1.705).fix1(), 17);
    }

    #[test]
    fn display() {
        let e1: Energy = 5.01f32.into();
        assert_eq!(format!("{}", e1), "5.01");
        assert_eq!(format!("{:?}", e1), "501/100 kWh");
        assert_eq!(format!("{}", Energy::ZERO - e1), "-5.01");
    }

    #[test]
    fn test_display_voltage() {
        assert_eq!(Voltage::with_fix1(2456).to_string(), "245.6");
        assert_eq!(Voltage::with_fix1(325).to_string(), "32.5");
        assert_eq!(Voltage::with_fix1(320).to_string(), "32");
        assert_eq!(Voltage::with_fix1(0).to_string(), "0");
        assert_eq!(Voltage::with_fix1(1).to_string(), "0.1");
        assert_eq!(Voltage::with_fix1(10).to_string(), "1");
    }

    #[test]
    fn test_display_current() {
        assert_eq!(Current::with_fix1(325).to_string(), "32.5");
        assert_eq!(Current::with_fix1(320).to_string(), "32");
        assert_eq!(Current::with_fix1(0).to_string(), "0");
        assert_eq!(Current::with_fix1(1).to_string(), "0.1");
        assert_eq!(Current::with_fix1(10).to_string(), "1");

        assert_eq!(Current::with_fix1(-325).to_string(), "-32.5");
        assert_eq!(Current::with_fix1(-320).to_string(), "-32");
        assert_eq!(Current::with_fix1(0).to_string(), "0");
        assert_eq!(Current::with_fix1(-1).to_string(), "-0.1");
        assert_eq!(Current::with_fix1(-10).to_string(), "-1");
    }

    #[test]
    fn test_parse_current() {
        assert_eq!("32.5".parse(), Ok(Current::with_fix1(325)));
        assert_eq!("32".parse(), Ok(Current::with_fix1(320)));
        assert_eq!("32.54".parse(), Ok(Current::with_fix1(325)));
        assert_eq!("0.5".parse(), Ok(Current::with_fix1(5)));
        assert_eq!("".parse::<Current>(), Err(ParseError));
        assert_eq!(".1".parse::<Current>(), Ok(Current::with_fix1(1)));
        assert_eq!("1.".parse::<Current>(), Ok(Current::with_fix1(10)));

        assert_eq!("-32.5".parse(), Ok(Current::with_fix1(-325)));
        assert_eq!("-32.54".parse(), Ok(Current::with_fix1(-325)));
        assert_eq!("-32".parse(), Ok(Current::with_fix1(-320)));
        assert_eq!("-0.5".parse(), Ok(Current::with_fix1(-5)));
        assert_eq!("0.".parse(), Ok(Current::with_fix1(0)));
    }

    #[test]
    fn test_display_energy() {
        assert_eq!(Energy::with_fix2(305).to_string(), "3.05");
        assert_eq!(Energy::with_fix2(310).to_string(), "3.1");
        assert_eq!(Energy::with_fix2(325).to_string(), "3.25");
        assert_eq!(Energy::with_fix2(300).to_string(), "3");
        assert_eq!(Energy::with_fix2(5).to_string(), "0.05");
    }

    #[test]
    fn test_display_power() {
        assert_eq!(Power::with_fix0(5).to_string(), "5");
        assert_eq!(Power::with_fix0(305).to_string(), "305");
        assert_eq!(Power::with_fix0(30).to_string(), "30");
        assert_eq!(Power::with_fix0(3102).kwh().to_string(), "3.1");
        assert_eq!(Power::with_fix0(1).kwh().to_string(), "0");
        assert_eq!(Power::with_fix0(114).kwh().to_string(), "0.1");
        assert_eq!(Power::with_fix0(14).kwh().to_string(), "0");
    }

    #[test]
    fn test_display_negative_numbers() {
        assert_eq!(Energy::with_fix2(-5).to_string(), "-0.05");
        assert_eq!(Energy::with_fix2(-305).to_string(), "-3.05");
        assert_eq!(Energy::with_fix2(-325).to_string(), "-3.25");
        assert_eq!(Energy::with_fix2(-300).to_string(), "-3");
        assert_eq!(Power::with_fix0(-3102).kwh().to_string(), "-3.1");
        assert_eq!(Power::with_fix0(-1).kwh().to_string(), "0");
        assert_eq!(Power::with_fix0(-114).kwh().to_string(), "-0.1");
        assert_eq!(Power::with_fix0(-14).kwh().to_string(), "0");
    }

    #[test]
    fn test_display_low_voltage() {
        assert_eq!(LowVoltage::with_fix3(305).to_string(), "0.305");
        assert_eq!(LowVoltage::with_fix3(310).to_string(), "0.31");
        assert_eq!(LowVoltage::with_fix3(325).to_string(), "0.325");
        assert_eq!(LowVoltage::with_fix3(300).to_string(), "0.3");
        assert_eq!(LowVoltage::with_fix3(1020).to_string(), "1.02");
    }
}
