#[cfg(feature = "async")]
use async_macro::function_async;

use crate::{Value, maths::num_traits::Number};

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn and<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::zero() && b != N::zero()  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("and can only be computed with two scalars".to_string()),
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn or<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::zero() || b != N::zero()  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("or can only be computed with two scalars".to_string()),
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round_to_precision() == rv.round_to_precision() {
        Ok(Value::Scalar(N::one()))
    } else {
        Ok(Value::Scalar(N::zero()))
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn not_eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round_to_precision() != rv.round_to_precision() {
        Ok(Value::Scalar(N::one()))
    } else {
        Ok(Value::Scalar(N::zero()))
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn lt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("only two scalars can be ordered".to_string()),
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn gt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("only two scalars can be ordered".to_string()),
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn lteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a <= b  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("only two scalars can be ordered".to_string()),
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn gteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a >= b  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("only two scalars can be ordered".to_string()),
    }
}

#[doc(hidden)]
#[cfg_attr(feature = "async", function_async)]
pub fn not<N: Number>(rv: &Value<N>) -> Result<Value<N>, String> {
    match rv.round_to_precision() {
        Value::Scalar(a) => if a == N::zero() {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("only two scalars can be ordered".to_string()),
    }
}
