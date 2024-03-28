use crate::basetypes::Value;

#[doc(hidden)]
pub fn sadd(a: f64, b: f64) -> Result<Value, String> {
    Ok(Value::Scalar(a + b))
}

#[doc(hidden)]
pub fn vadd(a: Vec<f64>, b: Vec<f64>) -> Result<Value, String> { 
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
pub fn madd(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Result<Value, String> {
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
pub fn vsub(a: Vec<f64>, mut b: Vec<f64>) -> Result<Value, String> {
    for i in 0..b.len() {
        b[i] *= -1.;
    }
    vadd(a, b)
}

#[doc(hidden)]
pub fn msub(a: Vec<Vec<f64>>, mut b: Vec<Vec<f64>>) -> Result<Value, String> {
    for i in 0..b.len() {
        for j in 0..b[0].len() {
            b[i][j] *= -1.;
        }
    }
    madd(a, b)
}
