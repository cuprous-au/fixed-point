use crate::{Float, Spec};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Volt;
impl Spec for Volt {
    type Repr = u32;
    const SCALE: Float = 10.0;
    const PRECISION: usize = 1;
    const SYMBOL: &'static str = "V";
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Amp;
impl Spec for Amp {
    type Repr = i32;
    const SCALE: Float = 10.0;
    const PRECISION: usize = 1;
    const SYMBOL: &'static str = "A";
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Watt;
impl Spec for Watt {
    type Repr = i32;
    const SCALE: Float = 100.0;
    const PRECISION: usize = 2;
    const SYMBOL: &'static str = "W";
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct KiloWatt;
impl Spec for KiloWatt {
    type Repr = i32;
    const SCALE: Float = 10.0;
    const PRECISION: usize = 1;
    const SYMBOL: &'static str = "kW";
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct KiloWattHour;
impl Spec for KiloWattHour {
    type Repr = u32;
    const SCALE: Float = 100.0;
    const PRECISION: usize = 2;
    const SYMBOL: &'static str = "kWh";
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CentiOhm;
impl Spec for CentiOhm {
    type Repr = u32;
    const SCALE: Float = 1000.0;
    const PRECISION: usize = 3;
    const SYMBOL: &'static str = "cΩ";
}
