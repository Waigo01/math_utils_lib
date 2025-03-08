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

#[doc(hidden)]
pub fn inv_m(a: &Vec<Vec<f64>>) -> Result<Value, String> {
    match det_m(a) {
        Err(_) => return Err("Can't calculate inverse of a non-square matrix!".to_string()),
        Ok(Value::Scalar(0.)) => return Err("Can't calculate inverse of a matrix with determinant 0!".to_string()),
        _ => {}
    };

    let n = a.len();

    let mut v = a.clone();

    for i in 0..n {
        for j in 0..n {
            if j == i {
                v[i].push(1.);
            } else {
                v[i].push(0.);
            }
        }
    }

    for i in 0..v.len() - 1{
        for j in (i+1)..v.len() {
            let divisor = v[i][i]/v[j][i];
            let mut zero_line = true;
            for k in i..v[j].len() {
                v[j][k] -= v[i][k]/divisor; 
                if v[j][k] != 0. {
                    zero_line = false;
                }
            }
            if zero_line {
                return Err("Infinite solutions".to_string());
            }
        }
    }

    v.reverse();

    v.iter_mut().for_each(|x| x.reverse());

    for i in 0..v.len() {
        for _ in 0..n {
            let value = v[i].remove(0);
            v[i].push(value);
        }
    }

    for i in 0..v.len() - 1 {
        for j in (i+1)..v.len() {
            let divisor = v[i][i]/v[j][i];
            let mut zero_line = true;
            for k in i..v[j].len() {
                v[j][k] -= v[i][k]/divisor;
                if v[j][k] != 0. {
                    zero_line = false;
                }
            }
            if zero_line {
                return Err("Infinite solutions".to_string());
            }
        }
    }

    let mut result_mat: Vec<Vec<f64>> = vec![];

    for i in 0..v.len() {
        let mut row = vec![];
        let mult = 1. / v[i][i];
        for j in v[i].len()-n..v[i].len() {
             row.insert(0, v[i][j]*mult);
        }
        result_mat.insert(0, row);
    }

    Ok(Value::Matrix(result_mat))
}
