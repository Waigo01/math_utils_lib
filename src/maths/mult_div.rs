use crate::basetypes::Value;

#[doc(hidden)]
pub fn ssmult(a: f64, b: f64) -> Result<Value, String> {
    Ok(Value::Scalar(a*b))
}

#[doc(hidden)]
pub fn svmult(a: f64, b: Vec<f64>) -> Result<Value, String> {
    let mut output_v = vec![];
    for i in b {
        output_v.push(i*a);
    }
    Ok(Value::Vector(output_v))
}

#[doc(hidden)]
pub fn smmult(a: f64, b: Vec<Vec<f64>>) -> Result<Value, String> {
    let mut output_m = vec![];
    for i in 0..b.len() {
        let mut row = vec![];
        for j in 0..b[0].len() {
            row.push(b[i][j]*a);
        }
        output_m.push(row);
    }
    Ok(Value::Matrix(output_m))
}

#[doc(hidden)]
pub fn vvmult(a: Vec<f64>, b: Vec<f64>) -> Result<Value, String> {
    if a.len() != b.len() {
        return Err("Vectors have different dimensions!".to_string());
    }
    let mut sum = 0f64;
    for i in 0..a.len() {
        sum += a[i]*b[i];
    }
    return Ok(Value::Scalar(sum));
}

#[doc(hidden)]
pub fn mvmult(a: Vec<Vec<f64>>, b: Vec<f64>) -> Result<Value, String> {
    if a[0].len() != b.len() {
        return Err("Vector and matrix have incompatible dimensions!".to_string());
    }
    let mut output_v = vec![];
    for i in 0..a.len() {
        let mut sum = 0f64;
        for j in 0..a[i].len() {
            sum += a[i][j]*b[j]
        }
        output_v.push(sum);
    }
    return Ok(Value::Vector(output_v));
}

#[doc(hidden)]
pub fn mmmult(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>) -> Result<Value, String> {
    if a[0].len() != b.len() {
        return Err("Matrices have incompatible dimensions".to_string());
    }
    let mut output_m = vec![];
    for i in 0..a.len() {
        let mut row = vec![];
        for j in 0..b[0].len() {
            let mut sum = 0f64;
            for k in 0..a[0].len() {
                sum += a[i][k]*b[k][j]
            }
            row.push(sum);
        }
        output_m.push(row);
    }
    return Ok(Value::Matrix(output_m))
}

#[doc(hidden)]
pub fn ssdiv(a: f64, b: f64) -> Result<Value, String> {
    return Ok(Value::Scalar(a/b));
}

#[doc(hidden)]
pub fn vsdiv(a: Vec<f64>, b: f64) -> Result<Value, String> {
    return svmult(1f64/b, a);
}

#[doc(hidden)]
pub fn msdiv(a: Vec<Vec<f64>>, b: f64) -> Result<Value, String> {
    return smmult(1f64/b, a);
}
