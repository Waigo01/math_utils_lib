use std::{fmt::{Debug, Display, LowerExp}, iter::Sum, ops::{Add, Div, Mul, Neg, Sub}, str::FromStr};

use crate::{Function, Variable, maths};

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
    /// The nan value of this number type.
    const NAN: Self;
    /// The inf value of this number type.
    const INFINITY: Self;
    /// The -inf value of this number type.
    const NEG_INFINITY: Self;
    /// returns the default constants that should be added to [Context::default()](crate::Context::default()).
    fn default_vars() -> Vec<Variable<Self>>;
    /// returns the default functions that should be added to
    /// [Context::default()](crate::Context::default()).
    fn default_functions() -> Vec<Function<Self>>;
    /// provides the newton's method with starting guesses. Should return an iterator with the size
    /// of n_values, which corresponds to the number of initial guesses.
    fn newton_search_pattern(n_values: usize) -> impl Iterator<Item = Self>;
    /// returns the arithmatic mean between two numbers of this type.
    fn mean(self, other: Self) -> Self;
    /// checks if the value is nan.
    fn is_nan(self) -> bool;
    /// checks if the value is infinite.
    fn is_infinite(self) -> bool;
    /// checks if the value is finitie.
    fn is_finite(self) -> bool;
    /// returns 1/value.
    fn recip(self) -> Self;
    /// returns the floor of the value.
    fn floor(self) -> Self;
    /// returns the ceil of the value.
    fn ceil(self) -> Self;
    /// rounds the value.
    fn round(self) -> Self;
    /// rounds the value and returns it as an integer. This method may return an Error if the value
    /// cannot be returned as an integer. This method is only used to index into a vector and
    /// raise a matrix to an integer power.
    fn as_rounded_int(self) -> Result<i32, ()>;
    /// returns the abs of the value.
    fn abs(self) -> Self;
    /// returns the value raised to an integer power.
    fn powi(self, n: i32) -> Self;
    /// returns the value raised to an arbitrary power.
    fn powf(self, n: Self) -> Self;
    /// returns the square root of the value.
    fn sqrt(self) -> Self;
}

/// Implements functions that are only useful or only defined for real numbers.
///
/// This Trait implementation is also used to avoid having nested complex numbers. Please do not
/// implement this trait if your number type is a complex number.
pub trait RealNumber {
    /// returns the 2-argument arctangent.
    fn atan2(self, other: Self) -> Self;
}

/// Implements some basic functions. It also provides a method that returns the appropriate default
/// functions for context creation.
pub trait StandardFunctions {
    /// returns the sin of the number.
    fn sin(self) -> Self;
    /// returns the cos of the number.
    fn cos(self) -> Self;
    /// returns the tan of the number.
    fn tan(self) -> Self;
    /// Returns the asin of the number.
    fn asin(self) -> Self;
    /// returns the acos of the number.
    fn acos(self) -> Self;
    /// returns the atan of the number.
    fn atan(self) -> Self;
    /// returns the sinh of the number.
    fn sinh(self) -> Self;
    /// returns the cosh of the number.
    fn cosh(self) -> Self;
    /// returns the tanh of the number.
    fn tanh(self) -> Self;
    /// returns the natural log of the number.
    fn ln(self) -> Self;
    /// returns exp(x).
    fn exp(self) -> Self;

    fn default_functions() -> Vec<Function<Self>> where Self: Number {
        vec![
            Function::new_internal("sin".to_string(), 1, maths::sin),
            Function::new_internal("cos".to_string(), 1, maths::cos),
            Function::new_internal("tan".to_string(), 1, maths::tan),
            Function::new_internal("abs".to_string(), 1, maths::abs),
            Function::new_internal("sqrt".to_string(), 1, maths::sqrt),
            Function::new_internal("root".to_string(), 2, maths::root),
            Function::new_internal("ln".to_string(), 1, maths::ln),
            Function::new_internal("arcsin".to_string(), 1, maths::arcsin),
            Function::new_internal("arccos".to_string(), 1, maths::arccos),
            Function::new_internal("arctan".to_string(), 1, maths::arctan),
            Function::new_internal("det".to_string(), 1, maths::det),
            Function::new_internal("inv".to_string(), 1, maths::inv)
        ]
    }
}
