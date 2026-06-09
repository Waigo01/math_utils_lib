use crate::{basetypes::Value, maths::num_trait::Number};

use super::mult_div::mmmult;

#[doc(hidden)]
pub fn vcross<N: Number>(a: &Vec<N>, b: &Vec<N>) -> Result<Value<N>, String> {
    if a.len() != b.len() {
        return Err("Vectors have different dimensions!".to_string());
    }
    if a.len() > 3 {
        return Err("Can't compute cross product with dim(V) > 3!".to_string());
    }

    let mut output_v = vec![];
    let mut expand_va = vec![];
    let mut expand_vb = vec![];
    for i in 0..3 {
        if i < a.len() {
            expand_va.push(a[i]);
            expand_vb.push(b[i]);
        } else {
            expand_va.push(N::from(0.));
            expand_vb.push(N::from(0.));
        }
    }

    output_v.push(expand_va[1] * expand_vb[2] - expand_va[2] * expand_vb[1]);
    output_v.push(expand_va[2] * expand_vb[0] - expand_va[0] * expand_vb[2]);
    output_v.push(expand_va[0] * expand_vb[1] - expand_va[1] * expand_vb[0]);

    return Ok(Value::Vector(output_v));
}

#[doc(hidden)]
pub fn sspow<N: Number>(a: &N, b: &N) -> Result<Value<N>, String> {
    return Ok(Value::Scalar(a.powf(*b)));
}

#[doc(hidden)]
pub fn mspow<N: Number>(a: &Vec<Vec<N>>, b: &N) -> Result<Value<N>, String> {
    if b.round() != *b {
        return Err("Exponent must be an integer!".to_string());
    }
    let mut mult = vec![];
    for i in 0..a.len() {
        let mut row = vec![];
        for j in 0..a[0].len() {
            if i == j {
                row.push(N::from(1.));
            } else {
                row.push(N::from(0.));
            }
        }
        mult.push(row);
    }
    for _ in 0..(b.round().into() as i32) {
        mult = mmmult(&mult, a)?.get_matrix().unwrap();
    }

    Ok(Value::Matrix(mult))
}
