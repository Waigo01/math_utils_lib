use crate::{basetypes::Value, maths::num_traits::{Number, StandardFunctions}};

pub mod add_sub;
pub mod mult_div;
pub mod cross_pow;
pub mod calculus;
pub mod special;
pub mod bool;
pub mod num_traits;
pub mod num_impls;

#[doc(hidden)]
pub fn add<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return add_sub::sadd(a, b),
        (Value::Vector(a), Value::Vector(b)) => return add_sub::vadd(a, b),
        (Value::Matrix(a), Value::Matrix(b)) => return add_sub::madd(a, b),
        (Value::Vector(_), Value::Scalar(_)) => return Err("can't add scalar to vector".to_string()),
        (Value::Scalar(_), Value::Vector(_)) => return Err("can't add vector to scalar".to_string()),
        (Value::Matrix(_), Value::Scalar(_)) => return Err("can't add scalar to matrix".to_string()),
        (Value::Scalar(_), Value::Matrix(_)) => return Err("can't add matrix to scalar".to_string()),
        (Value::Vector(_), Value::Matrix(_)) => return Err("can't add matrix to vector".to_string()),
        (Value::Matrix(_), Value::Vector(_)) => return Err("can't add vector to matrix".to_string())
    }
}

#[doc(hidden)]
pub fn sub<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return add_sub::sadd(a, &(*b * -N::one())),
        (Value::Vector(a), Value::Vector(b)) => return add_sub::vsub(a, b),
        (Value::Matrix(a), Value::Matrix(b)) => return add_sub::msub(a, b),
        (Value::Vector(_), Value::Scalar(_)) => return Err("can't subtract scalar from vector".to_string()),
        (Value::Scalar(_), Value::Vector(_)) => return Err("can't subtract vector from scalar".to_string()),
        (Value::Matrix(_), Value::Scalar(_)) => return Err("can't subtract scalar from matrix".to_string()),
        (Value::Scalar(_), Value::Matrix(_)) => return Err("can't subtract matrix from scalar".to_string()),
        (Value::Vector(_), Value::Matrix(_)) => return Err("can't subtract matrix from vector".to_string()),
        (Value::Matrix(_), Value::Vector(_)) => return Err("can't subtract vector from matrix".to_string())
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
        (Value::Vector(_), Value::Matrix(_)) => return Err("vector has to be on the right side of linear transformation".to_string())
    }
}

#[doc(hidden)]
pub fn neg<N: Number>(lv: &Value<N>) -> Result<Value<N>, String> {
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(-N::one()* *a)),
        Value::Vector(a) => return Ok(Value::Vector(a.iter().map(|x| -N::one()* *x).collect())),
        Value::Matrix(a) => return Ok(Value::Matrix(a.iter().map(|x| x.iter().map(|y| -N::one()* *y).collect()).collect()))
    }
}

#[doc(hidden)]
pub fn div<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match(lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return mult_div::ssdiv(a, b),
        (Value::Vector(a), Value::Scalar(b)) => return mult_div::vsdiv(a, b),
        (Value::Matrix(a), Value::Scalar(b)) => return mult_div::msdiv(a, b),
        (Value::Vector(a), Value::Vector(b)) => return mult_div::vvdiv(a, b),
        (Value::Scalar(_), Value::Vector(_)) => return Err("can't divide scalar by vector".to_string()),
        (Value::Scalar(_), Value::Matrix(_)) => return Err("can't divide scalar by matrix".to_string()),
        (Value::Matrix(_), Value::Vector(_)) => return Err("can't divide matrix by vector".to_string()),
        (Value::Vector(_), Value::Matrix(_)) => return Err("can't divide vector by matrix".to_string()),
        (Value::Matrix(_), Value::Matrix(_)) => return Err("can't divide matrix by matrix".to_string()),
    }
}

#[doc(hidden)]
pub fn cross<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv){
        (Value::Vector(a), Value::Vector(b)) => return cross_pow::vcross(a, b),
        _ => return Err("cross product can only be computed between two vectors".to_string())
    }
}

#[doc(hidden)]
pub fn get<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Vector(a), Value::Scalar(b)) => {
            let rounded_b = match b.as_rounded_int() {
                Ok(r) if r.is_negative() => return Err("index must be a positive integer".to_string()),
                Ok(r) => r,
                Err(_) => return Err("index must be a positive integer".to_string())
            };
            if rounded_b > (a.len() - 1) as i32 {
                return Err("index out of bounds for vector".to_string());
            }
            return Ok(Value::Scalar(a[rounded_b as usize]));
        },
        _ => return Err("can only index vector with scalar".to_string())
    }
}

#[doc(hidden)]
pub fn pow<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => return cross_pow::sspow(a, b),
        (Value::Matrix(m), Value::Scalar(b)) => return cross_pow::mspow(m, b),
        _ => return Err("can only raise scalar or matrix to the power of scalar".to_string())
    }
}

#[doc(hidden)]
pub fn tetration<N: Number>(lv: &Value<N>, rv: &Value<N>) -> Result<Value<N>, String> {
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => {
            if let Ok(lexp) = b.as_rounded_int() {
                let base = a.clone();
                let mut a = a.clone();
                for _ in 0..lexp {
                    a = a.powf(base);
                }
                return Ok(Value::Scalar(a));
            } else {
                return Err("can only tetrate using an integer left exponential".to_string());
            }
        },
        _ => return Err("can only tetrate using a scalar left exponential".to_string())
    }
}

#[doc(hidden)]
pub fn sin<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("sin requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.sin())),
        Value::Vector(_) => return Err("can't take sin of vector".to_string()),
        Value::Matrix(_) => return Err("can't take sin of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn cos<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("Cos requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.cos())),
        Value::Vector(_) => return Err("can't take cos of vector".to_string()),
        Value::Matrix(_) => return Err("can't take cos of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn tan<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("tan requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.tan())),
        Value::Vector(_) => return Err("can't take tan of vector".to_string()),
        Value::Matrix(_) => return Err("can't take tan of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn sinh<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("sinh requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.sinh())),
        Value::Vector(_) => return Err("can't take sinh of vector".to_string()),
        Value::Matrix(_) => return Err("can't take sinh of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn cosh<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("cosh requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.cosh())),
        Value::Vector(_) => return Err("can't take cosh of vector".to_string()),
        Value::Matrix(_) => return Err("can't take cosh of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn tanh<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("tanh requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.tanh())),
        Value::Vector(_) => return Err("can't take tanh of vector".to_string()),
        Value::Matrix(_) => return Err("can't take tanh of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn exp<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("exp requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.exp())),
        Value::Vector(_) => return Err("can't take exp of vector".to_string()),
        Value::Matrix(_) => return Err("can't take exp of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn fact<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("fact requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.fact())),
        Value::Vector(_) => return Err("can't take fact of vector".to_string()),
        Value::Matrix(_) => return Err("can't take fact of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn arcsin<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("arcsin requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.asin())),
        Value::Vector(_) => return Err("can't take arcsin of vector".to_string()),
        Value::Matrix(_) => return Err("can't take arcsin of matrxi".to_string())
    }
}

#[doc(hidden)]
pub fn arccos<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("arccos requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.acos())),
        Value::Vector(_) => return Err("can't take arccos of vector".to_string()),
        Value::Matrix(_) => return Err("can't take arccos of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn arctan<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("arctan requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.atan())),
        Value::Vector(_) => return Err("can't take arctan of vector".to_string()),
        Value::Matrix(_) => return Err("can't take arctan of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn abs<N: Number>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("abs requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => {
            if *a < N::zero() {return Ok(Value::Scalar(*a* -N::one()));}
            else {return Ok(Value::Scalar(*a));}
        },
        Value::Vector(a) => {
            let mut sum = N::zero();
            for i in a {
                sum = sum + i.powi(2);
            }
            return Ok(Value::Scalar(sum.sqrt()));
        },
        Value::Matrix(_) => return Err("can't take abs of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn sqrt<N: Number>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("sqrt requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.sqrt())),
        Value::Vector(_) => return Err("can't take sqrt of vector".to_string()),
        Value::Matrix(_) => return Err("can't take sqrt of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn root<N: Number>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("root requires exactly two arguments".to_string());
    };
    let Some(rv) = args.get(1) else {
        return Err("root requires exactly two arguments".to_string());
    };
    match (lv, rv) {
        (Value::Scalar(a), Value::Scalar(b)) => {
            return Ok(Value::Scalar(a.powf(b.recip())));
        },
        _ => return Err("can only take root of a scalar".to_string())
    }
}

#[doc(hidden)]
pub fn ln<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("sin requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(a) => return Ok(Value::Scalar(a.ln())),
        Value::Vector(_) => return Err("can't take ln of vector".to_string()),
        Value::Matrix(_) => return Err("can't take ln of matrix".to_string())
    }
}

#[doc(hidden)]
pub fn det<N: Number + StandardFunctions>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("sin requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(_) => return Err("can't calculate determinant of a scalar".to_string()),
        Value::Vector(_) => return Err("can't calculate determinant of a vector".to_string()),
        Value::Matrix(m) => return special::det_m(m),
    }
}

#[doc(hidden)]
pub fn inv<N: Number>(args: Vec<Value<N>>) -> Result<Value<N>, String> {
    let Some(lv) = args.get(0) else {
        return Err("sin requires exactly one argument".to_string());
    };
    match lv {
        Value::Scalar(_) => return Err("can't calculate inverse of a scalar".to_string()),
        Value::Vector(_) => return Err("can't calculate inverse of a vector".to_string()),
        Value::Matrix(m) => return special::inv_m(m),
    }
}
