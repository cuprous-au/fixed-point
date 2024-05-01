use core::ops::Add;

use serde::{Deserialize, Serialize};

/// Represents a three phase electrical quanity where some or all
/// phases may be out of service. (An out of service phase is
/// distinct from a phase with a zero value.)
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PhasesOpt<T>(pub Option<T>, pub Option<T>, pub Option<T>);

impl<T> PhasesOpt<T> {
    pub fn count(&self) -> usize {
        self.0.is_some() as usize + self.1.is_some() as usize + self.2.is_some() as usize
    }
}

impl<T> PhasesOpt<T>
where
    T: Add<T, Output = T>,
{
    pub fn sum(self) -> Option<T> {
        add_opt(self.0, add_opt(self.1, self.2))
    }
}

impl<T> PhasesOpt<T>
where
    T: Ord,
{
    pub fn max(self) -> Option<T> {
        max_opt(self.0, max_opt(self.1, self.2))
    }

    pub fn min(self) -> Option<T> {
        min_opt(self.0, min_opt(self.1, self.2))
    }
}

impl<T> Add<PhasesOpt<T>> for PhasesOpt<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(
            add_opt(self.0, rhs.0),
            add_opt(self.1, rhs.1),
            add_opt(self.2, rhs.2),
        )
    }
}

fn add_opt<T>(lhs: Option<T>, rhs: Option<T>) -> Option<T>
where
    T: Add<T, Output = T>,
{
    match (lhs, rhs) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn max_opt<T>(lhs: Option<T>, rhs: Option<T>) -> Option<T>
where
    T: Ord,
{
    match (lhs, rhs) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn min_opt<T>(lhs: Option<T>, rhs: Option<T>) -> Option<T>
where
    T: Ord,
{
    match (lhs, rhs) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}
