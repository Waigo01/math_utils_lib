use crate::{basetypes::{Value, Variable}, errors::{EvalError, SolveError}, maths, parser::{eval, Binary}, PREC};

fn clean_results(res: Vec<Value>) -> Vec<Value> {
    if res.len() == 0 {
        return vec![];
    }
    let mut new_res: Vec<Value> = vec![];
    for i in &res {
        let mut found = false;
        for j in &new_res {
            if i.round(PREC) == j.round(PREC) {
                found = true;
                break;
            }
        }
        if !found {
            new_res.push(i.clone());
        }
    }
    match res[0] {
        Value::Scalar(_) => {
            if new_res.len() > 10 {
                new_res.sort_by(|a, b| a.get_scalar().unwrap().abs().partial_cmp(&b.get_scalar().unwrap().abs()).unwrap());
                new_res = new_res[0..10].to_vec();
                new_res.sort_by(|a, b| a.get_scalar().unwrap().partial_cmp(&b.get_scalar().unwrap()).unwrap());
            }
        },
        Value::Vector(_) => {
            if new_res.len() > 10 {
                new_res.sort_by(|a, b| a.get_vector().unwrap()[0].abs().partial_cmp(&b.get_vector().unwrap()[0].abs()).unwrap());
                new_res = new_res[0..10].to_vec();
                new_res.sort_by(|a, b| a.get_vector().unwrap()[0].partial_cmp(&b.get_vector().unwrap()[0]).unwrap());
            }
        },
        Value::Matrix(_) => {}
    } 
    return new_res;
}

///used to find the roots of a given mathematical expression in the form of a
///Binary Tree in the context of the
///provided variables and a Variable Name in terms of which is to be solved.
pub fn find_roots(expr: Binary, mut vars: Vec<Variable>, var_name: String) -> Result<Vec<Value>, SolveError> {
    match expr {
        Binary::Value(_) => return Err(SolveError::NothingToDo),
        Binary::Variable(_) => return Err(SolveError::NothingToDo),
        Binary::Operation(_) => {}
    }

    let mut results = vec![];

    for i in -1000..1000 {
        let mut x = Value::Scalar(i as f64);
        for _ in 0..1000 {
            let finished = calc_newton(&mut x, &expr, &mut vars, var_name.clone())?;
            if finished {
                results.push(x);
                break;
            }
            if x.is_inf_or_nan() {
                break;
            }
        }
    }

    let clean_res = clean_results(results);

    return Ok(clean_res);
}

fn calc_newton(x: &mut Value, expr: &Binary, vars: &mut Vec<Variable>, var_name: String) -> Result<bool, SolveError> {
    vars.push(Variable::new(var_name.to_string(), x.to_owned()));
    let fx = eval(&expr, vars)?;
    vars.remove(vars.len()-1);
    match fx {
        Value::Scalar(s) => {
            if (s*10f64.powf(PREC)).round()/10f64.powf(PREC) == 0. {
                return Ok(true);
            }
        },
        Value::Vector(ref v) => {
            let mut sum = 0.;
            for i in v {
                sum += (i*10f64.powf(PREC)).round()/10f64.powf(PREC);
            }
            if sum == 0. {
                return Ok(true)
            }
        },
        Value::Matrix(ref m) => {
            let mut sum = 0.;
            for i in m {
                for j in i {
                    sum += (j*10f64.powf(PREC)).round()/10f64.powf(PREC);
                }
            }
            if sum == 0. {
                return Ok(true)
            }
        }
    }

    let fxd = maths::calculus::calculate_derivative(expr, var_name.to_string(), x.clone(), Some(fx.clone()), vars)?;

    let div = match maths::div(fx, fxd) {
        Ok(v) => v,
        Err(s) => return Err(EvalError::MathError(s).into())
    };

    let sub = match maths::sub(x.clone(), div) {
        Ok(v) => v,
        Err(s) => return Err(EvalError::MathError(s).into())
    };

    *x = sub;
    return Ok(false);
}
