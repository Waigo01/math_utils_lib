use crate::{Value, maths::num_traits::Number};

#[doc(hidden)]
pub fn and<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::zero() && b != N::zero()  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("And can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn or<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::zero() || b != N::zero()  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("Or can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round_to_precision() == rv.round_to_precision() {
        Ok(Value::Scalar(N::one()))
    } else {
        Ok(Value::Scalar(N::zero()))
    }
}

#[doc(hidden)]
pub fn not_eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round_to_precision() != rv.round_to_precision() {
        Ok(Value::Scalar(N::one()))
    } else {
        Ok(Value::Scalar(N::zero()))
    }
}

#[doc(hidden)]
pub fn lt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn lteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a <= b  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round_to_precision(), rv.round_to_precision()) {
        (Value::Scalar(a), Value::Scalar(b)) => if a >= b  {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn not<N: Number>(rv: &Value<N>) -> Result<Value<N>, String> {
    match rv.round_to_precision() {
        Value::Scalar(a) => if a == N::zero() {Ok(Value::Scalar(N::one()))} else {Ok(Value::Scalar(N::zero()))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}
