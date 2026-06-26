use crate::{basetypes::Value, maths::num_traits::Number};

#[doc(hidden)]
pub fn sadd<N: Number>(a: &N, b: &N) -> Result<Value<N>, String> {
    Ok(Value::Scalar(*a + *b))
}

#[doc(hidden)]
pub fn vadd<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> { 
    if a.len() != b.len() {
        return Err(format!("Vectors have different dimensions!"));
    }
    let mut output_v = vec![];
    for i in 0..a.len() {
        output_v.push(a[i]+b[i]);
    }
    return Ok(Value::Vector(output_v));
}

#[doc(hidden)]
pub fn madd<N: Number>(a: &Vec<Vec<N>>, b: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    if a.len() != b.len() || a[0].len() != b[0].len() {
        return Err(format!("Matrices have different dimensions!"));
    }
    let mut output_m = vec![];
    for i in 0..a.len() {
        let mut row = vec![];
        for j in 0..a[0].len() {
            row.push(a[i][j] + b[i][j])
        }
        output_m.push(row);
    }
    return Ok(Value::Matrix(output_m));
}

#[doc(hidden)]
pub fn vsub<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> {
    let mut b_neg = vec![];
    for i in 0..b.len() {
        b_neg.push(b[i] * -N::one());
    }
    vadd(a, &b_neg)
}

#[doc(hidden)]
pub fn msub<N: Number>(a: &Vec<Vec<N>>, b: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    let mut b_neg = vec![];
    for i in 0..b.len() {
        let mut r_neg = vec![];
        for j in 0..b[0].len() {
            r_neg.push(b[i][j] * -N::one());
        }
        b_neg.push(r_neg);
    }
    madd(a, &b_neg)
}
