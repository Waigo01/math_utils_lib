#[cfg(feature = "async")]
use async_macro::function_async;

use async_macro::async_call;

use crate::{basetypes::Value, maths::num_traits::Number};

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn ssmult<N: Number>(a: &N, b: &N) -> Result<Value<N>, String> {
    Ok(Value::Scalar(*a**b))
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn svmult<N: Number>(a: &N, b: &Vec<N>) -> Result<Value<N>, String> {
    let mut output_v = vec![];
    for i in b {
        output_v.push(*i**a);
    }
    Ok(Value::Vector(output_v))
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn smmult<N: Number>(a: &N, b: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    let mut output_m = vec![];
    for i in 0..b.len() {
        let mut row = vec![];
        for j in 0..b[0].len() {
            row.push(b[i][j]**a);
        }
        output_m.push(row);
    }
    Ok(Value::Matrix(output_m))
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn vvmult<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a.len() != b.len() {
        return Err("vectors have different dimensions".to_string());
    }
    let mut sum = N::zero();
    for i in 0..a.len() {
        sum = sum + a[i]*b[i];
    }
    return Ok(Value::Scalar(sum));
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn mvmult<N: Number>(a: &Vec<Vec<N>>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a[0].len() != b.len() {
        return Err("vector and matrix have incompatible dimensions".to_string());
    }
    let mut output_v = vec![];
    for i in 0..a.len() {
        let mut sum = N::zero();
        for j in 0..a[i].len() {
            sum = sum + a[i][j]*b[j]
        }
        output_v.push(sum);
    }
    return Ok(Value::Vector(output_v));
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn mmmult<N: Number>(a: &Vec<Vec<N>>, b: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    if a[0].len() != b.len() {
        return Err("matrices have incompatible dimensions".to_string());
    }
    let mut output_m = vec![];
    for i in 0..a.len() {
        let mut row = vec![];
        for j in 0..b[0].len() {
            let mut sum = N::zero();
            for k in 0..a[0].len() {
                sum = sum + a[i][k]*b[k][j]
            }
            row.push(sum);
        }
        output_m.push(row);
    }
    return Ok(Value::Matrix(output_m))
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn ssdiv<N: Number>(a: &N, b: &N) -> Result<Value<N>, String> {
    return Ok(Value::Scalar(*a/ *b));
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn vsdiv<N: Number>(a: &Vec<N>, b: &N) -> Result<Value<N>, String> {
    return async_call!(svmult(&b.recip(), a));
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn vvdiv<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a.len() != b.len() {
        return Err("vectors have incompatible dimensions".to_string());
    }
    let mut sum = N::zero();
    for i in 0..a.len() {
        sum = sum + a[i]/b[i];
    }

    return Ok(Value::Scalar(sum));
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn msdiv<N: Number>(a: &Vec<Vec<N>>, b: &N) -> Result<Value<N>, String> {
    return async_call!(smmult(&b.recip(), a));
}
