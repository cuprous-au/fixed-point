use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::Float;

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

impl<T> AddAssign<PhasesOpt<T>> for PhasesOpt<T>
where
    T: Add<T, Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: PhasesOpt<T>) {
        *self = *self + rhs;
    }
}

impl<T> Sub<PhasesOpt<T>> for PhasesOpt<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(
            sub_opt(self.0, rhs.0),
            sub_opt(self.1, rhs.1),
            sub_opt(self.2, rhs.2),
        )
    }
}

impl<T> SubAssign<PhasesOpt<T>> for PhasesOpt<T>
where
    T: Sub<T, Output = T> + Copy,
{
    fn sub_assign(&mut self, rhs: PhasesOpt<T>) {
        *self = *self - rhs;
    }
}

impl<T> Mul<Float> for PhasesOpt<T>
where
    T: Mul<T, Output = T> + From<Float> + Into<Float>,
{
    type Output = Self;

    fn mul(self, rhs: Float) -> Self {
        Self(
            mul_opt(self.0, rhs),
            mul_opt(self.1, rhs),
            mul_opt(self.2, rhs),
        )
    }
}

impl<T> MulAssign<Float> for PhasesOpt<T>
where
    T: Mul<T, Output = T> + From<Float> + Into<Float> + Copy,
{
    fn mul_assign(&mut self, rhs: Float) {
        *self = *self * rhs;
    }
}

impl<T> Div<Float> for PhasesOpt<T>
where
    T: Div<Float, Output = T> + From<Float> + Into<Float>,
{
    type Output = Self;

    fn div(self, rhs: Float) -> Self {
        Self(
            div_opt(self.0, rhs),
            div_opt(self.1, rhs),
            div_opt(self.2, rhs),
        )
    }
}

impl<T> DivAssign<Float> for PhasesOpt<T>
where
    T: Div<Float, Output = T> + From<Float> + Into<Float> + Copy,
{
    fn div_assign(&mut self, rhs: Float) {
        *self = *self / rhs;
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

fn sub_opt<T>(lhs: Option<T>, rhs: Option<T>) -> Option<T>
where
    T: Sub<T, Output = T>,
{
    match (lhs, rhs) {
        (Some(a), Some(b)) => Some(a - b),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    }
}

fn mul_opt<T>(lhs: Option<T>, rhs: Float) -> Option<T>
where
    T: Mul<T, Output = T> + From<Float> + Into<Float>,
{
    match (lhs, rhs) {
        (Some(a), b) => Some((a.into() as Float * b).into()),
        (None, _) => None,
    }
}

fn div_opt<T>(lhs: Option<T>, rhs: Float) -> Option<T>
where
    T: Div<Float, Output = T> + From<Float> + Into<Float>,
{
    match (lhs, rhs) {
        (Some(a), b) => Some((a.into() as Float / b).into()),
        (None, _) => None,
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
