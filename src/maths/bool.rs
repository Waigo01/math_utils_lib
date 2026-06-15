use crate::{PREC, Value, maths::num_traits::Number};

#[doc(hidden)]
pub fn and<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::ZERO && b != N::ZERO  {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("And can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn or<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::ZERO || b != N::ZERO  {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("Or can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round(PREC-2) == rv.round(PREC-2) {
        Ok(Value::Scalar(N::ONE))
    } else {
        Ok(Value::Scalar(N::ZERO))
    }
}

#[doc(hidden)]
pub fn not_eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round(PREC-2) != rv.round(PREC-2) {
        Ok(Value::Scalar(N::ONE))
    } else {
        Ok(Value::Scalar(N::ZERO))
    }
}

#[doc(hidden)]
pub fn lt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn lteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a <= b  {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a >= b  {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn not<N: Number>(rv: &Value<N>) -> Result<Value<N>, String> {
    match rv.round(PREC - 2) {
        Value::Scalar(a) => if a == N::ZERO {Ok(Value::Scalar(N::ONE))} else {Ok(Value::Scalar(N::ZERO))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}
