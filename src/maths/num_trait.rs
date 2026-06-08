use std::{fmt::LowerExp, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

/// A simple number trait to use with generics.
pub trait Number<T: Number = Self>:
Add<Self, Output = Self> +
AddAssign +
Sub<Self, Output = Self> +
SubAssign +
Div<Self, Output = Self> +
DivAssign +
Mul<Self, Output = Self> +
MulAssign +
ToString +
Clone +
PartialEq +
PartialOrd +
Copy +
From<f64> +
Into<f64> +
LowerExp
{
    fn nan() -> Self;
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
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
