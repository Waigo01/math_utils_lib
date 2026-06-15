use crate::{basetypes::Value, maths::num_traits::Number};

pub mod add_sub;
pub mod mult_div;
pub mod cross_pow;
pub mod calculus;
pub mod special;
pub mod bool;
pub mod num_traits;

#[doc(hidden)]
pub fn add<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
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
pub fn sub<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return add_sub::sadd(a, &(*b * -N::ONE)),
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
pub fn mult<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
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
pub fn neg<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(-N::ONE* *a)),
        Value::Vector(a) => return Ok(Value::Vector(a.iter().map(|x| -N::ONE* *x).collect())),
        Value::Matrix(a) => return Ok(Value::Matrix(a.iter().map(|x| x.iter().map(|y| -N::ONE* *y).collect()).collect()))
    }
}

#[doc(hidden)]
pub fn div<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
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
pub fn cross<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv){
        (Value::Vector(a), Value::Vector(b)) => return cross_pow::vcross(a, b),
        _ => return Err("Cross product can only be computed between two vectors!".to_string())
    }
}

#[doc(hidden)]
pub fn get<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Vector(a), Value::Scalar(b)) => {
            let rounded_b = match b.as_rounded_int() {
                Ok(r) if r.is_negative() => return Err("Indexd must be a positive Integer!".to_string()),
                Ok(r) => r,
                Err(_) => return Err("Index must be a positive Integer!".to_string())
            };
            if rounded_b > (a.len() - 1) as i32 {
                return Err("Index out of bounds for vector!".to_string());
            }
            return Ok(Value::Scalar(a[rounded_b as usize]));
        },
        _ => return Err("Can only index vector with scalar!".to_string())
    }
}

#[doc(hidden)]
pub fn pow<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return cross_pow::sspow(a, b),
        (Value::Matrix(m), Value::Scalar(b)) => return cross_pow::mspow(m, b),
        _ => return Err("Can only raise scalar or matrix to the power of scalar!".to_string())
    }
}

#[doc(hidden)]
pub fn sin<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.sin())),
        Value::Vector(_) => return Err("Can't take sin of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take sin of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn cos<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.cos())),
        Value::Vector(_) => return Err("Can't take cos of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take cos of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn tan<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.tan())),
        Value::Vector(_) => return Err("Can't take tan of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take tan of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn arcsin<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.asin())),
        Value::Vector(_) => return Err("Can't take arcsin of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take arcsin of matrxi!".to_string())
    }
}

#[doc(hidden)]
pub fn arccos<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.acos())),
        Value::Vector(_) => return Err("Can't take arccos of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take arccos of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn arctan<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.atan())),
        Value::Vector(_) => return Err("Can't take arctan of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take arctan of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn abs<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => {
            if *a < N::ZERO {return Ok(Value::Scalar(*a* -N::ONE));}
            else {return Ok(Value::Scalar(*a));}
        },
        Value::Vector(a) => {
            let mut sum = N::ZERO;
            for i in a {
                sum = sum + i.powi(2);
            }
            return Ok(Value::Scalar(sum.sqrt()));
        },
        Value::Matrix(_) => return Err("Can't take abs of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn sqrt<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.sqrt())),
        Value::Vector(_) => return Err("Can't take sqrt of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take sqrt of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn root<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => {
            return Ok(Value::Scalar(a.powf(b.recip())));
        },
        _ => return Err("Can only take root of a scalar!".to_string())
    }
}

#[doc(hidden)]
pub fn ln<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.ln())),
        Value::Vector(_) => return Err("Can't take ln of vector!".to_string()),
        Value::Matrix(_) => return Err("Can't take ln of matrix!".to_string())
    }
}

#[doc(hidden)]
pub fn det<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(_) => return Err("Can't calculate determinant of a scalar!".to_string()),
        Value::Vector(_) => return Err("Can't calculate determinant of a vector!".to_string()),
        Value::Matrix(m) => return special::det_m(m),
    }
}

#[doc(hidden)]
pub fn inv<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(_) => return Err("Can't calculate inverse of a scalar!".to_string()),
        Value::Vector(_) => return Err("Can't calculate inverse of a vector!".to_string()),
        Value::Matrix(m) => return special::inv_m(m),
    }
}
