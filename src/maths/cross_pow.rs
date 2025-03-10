use crate::basetypes::Value;

use super::mult_div::mmmult;

#[doc(hidden)]
pub fn vcross(a: &Vec<f64>, b: &Vec<f64>) -> Result<Value, String> {
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
            expand_va.push(0.);
            expand_vb.push(0.);
        }
    }

    output_v.push(expand_va[1] * expand_vb[2] - expand_va[2] * expand_vb[1]);
    output_v.push(expand_va[2] * expand_vb[0] - expand_va[0] * expand_vb[2]);
    output_v.push(expand_va[0] * expand_vb[1] - expand_va[1] * expand_vb[0]);

    return Ok(Value::Vector(output_v));
}

#[doc(hidden)]
pub fn sspow(a: &f64, b: &f64) -> Result<Value, String> {
    return Ok(Value::Scalar(a.powf(*b)));
}

#[doc(hidden)]
pub fn mspow(a: &Vec<Vec<f64>>, b: &f64) -> Result<Value, String> {
    if b.round() != *b {
        return Err("Exponent must be an integer!".to_string());
    }
    let mut mult = vec![];
    for i in 0..a.len() {
        let mut row = vec![];
        for j in 0..a[0].len() {
            if i == j {
                row.push(1.);
            } else {
                row.push(0.);
            }
        }
        mult.push(row);
    }
    for _ in 0..(*b as i32) {
        mult = mmmult(&mult, a)?.get_matrix().unwrap();
    }

    Ok(Value::Matrix(mult))
}
