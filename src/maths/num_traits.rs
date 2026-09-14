use std::{fmt::{Debug, Display, LowerExp}, ops::{Add, Div, Mul, Neg, Sub}, str::FromStr};

use crate::{Variable, basetypes::InternalFunction, maths};

/// This trait the number as it is required by the parser and evaluator. 
///
/// The [FromStr] trait is used during parsing. The parser will first check if a given string can be parsed to a type
/// implementing this trait. It is therefore very important that the from_str method returns an error if
/// a string cannot be parsed.
pub trait Number:
Add<Self, Output = Self> +
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
Send +
Sync +
LowerExp +
{
    /// returns the neutral element of addition of this number type.
    fn zero() -> Self;
    /// returns the neutral element of multiplication of this number type.
    fn one() -> Self;
    /// returns a small number indicating the precision of this number type. 
    ///
    /// don't choose epsilon too small, think what precision you want the calculations to be and choose accordingly.
    fn epsilon() -> Self;
    /// returns a small number indicating the display precision of this number type.
    fn display_epsilon() -> Self;
    /// returns a small number which indicates when to print values in scientific notation.
    ///
    /// if a value abs(x) < scientific_epsilon then print x in scientific notation. If a value abs(x) > 1/display_epsilon then print x in scientific notation.
    fn scientific_epsilon() -> Self;
    /// returns the base/radix of this number type.
    fn base() -> Self;
    /// returns the nan value of this number type.
    fn nan() -> Self;
    /// returns the inf value of this number type.
    fn infinity() -> Self;
    /// returns the -inf value of this number type.
    fn neg_infinity() -> Self;
    /// returns the default constants that should be added to [Context::default()](crate::Context::default()).
    fn default_vars() -> Vec<Variable<Self>>;
    /// returns the default functions that should be added to
    /// [Context::default()](crate::Context::default()).
    fn default_functions() -> Vec<InternalFunction<Self>>;
    /// provides the newton's method with starting guesses. Should return an iterator with the size
    /// of n_values, which corresponds to the number of initial guesses.
    fn newton_search_pattern(n_values: usize) -> Vec<Self>;
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
    /// returns the asin of the number.
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
    /// returns e^x.
    fn exp(self) -> Self;
    /// computes the factorial of the number.
    fn fact(self) -> Self;

    /// returns some basic functions that are used by [Context::default()](crate::Context::default()).
    ///
    /// The default functions are: sin(x), cos(x), tan(x), sinh(x), cosh(x), tanh(x), exp(x), abs(x),
    /// sqrt(x), root(x, n) (the nth root of x), ln(x), arcsin(x), arccos(x), arctan(x), det(M) (the determinant of the matrix M),
    /// inv(M) (the inverse of the matrix M), fact(x) (the factorial of x, in the case of [f64] and
    /// [Complex](crate::Complex) this is implemented with the Gamma function)
    fn default_functions() -> Vec<InternalFunction<Self>> where Self: Number {
        vec![
            InternalFunction::new("sin".to_string(), 1, maths::sin),
            InternalFunction::new("cos".to_string(), 1, maths::cos),
            InternalFunction::new("tan".to_string(), 1, maths::tan),
            InternalFunction::new("sinh".to_string(), 1, maths::sinh),
            InternalFunction::new("cosh".to_string(), 1, maths::cosh),
            InternalFunction::new("tanh".to_string(), 1, maths::tanh),
            InternalFunction::new("exp".to_string(), 1, maths::exp),
            InternalFunction::new("abs".to_string(), 1, maths::abs),
            InternalFunction::new("sqrt".to_string(), 1, maths::sqrt),
            InternalFunction::new("root".to_string(), 2, maths::root),
            InternalFunction::new("ln".to_string(), 1, maths::ln),
            InternalFunction::new("arcsin".to_string(), 1, maths::arcsin),
            InternalFunction::new("arccos".to_string(), 1, maths::arccos),
            InternalFunction::new("arctan".to_string(), 1, maths::arctan),
            InternalFunction::new("det".to_string(), 1, maths::det),
            InternalFunction::new("inv".to_string(), 1, maths::inv),
            InternalFunction::new("fact".to_string(), 1, maths::fact)
        ]
    }
}
