use crate::{PREC, Value};

#[doc(hidden)]
pub fn and(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != 0. && b != 0.  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("And can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn or(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a != 0. || b != 0.  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Or can only be computed with two scalars!".to_string()),
    }
}

#[doc(hidden)]
pub fn eq(lv: &Value, rv: &Value) -> Result<Value, String> {
    if lv.round(PREC-2) == rv.round(PREC-2) {
        Ok(Value::Scalar(1.))
    } else {
        Ok(Value::Scalar(0.))
    }
}

#[doc(hidden)]
pub fn not_eq(lv: &Value, rv: &Value) -> Result<Value, String> {
    if lv.round(PREC-2) != rv.round(PREC-2) {
        Ok(Value::Scalar(1.))
    } else {
        Ok(Value::Scalar(0.))
    }
}

#[doc(hidden)]
pub fn lt(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gt(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn lteq(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a <= b  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gteq(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv.round(PREC - 2), rv.round(PREC - 2)) {
        (Value::Scalar(a), Value::Scalar(b)) => if a >= b  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn not(rv: &Value) -> Result<Value, String> {
    match rv.round(PREC - 2) {
        Value::Scalar(a) => if a == 0. {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}
