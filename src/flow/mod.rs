pub mod dfs_lca_network_simplex;
pub mod dinic;
pub mod network_simplex;

use std::fmt::Display;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub trait Zero: Sized {
    fn zero() -> Self;
}

pub trait One: Sized {
    fn one() -> Self;
}

pub trait Cost:
    Display
    + Copy
    + Eq
    + Ord
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Neg<Output = Self>
{
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }
    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }
}

pub trait Flow:
    Display
    + Copy
    + Eq
    + Ord
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Neg<Output = Self>
{
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
    fn is_positive(&self) -> bool {
        self > &Self::zero()
    }
    fn is_negative(&self) -> bool {
        self < &Self::zero()
    }
    fn abs(&self) -> Self {
        if self.is_negative() {
            -*self
        } else {
            *self
        }
    }
}

macro_rules! implement {
    ($T:ty) => {
        impl Zero for $T {
            #[inline]
            fn zero() -> Self {
                0
            }
        }
        impl One for $T {
            #[inline]
            fn one() -> Self {
                1
            }
        }
        impl Flow for $T {}
        impl Cost for $T {}
    };
}

implement!(i8);
implement!(i16);
implement!(i32);
implement!(i64);
implement!(i128);
implement!(isize);
