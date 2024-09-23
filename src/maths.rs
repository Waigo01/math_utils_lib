use crate::basetypes::Value;

pub mod add_sub;
pub mod mult_div;
pub mod cross_pow;
pub mod calculus;

#[doc(hidden)]
pub fn add(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return add_sub::sadd(a, b),
        (Value::Vector(a), Value::Vector(b)) => return add_sub::vadd(a, b),
        (Value::Matrix(a), Value::Matrix(b)) => return add_sub::madd(a, b),
        (Value::Vector(_), Value::Scalar(_)) => return Err("Can't add scalar to vector!".to_string()),
        (Value::Scalar(_), Value::Vector(_)) => return Err("Can't add vector to scalar!".to_string()),
        (Value::Matrix(_), Value::Scalar(_)) => return Err("Can't add scalar to matrix!".to_string()),
        (Value::Scalar(_), Value::Matrix(_)) => return Err("Can't add matrix to scalar!".to_string()),
        (Value::Vector(_), Value::Matrix(_)) => return Err("Can't add matrix to vector!".to_string()),
        (Value::Matrix(_), Value::Vector(_)) => return Err("Can't add vector to matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn sub(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return add_sub::sadd(a, &(b * (-1.))),
        (Value::Vector(a), Value::Vector(b)) => return add_sub::vsub(a, b),
        (Value::Matrix(a), Value::Matrix(b)) => return add_sub::msub(a, b),
        (Value::Vector(_), Value::Scalar(_)) => return Err("Can't subtract scalar from vector!".to_string()),
        (Value::Scalar(_), Value::Vector(_)) => return Err("Can't subtract vector from scalar!".to_string()),
        (Value::Matrix(_), Value::Scalar(_)) => return Err("Can't subtract scalar from matrix!".to_string()),
        (Value::Scalar(_), Value::Matrix(_)) => return Err("Can't subtract matrix from scalar!".to_string()),
        (Value::Vector(_), Value::Matrix(_)) => return Err("Can't subtract matrix from vector!".to_string()),
        (Value::Matrix(_), Value::Vector(_)) => return Err("Can't subtract vector from matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn mult(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return mult_div::ssmult(a, b),
        (Value::Vector(a), Value::Scalar(b)) => return mult_div::svmult(b, a),
        (Value::Scalar(a), Value::Vector(b)) => return mult_div::svmult(a, b),
        (Value::Scalar(a), Value::Matrix(b)) => return mult_div::smmult(a, b),
        (Value::Matrix(a), Value::Scalar(b)) => return mult_div::smmult(b, a),
        (Value::Matrix(a), Value::Matrix(b)) => return mult_div::mmmult(a, b),
        (Value::Vector(a), Value::Vector(b)) => return mult_div::vvmult(a, b),
        (Value::Matrix(a), Value::Vector(b)) => return mult_div::mvmult(a, b),
        (Value::Vector(_), Value::Matrix(_)) => return Err("Vector has to be on the right side of linear transformation!".to_string())
    }
}

#[doc(hidden)]
pub fn neg(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(-1.*a)),
        Value::Vector(a) => return Ok(Value::Vector(a.iter().map(|x| -1.*x).collect())),
        Value::Matrix(a) => return Ok(Value::Matrix(a.iter().map(|x| x.iter().map(|y| -1.*y).collect()).collect()))
    }
}

#[doc(hidden)]
pub fn div(lv: &Value, rv: &Value) -> Result<Value, String> {
    match(lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return mult_div::ssdiv(a, b),
        (Value::Vector(a), Value::Scalar(b)) => return mult_div::vsdiv(a, b),
        (Value::Matrix(a), Value::Scalar(b)) => return mult_div::msdiv(a, b),
        (Value::Vector(a), Value::Vector(b)) => return mult_div::vvdiv(a, b),
        (Value::Scalar(_), Value::Vector(_)) => return Err("Can't divide scalar by vector!".to_string()),
        (Value::Scalar(_), Value::Matrix(_)) => return Err("Can't divide scalar by matrix!".to_string()),
        (Value::Matrix(_), Value::Vector(_)) => return Err("Can't divide matrix by vector!".to_string()),
        (Value::Vector(_), Value::Matrix(_)) => return Err("Can't divide vector by matrix!".to_string()),
        (Value::Matrix(_), Value::Matrix(_)) => return Err("Can't divide matrix by matrix!".to_string()),
    }
}

#[doc(hidden)]
pub fn cross(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv){
        (Value::Vector(a), Value::Vector(b)) => return cross_pow::vcross(a, b),
        _ => return Err("Cross product can only be computed between two vectors!".to_string())
    }
}

#[doc(hidden)]
pub fn get(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Vector(a), Value::Scalar(b)) => {
            if b % 1. != 0. || b.is_sign_negative() {
                return Err("Index must be a positive Integer!".to_string());
            }
            if *b as usize > a.len() - 1 {
                return Err("Index out of bounds for vector!".to_string());
            }
            return Ok(Value::Scalar(a[*b as usize]));
        },
        _ => return Err("Can only index vector with scalar!".to_string())
    }
}

#[doc(hidden)]
pub fn pow(lv: &Value, rv: &Value) -> Result<Value, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return cross_pow::sspow(a, b),
        _ => return Err("Can only raise scalar to the power of scalar!".to_string())
    }
}

#[doc(hidden)]
pub fn sin(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.sin())),
        Value::Vector(_) => return Err("Can't take sin of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take sin of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn cos(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.cos())),
        Value::Vector(_) => return Err("Can't take cos of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take cos of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn tan(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.tan())),
        Value::Vector(_) => return Err("Can't take tan of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take tan of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn arcsin(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.asin())),
        Value::Vector(_) => return Err("Can't take arcsin of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take arcsin of matrxi!".to_string())
    }
}

#[doc(hidden)]
pub fn arccos(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.acos())),
        Value::Vector(_) => return Err("Can't take arccos of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take arccos of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn arctan(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.atan())),
        Value::Vector(_) => return Err("Can't take arctan of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take arctan of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn abs(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => {
            if *a < 0. {return Ok(Value::Scalar(a*(-1.)));}
            else {return Ok(Value::Scalar(*a));}
        },
        Value::Vector(a) => {
            let mut sum = 0.;
            for i in a {
                sum += i.powi(2);
            }
            return Ok(Value::Scalar(sum.sqrt()));
        },
        Value::Matrix(_) => return Err("Can't take abs of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn sqrt(lv: &Value) -> Result<Vec<Value>, String> {
    match lv {
        Value::Scalar(a) => return Ok(vec![Value::Scalar(a.sqrt()), Value::Scalar(-1. * a.sqrt())]),
        Value::Vector(_) => return Err("Can't take sqrt of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take sqrt of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn root(lv: &Value, rv: &Value) -> Result<Vec<Value>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => {
            if b % 2. == 0. {
                return Ok(vec![Value::Scalar(a.powf(1./b)), Value::Scalar(-1. * a.powf(1./b))]);
            } else {
                return Ok(vec![Value::Scalar(a.powf(1./b))]);
            }
        },
        _ => return Err("Can only take root of a scalar!".to_string())
    }
}

#[doc(hidden)]
pub fn ln(lv: &Value) -> Result<Value, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.ln())),
        Value::Vector(_) => return Err("Can't take ln of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take ln of matrix!".to_string())
    }
}
