use crate::{Float, Repr, Spec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Volt(Repr);
impl Spec for Volt {
    fn to_repr(self) -> Repr {
        self.0
    }
    fn from_repr(repr: Repr) -> Self {
        Self(repr)
    }

    const SCALE: Float = 10.0;
    const PRECISION: usize = 1;
    const SYMBOL: &'static str = "V";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Amp(Repr);
impl Spec for Amp {
    fn to_repr(self) -> Repr {
        self.0
    }
    fn from_repr(repr: Repr) -> Self {
        Self(repr)
    }
    const SCALE: Float = 10.0;
    const PRECISION: usize = 1;
    const SYMBOL: &'static str = "A";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct Watt(Repr);
impl Spec for Watt {
    fn to_repr(self) -> Repr {
        self.0
    }
    fn from_repr(repr: Repr) -> Self {
        Self(repr)
    }
    const SCALE: Float = 100.0;
    const PRECISION: usize = 2;
    const SYMBOL: &'static str = "W";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct KiloWatt(Repr);
impl Spec for KiloWatt {
    fn to_repr(self) -> Repr {
        self.0
    }
    fn from_repr(repr: Repr) -> Self {
        Self(repr)
    }
    const SCALE: Float = 10.0;
    const PRECISION: usize = 1;
    const SYMBOL: &'static str = "kW";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct KiloWattHour(Repr);
impl Spec for KiloWattHour {
    fn to_repr(self) -> Repr {
        self.0
    }
    fn from_repr(repr: Repr) -> Self {
        Self(repr)
    }
    const SCALE: Float = 100.0;
    const PRECISION: usize = 2;
    const SYMBOL: &'static str = "kWh";
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Serialize, Deserialize, Ord, PartialOrd)]
pub struct CentiOhm(Repr);
impl Spec for CentiOhm {
    fn to_repr(self) -> Repr {
        self.0
    }
    fn from_repr(repr: Repr) -> Self {
        Self(repr)
    }
    const SCALE: Float = 1000.0;
    const PRECISION: usize = 3;
    const SYMBOL: &'static str = "cΩ";
}
