use crate::{Fixed, Float, Spec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Volt(pub Fixed);
impl Spec for Volt {
    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }

    const SCALE: Float = 10.0;
    const SYMBOL: &'static str = "V";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct PreciseVolt(pub Fixed);
impl Spec for PreciseVolt {
    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }

    const SCALE: Float = 1000.0;
    const SYMBOL: &'static str = "V";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Amp(pub Fixed);
impl Spec for Amp {
    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
    const SCALE: Float = 10.0;
    const SYMBOL: &'static str = "A";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Watt(pub Fixed);
impl Spec for Watt {
    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
    const SCALE: Float = 100.0;
    const SYMBOL: &'static str = "W";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct KiloWatt(pub Fixed);
impl Spec for KiloWatt {
    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
    const SCALE: Float = 10.0;
    const SYMBOL: &'static str = "kW";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct KiloWattHour(pub Fixed);
impl Spec for KiloWattHour {
    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
    const SCALE: Float = 100.0;
    const SYMBOL: &'static str = "kWh";
}
