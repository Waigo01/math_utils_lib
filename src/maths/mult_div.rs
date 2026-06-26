use crate::{basetypes::Value, maths::num_traits::Number};

#[doc(hidden)]
pub fn ssmult<N: Number>(a: &N, b: &N) -> Result<Value<N>, String> {
    Ok(Value::Scalar(*a**b))
}

#[doc(hidden)]
pub fn svmult<N: Number>(a: &N, b: &Vec<N>) -> Result<Value<N>, String> {
    let mut output_v = vec![];
    for i in b {
        output_v.push(*i**a);
    }
    Ok(Value::Vector(output_v))
}

#[doc(hidden)]
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
pub fn vvmult<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a.len() != b.len() {
        return Err("Vectors have different dimensions!".to_string());
    }
    let mut sum = N::zero();
    for i in 0..a.len() {
        sum = sum + a[i]*b[i];
    }
    return Ok(Value::Scalar(sum));
}

#[doc(hidden)]
pub fn mvmult<N: Number>(a: &Vec<Vec<N>>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a[0].len() != b.len() {
        return Err("Vector and matrix have incompatible dimensions!".to_string());
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
pub fn mmmult<N: Number>(a: &Vec<Vec<N>>, b: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    if a[0].len() != b.len() {
        return Err("Matrices have incompatible dimensions!".to_string());
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
pub fn ssdiv<N: Number>(a: &N, b: &N) -> Result<Value<N>, String> {
    return Ok(Value::Scalar(*a/ *b));
}

#[doc(hidden)]
pub fn vsdiv<N: Number>(a: &Vec<N>, b: &N) -> Result<Value<N>, String> {
    return svmult(&b.recip(), a);
}

#[doc(hidden)]
pub fn vvdiv<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a.len() != b.len() {
        return Err("Vectors have incompatible dimensions!".to_string());
    }
    let mut sum = N::zero();
    for i in 0..a.len() {
        sum = sum + a[i]/b[i];
    }

    return Ok(Value::Scalar(sum));
}

#[doc(hidden)]
pub fn msdiv<N: Number>(a: &Vec<Vec<N>>, b: &N) -> Result<Value<N>, String> {
    return smmult(&b.recip(), a);
}
