use crate::{Fixed, Float, Spec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Volt(pub Fixed);
impl Spec for Volt {
    const SCALE: Float = 10.0;
    const SYMBOL: &'static str = "V";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct PreciseVolt(pub Fixed);
impl Spec for PreciseVolt {
    const SCALE: Float = 1000.0;
    const SYMBOL: &'static str = "V";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Amp(pub Fixed);
impl Spec for Amp {
    const SCALE: Float = 10.0;
    const SYMBOL: &'static str = "A";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Watt(pub Fixed);
impl Spec for Watt {
    const SCALE: Float = 1.0;
    const SYMBOL: &'static str = "W";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct KiloWatt(pub Fixed);
impl Spec for KiloWatt {
    const SCALE: Float = 10.0;
    const SYMBOL: &'static str = "kW";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct KiloWattHour(pub Fixed);
impl Spec for KiloWattHour {
    const SCALE: Float = 100.0;
    const SYMBOL: &'static str = "kWh";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Celsius(pub Fixed);
impl Spec for Celsius {
    const SCALE: Float = 100.0;
    const SYMBOL: &'static str = "C";

    fn to_fixed(self) -> Fixed {
        self.0
    }
    fn from_fixed(fixed: Fixed) -> Self {
        Self(fixed)
    }
}
