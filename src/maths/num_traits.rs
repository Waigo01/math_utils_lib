use std::{fmt::{Debug, Display, LowerExp}, iter::Sum, ops::{Add, Div, Mul, Neg, Rem, Sub}, str::FromStr};

/// This trait the number as it is required by the parser and evaluator. 
///
/// The [FromStr] trait is used during parsing. The parser will first check if a given string can be parsed to a type
/// implementing this trait. It is therefore very important that the from_str method returns an error if
/// a string cannot be parsed.
///
/// The other two [From] trait implementations are only used for the value! macro as a
/// quality of life feature. It is up to the person implementing this trait on a type to think of a
/// reasonable way to convert an f64 and i32 to their number type.
pub trait Number:
Add<Self, Output = Self> +
Sum +
Sub<Self, Output = Self> +
Div<Self, Output = Self> +
Mul<Self, Output = Self> +
Rem<Self, Output = Self> +
Neg<Output = Self> +
Clone +
Copy +
PartialEq +
PartialOrd +
From<i32> +
From<f64> +
FromStr<Err: Debug> +
Display +
Debug +
LowerExp
{
    /// The neutral element of addition of this number type.
    const ZERO: Self;
    /// The neutral element of multiplication of this number type.
    const ONE: Self;
    /// The Base/radix of this number type.
    const BASE: Self;
    /// The default constants that should be added to [Context::default()](crate::Context::default()) returned as their name and value.
    fn default_consts() -> Vec<(String, Self)>;
    /// Provides the newton's method with starting guesses. Should return an iterator with the size
    /// of n_values, which corresponds to the number of initial guesses.
    fn newton_search_pattern(n_values: usize) -> impl Iterator<Item = Self>;
    /// Returns the nan value of this number type.
    fn nan() -> Self;
    /// Returns the inf value of this number type.
    fn infinity() -> Self;
    /// Returns the -inf value of this number type.
    fn neg_infinity() -> Self;
    /// Returns the arithmatic mean between two numbers of this type.
    fn mean(self, other: Self) -> Self;
    /// Checks if the value is nan.
    fn is_nan(self) -> bool;
    /// Checks if the value is infinite.
    fn is_infinite(self) -> bool;
    /// Checks if the value is finitie.
    fn is_finite(self) -> bool;
    /// Returns 1/value.
    fn recip(self) -> Self;
    /// Returns the floor of the value.
    fn floor(self) -> Self;
    /// Returns the ceil of the value.
    fn ceil(self) -> Self;
    /// Rounds the value.
    fn round(self) -> Self;
    /// Rounds the value and returns it as an integer. This method may return an Error if the value
    /// cannot be returned as an integer. This method is only used to index into a vector and
    /// raise a matrix to an integer power.
    fn as_rounded_int(self) -> Result<i32, ()>;
    /// Returns the abs of the value.
    fn abs(self) -> Self;
    /// Returns the value raised to an integer power.
    fn powi(self, n: i32) -> Self;
    /// Returns the value raised to an arbitrary power.
    fn powf(self, n: Self) -> Self;
    /// Returns the square root of the value.
    fn sqrt(self) -> Self;
    /// Returns the natural log of the value.
    fn ln(self) -> Self;
    /// Returns the log of the value with an aribtrary base.
    fn log(self, base: Self) -> Self;
    /// Returns the max out of two values.
    fn max(self, other: Self) -> Self;
    /// Returns the min out of two values.
    fn min(self, other: Self) -> Self;
    /// Returns the sin of the value.
    fn sin(self) -> Self;
    /// Returns the cos of the value.
    fn cos(self) -> Self;
    /// Returns the tan of the value.
    fn tan(self) -> Self;
    /// Returns the arcsin of the value.
    fn asin(self) -> Self;
    /// Returns the arccos of the value.
    fn acos(self) -> Self;
    /// Returns the arctan of the value.
    fn atan(self) -> Self;
}

impl Number for f64 {
    const ONE: Self = 1.;
    const ZERO: Self = 0.;
    const BASE: Self = 10.;
    fn default_consts() -> Vec<(String, Self)> {
        vec![
            ("pi".to_string(), std::f64::consts::PI),
            ("e".to_string(), std::f64::consts::E),
        ]
    }
    fn newton_search_pattern(n_values: usize) -> impl Iterator<Item = Self> {
        ((-(n_values as i32)/2)..(n_values as i32/2)).map(|v| v as f64)
    }
    fn nan() -> Self {
        Self::NAN
    }
    fn infinity() -> Self {
        Self::INFINITY
    }
    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }
    fn mean(self, other: Self) -> Self {
        (self + other)/2.
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
    fn as_rounded_int(self) -> Result<i32, ()> {
        Ok(self.round() as i32)
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
