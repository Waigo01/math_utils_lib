use crate::{basetypes::{Value, Variable}, errors::{SolveError, SolveErrorCode}, parser::{eval, Binary}};

#[cfg(feature = "high-prec-solve")]
const PREC: f64 = 1e13;
#[cfg(not(feature = "high-prec-solve"))]
const PREC: f64 = 1e8;

fn clean_results(res: Vec<f64>) -> Vec<f64> {
    let mut new_res: Vec<f64> = vec![];
    for i in res {
        let mut found = false;
        for j in &new_res {
            if (i*1e5).round()/1e5 == (j*1e5).round()/1e5 {
                found = true;
                break;
            }
        }
        if !found {
            new_res.push(i);
        }
    }
    if new_res.len() > 10 {
        new_res.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
        new_res = new_res[0..10].to_vec();
        new_res.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    }
    return new_res;
}

///used to find the roots of a given mathematical expression in the form of a
///Binary Tree in the context of the
///provided variables and a Variable Name in terms of which is to be solved.
pub fn find_roots(expr: Binary, mut vars: Vec<Variable>, var_name: &str) -> Result<Vec<Value>, SolveError> {
    match expr {
        Binary::Value(_) => {return Err(SolveError { code: SolveErrorCode::NothingToDo, reason: "Nothing to do!".to_string() });},
        Binary::Variable(_) => {return Err(SolveError { code: SolveErrorCode::NothingToDo, reason: "Nothing to do!".to_string() });},
        Binary::Operation(_) => {}
    }
    vars.push(Variable { name: var_name.to_string(), value: Value::Scalar(0.) });
    match eval(&expr, &vars)? {
        Value::Scalar(_) => {},
        Value::Vector(_) => {return Err(SolveError { code: SolveErrorCode::VectorInEq, reason: "Can't have vectors in equations".to_string() });},
        Value::Matrix(_) => {return Err(SolveError { code: SolveErrorCode::MatrixInEq, reason: "Can't have matrices in equations!".to_string() });}
    }
    vars.remove(vars.len()-1);

    let mut results = vec![];

    for i in -1000..1000 {
        let mut x = i as f64;
        for _ in 0..1000 {
            let finished = calc_newton(&mut x, &expr, &mut vars, var_name)?;
            if finished {
                results.push(x);
                break;
            }
            if x.is_nan() || x.is_infinite() {
                break;
            }
        }
    }

    let clean_res = clean_results(results);
    let clean_res_values = clean_res.iter().map(|x| Value::Scalar(*x)).collect();

    return Ok(clean_res_values);
}

fn calc_newton(x: &mut f64, expr: &Binary, vars: &mut Vec<Variable>, var_name: &str) -> Result<bool, SolveError> {
    vars.push(Variable {name: var_name.to_string(), value: Value::Scalar(*x)});
    let fx = eval(&expr, &vars)?.get_scalar();
    vars.remove(vars.len()-1);
    if (fx*PREC).round()/PREC == 0. {
        return Ok(true);
    }
    vars.push(Variable {name: var_name.to_string(), value: Value::Scalar(*x+1e-5)});
    let fxh = eval(&expr, &vars)?.get_scalar();
    *x = *x - (fx/((fxh-fx)/1e-5));
    vars.remove(vars.len()-1);
    return Ok(false);
}
