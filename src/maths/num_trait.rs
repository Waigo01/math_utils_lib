use std::{fmt::{Debug, Display, LowerExp}, iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign}};

/// A simple number trait to use with generics.
pub trait Number<T: Number = Self>:
Add<Self, Output = Self> +
AddAssign +
Sum +
Sub<Self, Output = Self> +
SubAssign +
Div<Self, Output = Self> +
DivAssign +
Mul<Self, Output = Self> +
MulAssign +
Rem<Self, Output = Self> +
RemAssign +
Display +
Clone +
PartialEq +
PartialOrd +
Copy +
From<f64> +
From<i32> +
Into<f64> +
Debug +
LowerExp
{
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn recip(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn abs(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
}

impl Number for f64 {
    fn nan() -> Self {
        Self::NAN
    }
    fn infinity() -> Self {
        Self::INFINITY
    }
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }
    fn is_nan(self) -> bool {
        self.is_nan()
    }
    fn is_infinite(self) -> bool {
        self.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.is_finite()
    }
    fn is_sign_negative(self) -> bool {
        self.is_sign_negative()
    }
    fn is_sign_positive(self) -> bool {
        self.is_sign_positive()
    }
    fn recip(self) -> Self {
        self.recip()
    }
    fn floor(self) -> Self {
        self.floor()
    }
    fn ceil(self) -> Self {
        self.ceil()
    }
    fn round(self) -> Self {
        self.round()
    }
    fn abs(self) -> Self {
        self.abs()
    }
    fn powi(self, n: i32) -> Self {
        self.powi(n)
    }
    fn powf(self, n: Self) -> Self {
        self.powf(n)
    }
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn exp2(self) -> Self {
        self.exp2()
    }
    fn ln(self) -> Self {
        self.ln()
    }
    fn log(self, base: Self) -> Self {
        self.log(base)
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn sin(self) -> Self {
        self.sin()
    }
    fn cos(self) -> Self {
        self.cos()
    }
    fn tan(self) -> Self {
        self.tan()
    }
    fn asin(self) -> Self {
        self.asin()
    }
    fn acos(self) -> Self {
        self.acos()
    }
    fn atan(self) -> Self {
        self.atan()
    }
}
