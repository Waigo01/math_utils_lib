use crate::{PREC, Value};

#[doc(hidden)]
pub fn and(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => if (0.-a).abs() > 10f64.powf(-(PREC as f64)-2.) && (0.-b).abs() > 10f64.powf(-(PREC as f64)-2.)  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn or(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => if (0.-a).abs() > 10f64.powf(-(PREC as f64)-2.) || (0.-b).abs() > 10f64.powf(-(PREC as f64)-2.)  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
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
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gt(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn lteq(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => if a < b || (a-b).abs() < 10f64.powf(-(PREC as f64)-2.)  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn gteq(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => if a > b || (a-b).abs() < 10f64.powf(-(PREC as f64)-2.)  {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}

#[doc(hidden)]
pub fn not(rv: &Value) -> Result<Value, String> {
    match rv {
        Value::Scalar(a) => if (0.-a).abs() < 10f64.powf(-(PREC as f64)-2.) {Ok(Value::Scalar(1.))} else {Ok(Value::Scalar(0.))},
        _ => Err("Only two scalars can be ordered!".to_string()),
    }
}
