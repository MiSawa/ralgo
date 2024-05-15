pub use num::{Integer, One, Zero};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Product;
use std::ops::{Add, Mul, MulAssign, Neg};

pub trait Modular {
    type I: Integer + Clone + Display;
    type II: Integer + From<Self::I> + Clone;
    fn modular() -> Self::I;
    fn convert(ii: Self::II) -> Self::I;

    fn take_modulo(mut value: Self::I) -> Self::I {
        value = value % Self::modular();
        if value < Self::I::zero() {
            value + Self::modular()
        } else {
            value
        }
    }

    fn take_modulo_larger(value: Self::II) -> Self::I {
        let value: Self::I = Self::convert(value % std::convert::From::from(Self::modular()));
        if value < Self::I::zero() {
            value + Self::modular()
        } else {
            value
        }
    }
}

#[derive(Eq, PartialEq, Default, Copy)]
pub struct ModInt<M: Modular> {
    pub value: M::I,
}

impl<M: Modular> Debug for ModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("ModInt {}", self.value))
    }
}

impl<M: Modular> Display for ModInt<M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<M: Modular> Clone for ModInt<M> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<M: Modular> ModInt<M> {
    pub fn from(v: M::I) -> Self {
        Self {
            value: M::take_modulo(v),
        }
    }

    pub fn from_large(v: M::II) -> Self {
        Self {
            value: M::take_modulo_larger(v),
        }
    }
}

impl<M: Modular> Zero for ModInt<M> {
    fn zero() -> Self {
        Self {
            value: M::I::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<M: Modular> One for ModInt<M> {
    fn one() -> Self {
        Self { value: M::I::one() }
    }

    fn is_one(&self) -> bool {
        self.value.is_one()
    }
}

impl<M: Modular> Neg for ModInt<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.value.is_zero() {
            self
        } else {
            Self {
                value: M::modular() - self.value,
            }
        }
    }
}

impl<M: Modular> Add for ModInt<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let tmp = self.value + rhs.value;
        let value = if tmp >= M::modular() {
            tmp - M::modular()
        } else {
            tmp
        };
        Self { value }
    }
}

impl<M: Modular> Mul for ModInt<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs: M::II = self.value.into();
        let rhs: M::II = rhs.value.into();
        Self {
            value: M::take_modulo_larger(lhs * rhs),
        }
    }
}
impl<M: Modular> MulAssign for ModInt<M> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}

impl<M: Modular> Product for ModInt<M> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), Self::mul)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Modular5();
    impl Modular for Modular5 {
        type I = i32;
        type II = i64;

        fn modular() -> Self::I {
            5
        }

        fn convert(ii: Self::II) -> Self::I {
            ii as Self::I
        }
    }
    type ModInt = super::ModInt<Modular5>;

    #[test]
    fn test() {
        let a = ModInt::from(3);
        let b = ModInt::from(2);
        assert_eq!((a * b).value, 1);
    }
}
