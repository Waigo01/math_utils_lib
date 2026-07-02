use crate::{Value, maths::num_traits::Number};

#[doc(hidden)]
pub fn det_m<N: Number>(a: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    if a.iter().filter(|r| r.len() != a[0].len()).count() != 0 || a.len() != a[0].len() {
        return Err("can't calculate determinant of a non-square matrix".to_string());
    } else if a.len() == 1 {
        return Ok(Value::Scalar(a[0][0]));
    } else if a.len() == 2 {
        return Ok(Value::Scalar(a[0][0]*a[1][1]-a[0][1]*a[1][0]));
    } else {
        let mut sum = N::zero();
        for i in 0..a[0].len() {
            let new_matrix = a[1..].iter().map(|r| r[0..i].iter().cloned().chain(r[i+1..].iter().cloned()).collect()).collect::<Vec<Vec<N>>>();
            sum = sum + (-N::one()).powi(i as i32)*a[0][i]*det_m(&new_matrix)?.get_scalar().unwrap();
        }
        return Ok(Value::Scalar(sum));
    }
}

#[doc(hidden)]
pub fn inv_m<N: Number>(a: &Vec<Vec<N>>) -> Result<Value<N>, String> {
    match det_m(a) {
        Err(_) => return Err("can't calculate inverse of a non-square matrix".to_string()),
        Ok(Value::Scalar(s)) if s == N::zero() => return Err("can't calculate inverse of a matrix with determinant 0".to_string()),
        _ => {}
    };

    let n = a.len();

    let mut v = a.clone();

    for i in 0..n {
        for j in 0..n {
            if j == i {
                v[i].push(N::one());
            } else {
                v[i].push(N::zero());
            }
        }
    }

    for i in 0..v.len() - 1{
        for j in (i+1)..v.len() {
            let divisor = v[i][i]/v[j][i];
            let mut zero_line = true;
            for k in i..v[j].len() {
                v[j][k] = v[j][k] - v[i][k]/divisor; 
                if v[j][k] != N::zero() {
                    zero_line = false;
                }
            }
            if zero_line {
                return Err("infinite solutions".to_string());
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
                v[j][k] = v[j][k] - v[i][k]/divisor;
                if v[j][k] != N::zero() {
                    zero_line = false;
                }
            }
            if zero_line {
                return Err("infinite solutions".to_string());
            }
        }
    }

    let mut result_mat: Vec<Vec<N>> = vec![];

    for i in 0..v.len() {
        let mut row = vec![];
        let mult = v[i][i].recip();
        for j in v[i].len()-n..v[i].len() {
             row.insert(0, v[i][j]*mult);
        }
        result_mat.insert(0, row);
    }

    Ok(Value::Matrix(result_mat))
}
