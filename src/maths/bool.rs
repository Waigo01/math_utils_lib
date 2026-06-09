use crate::{PREC, Value, maths::num_trait::Number};

#[doc(hidden)]
pub fn and<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::from(0.) && b != N::from(0.)  {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("And can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn or<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != N::from(0.) || b != N::from(0.)  {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("Or can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round(PREC-2) == rv.round(PREC-2) {
        Ok(Value::Scalar(N::from(1.)))
    } else {
        Ok(Value::Scalar(N::from(0.)))
    }
}

#[doc(hidden)]
pub fn not_eq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    if lv.round(PREC-2) != rv.round(PREC-2) {
        Ok(Value::Scalar(N::from(1.)))
    } else {
        Ok(Value::Scalar(N::from(0.)))
    }
}

#[doc(hidden)]
pub fn lt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gt<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn lteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a <= b  {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gteq<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a >= b  {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn not<N: Number>(rv: &Value<N>) -> Result<Value<N>, String> {
    match rv.round(PREC - 2) {
        Value::Scalar(a) => if a == N::from(0.) {Ok(Value::Scalar(N::from(1.)))} else {Ok(Value::Scalar(N::from(0.)))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}
