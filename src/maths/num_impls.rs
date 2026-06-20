use std::{fmt::{Display, LowerExp}, iter::Sum, ops::{Add, Div, Mul, Neg, Sub}, str::FromStr};

use crate::{Number, Value, Variable, basetypes::InternalFunction, maths::num_traits::{RealNumber, StandardFunctions}};

impl StandardFunctions for f64 {
    fn ln(self) -> Self {
        self.ln()
    }
    fn sin(self) -> Self {
        if self == std::f64::consts::PI {
            return 0.
        } else {
            self.sin()
        }
    }
    fn cos(self) -> Self {
        if self == std::f64::consts::PI/2. {
            return 0.
        } else {
            self.cos()
        }
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
    fn sinh(self) -> Self {
        self.sinh()
    }
    fn cosh(self) -> Self {
        self.cosh()
    }
    fn tanh(self) -> Self {
        self.tanh()
    }
    fn atan(self) -> Self {
        self.atan()
    }
    fn exp(self) -> Self {
        self.exp()
    }
    fn fact(self) -> Self {
        let res = crate::helpers::lanczos_approx(Complex { re: self + Self::ONE, im: Self::ZERO });
        if res.im < Self::EPSILON {
            return res.re;
        } else {
            return Self::NAN;
        }
    }
}

impl RealNumber for f64 {
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
}

impl Number for f64 {
    const ONE: Self = 1.;
    const ZERO: Self = 0.;
    const NAN: Self = f64::NAN*2.;
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
    const EPSILON: Self = 0.00000001;
    const DISPLAY_EPSILON: Self = 0.000001;
    const BASE: Self = 10.;
    fn default_vars() -> Vec<Variable<Self>> {
        vec![
            Variable::new("pi".to_string(), Value::Scalar(std::f64::consts::PI)),
            Variable::new("e".to_string(), Value::Scalar(std::f64::consts::E)),
        ]
    }
    fn default_functions() -> Vec<InternalFunction<Self>> {
       <Self as StandardFunctions>::default_functions() 
    }
    fn newton_search_pattern(n_values: usize) -> Vec<Self> {
        ((-(n_values as i32)/2)..(n_values as i32/2)).map(|v| v as f64/4.).collect()
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
}

/// A simple complex number type that can be used to build a complex number from a type that
/// implements [Number](crate::Number), [StandardFunctions](crate::StandardFunctions) and [RealNumber](crate::RealNumber).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Complex<N: Number + StandardFunctions + RealNumber> {
    re: N,
    im: N
}

impl<N: Number + StandardFunctions + RealNumber> Display for Complex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.re, if self.im != N::ZERO && self.im > N::ZERO {"+".to_string() + &self.im.to_string()} else if self.im != N::ZERO && self.im < N::ZERO {self.im.to_string()} else {"".to_string()})
    }
}

impl<N: Number + StandardFunctions + RealNumber> FromStr for Complex<N> {
    type Err = N::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Complex { re: s.parse::<N>()?, im: N::ZERO })
    }
}

impl<N: Number + StandardFunctions + RealNumber> From<f64> for Complex<N> {
    fn from(value: f64) -> Self {
        Complex { re: N::from(value), im: N::ZERO }
    }
}

impl<N: Number + StandardFunctions + RealNumber> From<i32> for Complex<N> {
    fn from(value: i32) -> Self {
        Complex { re: N::from(value), im: N::ZERO }
    }
}

impl<N: Number + StandardFunctions + RealNumber> Complex<N> {
    /// The imaginary unit.
    const I: Self = Complex{re: N::ZERO, im: N::ONE};
    /// returns the argument of the complex number.
    fn argument(self) -> N {
        self.im.atan2(self.re)
    }
    /// creates a new complex number from a real and imaginiary part.
    pub fn new(re: N, im: N) -> Self {
        Complex { re, im }
    }
    /// returns the real part of the number.
    pub fn re(self) -> N {
        self.re
    }
    /// returns the imaginary part of the number.
    pub fn im(self) -> N {
        self.im
    }
}

impl<N: Number + StandardFunctions + RealNumber> LowerExp for Complex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im != N::ZERO {
            write!(f, "{:+e}{}{:+e}", self.re, if self.im > N::ZERO {"+".to_string()} else {"".to_string()}, self.im)
        } else {
            write!(f, "{:+e}", self.re)
        }
    }
}

impl<N: Number + StandardFunctions + RealNumber> Neg for Complex<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex{re: -self.re, im: -self.im}
    }
}

impl<N: Number + StandardFunctions + RealNumber> Add for Complex<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex{re: self.re + rhs.re, im: self.im + rhs.im}
    }
}

impl<N: Number + StandardFunctions + RealNumber> Sub for Complex<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex{re: self.re - rhs.re, im: self.im - rhs.im}
    }
}

impl<N: Number + StandardFunctions + RealNumber> Mul for Complex<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex{re: self.re*rhs.re - self.im*rhs.im, im: self.re*rhs.im+self.im*rhs.re}
    }
}

impl<N: Number + StandardFunctions + RealNumber> Div for Complex<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Complex{re: (self.re*rhs.re+self.im*rhs.im)/(rhs.re.powi(2) + rhs.im.powi(2)), im: (self.im*rhs.re-self.re*rhs.im)/(rhs.re.powi(2) + rhs.im.powi(2))}
    }
}

impl<N: Number + StandardFunctions + RealNumber> Sum for Complex<N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = Self::ZERO;
        for item in iter {
            sum = sum + item
        }
        sum
    }
}

impl<N: Number + StandardFunctions + RealNumber> Number for Complex<N> {
    const ONE: Self = Complex{re: N::ONE, im: N::ZERO};
    const ZERO: Self = Complex{re: N::ZERO, im: N::ZERO};
    const NAN: Self = Complex{re: N::NAN, im: N::NAN};
    const INFINITY: Self = Complex{re: N::INFINITY, im: N::INFINITY};
    const NEG_INFINITY: Self = Complex{re: N::NEG_INFINITY, im: N::NEG_INFINITY};
    const EPSILON: Self = Complex{re: N::EPSILON, im: N::ZERO};
    const DISPLAY_EPSILON: Self = Complex{re: N::DISPLAY_EPSILON, im: N::ZERO};
    const BASE: Self = Complex{re: N::BASE, im: N::ZERO};
    fn default_vars() -> Vec<Variable<Self>> {
        let mut default_vars = vec![];

        for var in N::default_vars() {
            let mut new_values = vec![];
            for value in var.values.to_vec() {
                match value {
                    Value::Scalar(s) => new_values.push(Value::Scalar(Complex{re: s, im: N::ZERO})),
                    Value::Vector(v) => new_values.push(Value::Vector(v.into_iter().map(|s| Complex{re: s, im: N::ZERO}).collect())),
                    Value::Matrix(m) => new_values.push(Value::Matrix(m.into_iter().map(|v| v.into_iter().map(|s| Complex{re: s, im: N::ZERO}).collect()).collect()))
                }
            }
            default_vars.push(Variable::new(var.name, new_values));
        }

        default_vars.push(Variable::new("i".to_string(), Value::Scalar(Complex{re: N::ZERO, im: N::ONE})));

        default_vars
    }
    fn default_functions() -> Vec<InternalFunction<Self>> {
        <Self as StandardFunctions>::default_functions()
    }
    fn newton_search_pattern(n_values: usize) -> Vec<Self> {
        ((-(n_values as i32)/2)..(n_values as i32/2)).map(|v| Self::from(v/4)*(Self::I*Self::from(v/4)).exp()).collect()
    }
    fn abs(self) -> Self {
        Complex{re: (self.re.powi(2) + self.im.powi(2)).sqrt(), im: N::ZERO}
    }
    fn ceil(self) -> Self {
        Complex { re: self.re.ceil(), im: self.im.ceil() }
    }
    fn floor(self) -> Self {
        Complex { re: self.re.floor(), im: self.im.floor() }
    }
    fn round(self) -> Self {
        Complex { re: self.re.round(), im: self.im.round() }
    }
    fn as_rounded_int(self) -> Result<i32, ()> {
        self.re.as_rounded_int()
    }
    fn is_infinite(self) -> bool {
        self.re.is_infinite() || self.im.is_infinite()
    }
    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
    fn is_nan(self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }
    fn mean(self, other: Self) -> Self {
        Complex { re: self.re.mean(other.re), im: self.im.mean(other.im) }
    }
    fn powi(self, n: i32) -> Self {
        let modulus = self.abs().re;
        let argument = self.argument();
        Complex{re: modulus.powi(n)*(N::from(n)*argument).cos(), im: modulus.powi(n)*(N::from(n)*argument).sin()}
    }
    fn recip(self) -> Self {
        let modulus = self.abs().re;
        let argument = self.argument();
        Complex { re: modulus.recip()*(-argument).cos(), im: modulus.recip()*(-argument).sin() }
    }
    fn powf(self, n: Self) -> Self {
        let modulus = self.abs().re;
        let argument = self.argument();
        let re = modulus.powf(n.re)*(-n.im*argument).exp()*(n.re*argument+n.im*modulus.ln()).cos();
        let im = modulus.powf(n.re)*(-n.im*argument).exp()*(n.re*argument+n.im*modulus.ln()).sin();
        Complex { re, im }
    }
    fn sqrt(self) -> Self {
        let argument = self.argument();
        let a = (self.re.powi(2) + self.im.powi(2)).sqrt().sqrt() * N::ZERO.mean(argument).cos();
        let b = (self.re.powi(2) + self.im.powi(2)).sqrt().sqrt() * N::ZERO.mean(argument).sin();
        Complex { re: a, im: b }
    }
}

impl<N: Number + StandardFunctions + RealNumber> StandardFunctions for Complex<N> {
    fn sin(self) -> Self {
        Complex { re: self.re.sin()*self.im.cosh(), im: self.re.cos()*self.im.sinh() }
    }
    fn cos(self) -> Self {
        Complex { re: self.re.cos()*self.im.cosh(), im: self.re.sin()*self.im.sinh() }
    }
    fn tan(self) -> Self {
        Complex { re: self.re.tan(), im: self.im.tanh() }/Complex{re: N::ONE, im: -self.re.tan()*self.im.tanh()}
    }
    fn sinh(self) -> Self {
        Complex { re: -self.im, im: self.re }.sin()/Complex{re: N::ZERO, im: N::ONE}
    }
    fn cosh(self) -> Self {
        Complex { re: -self.im, im: self.re }.cos()
    }
    fn tanh(self) -> Self {
        Complex { re: -self.im, im: self.re }.tan()/Complex{re: N::ZERO, im: N::ONE}
    }
    fn ln(self) -> Self {
        Complex { re: self.abs().re.ln(), im: self.argument() }
    }
    fn asin(self) -> Self {
        Complex::I.recip()*(Complex::I*self + (Complex::ONE-self.powi(2)).sqrt()).ln()
    }
    fn acos(self) -> Self {
        Complex::I.recip()*(self+(self.powi(2)-Complex::ONE).sqrt()).ln()
    }
    fn atan(self) -> Self {
        Complex::ZERO.mean(Complex::I.recip()*((Complex::I-self)/(Complex::I+self)).ln())
    }
    fn exp(self) -> Self {
        Complex { re: self.re.exp(), im: N::ZERO }*Complex{re: self.im.cos(), im: self.im.sin()}
    }
    fn fact(self) -> Self {
        crate::helpers::lanczos_approx(self + Self::ONE)
    }
}
