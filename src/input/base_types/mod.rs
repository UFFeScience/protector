use std::convert::TryFrom;

use derive_more::{
    Add, AddAssign, AsRef, Constructor, Display, From, FromStr, Sub, SubAssign, Sum,
};
use ordered_float::NotNan;

#[derive(
    Clone,
    Copy,
    AsRef,
    Display,
    From,
    Add,
    AddAssign,
    Sub,
    Sum,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    FromStr,
)]
pub struct CrimeFactor(NotNan<f64>);

#[derive(
    Clone,
    Copy,
    AsRef,
    Display,
    From,
    Add,
    AddAssign,
    SubAssign,
    Sub,
    Sum,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    FromStr,
)]
pub struct Distance(NotNan<f64>);

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash, Constructor, Display, FromStr, AsRef)]
pub struct Id(u32);

#[derive(
    PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash, Constructor, Display, FromStr, AsRef,
)]
pub struct Zone(u32);

macro_rules! shared_impl {
    ($a:ident) => {
        impl Default for $a {
            fn default() -> Self {
                Self(NotNan::new(0.0).unwrap())
            }
        }

        impl TryFrom<f64> for $a {
            type Error = anyhow::Error;

            fn try_from(value: f64) -> Result<Self, Self::Error> {
                Ok(Self(NotNan::try_from(value)?))
            }
        }

        impl std::fmt::Debug for $a {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.as_ref().as_ref())
            }
        }
    };
}

shared_impl!(CrimeFactor);

shared_impl!(Distance);
