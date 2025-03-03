use crate::Value;

#[doc(hidden)]
pub fn det_m(a: &Vec<Vec<f64>>) -> Result<Value, String> {
    if a.iter().filter(|r| r.len() != a[0].len()).count() != 0 || a.len() != a[0].len() {
        return Err("Can't calculate determinant of a non-square matrix!".to_string());
    } else if a.len() == 1 {
        return Ok(Value::Scalar(a[0][0]));
    } else if a.len() == 2 {
        return Ok(Value::Scalar(a[0][0]*a[1][1]-a[0][1]*a[1][0]));
    } else {
        let mut sum: f64 = 0.0;
        for i in 0..a[0].len() {
            let new_matrix = a[1..].iter().map(|r| r[0..i].iter().cloned().chain(r[i+1..].iter().cloned()).collect()).collect::<Vec<Vec<f64>>>();
            sum += (-1f64).powi(i as i32)*a[0][i]*det_m(&new_matrix)?.get_scalar().unwrap();
        }
        return Ok(Value::Scalar(sum));
    }
}
