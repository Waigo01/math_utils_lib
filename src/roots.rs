use crate::{basetypes::{Value, Variable}, errors::SolveError, maths::{abs, calculus::calculate_derivative}, parser::{eval, AdvancedOperation, Binary, Operation}, PREC};

fn clean_results(res: Vec<Value>) -> Vec<Value> {
    if res.len() == 0 {
        return vec![];
    }
    let mut new_res: Vec<Value> = vec![];
    for i in &res {
        let mut found = false;
        for j in &new_res {
            if i.round(PREC-2.) == j.round(PREC-2.) {
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

fn find_vars_in_expr(b: &Binary, mut ov: Vec<String>) -> Vec<String> {
    match b {
        Binary::Variable(v) => {
            ov.push(v.to_string());
            return ov.to_owned();
        },
        Binary::Value(_) => {
            return ov.to_owned();
        },
        Binary::Operation(o) => {
            match &**o {
                Operation::SimpleOperation { left, right, .. } => {
                    let mut lvars = find_vars_in_expr(left, ov.clone());
                    let mut rvars = find_vars_in_expr(right, ov);
                    lvars.append(&mut rvars);
                    return lvars;
                },
                Operation::AdvancedOperation(ao) => {
                    match ao {
                        AdvancedOperation::Integral { upper_bound, lower_bound, .. } => {
                            let mut found_vars = vec![];
                            let upper_bound_recurse = find_vars_in_expr(upper_bound, ov.clone());
                            for i in upper_bound_recurse {
                                found_vars.push(i);
                            }
                            let lower_bound_recurse = find_vars_in_expr(lower_bound, ov);
                            for i in lower_bound_recurse {
                                found_vars.push(i);
                            }
                            return found_vars;
                        },
                        AdvancedOperation::Derivative { at, .. } => {
                            return find_vars_in_expr(at, ov);
                        } 
                    }
                }
            }
        }
    }
}

pub fn gauss_algorithm(mut v: Vec<Vec<f64>>) -> Result<Value, SolveError> {
    if v.len()+1 != v[0].len() {
        return Err(SolveError::UnderdeterminedSystem);
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
                return Err(SolveError::InfiniteSolutions);
            }
        }
    } 

    v.reverse();

    v.iter_mut().for_each(|x| x.reverse());

    let aug_col = v.iter().map(|x| x[0]).collect::<Vec<f64>>();

    for i in 0..v.len() {
        v[i].remove(0);
        v[i].push(aug_col[i]);
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
                return Err(SolveError::InfiniteSolutions);
            }
        }
    } 

    let mut result_vec = vec![];

    for i in 0..v.len() {
        result_vec.push(v[i][v[i].len()-1]/v[i][i]);
    }

    result_vec.reverse();

    return Ok(Value::Vector(result_vec));
}

fn jacobi_and_gauss(search_expres: &Vec<Binary>, x: &Vec<Variable>, vars: &mut Vec<Variable>, fx: Vec<f64>) -> Result<Vec<Variable>, SolveError> {
    let mut jacobi: Vec<Vec<f64>> = vec![];

    for i in 0..search_expres.len() {
        let mut row = vec![];
        for j in 0..x.len() {
            let mut added_vars = 0;
            for k in 0..x.len() {
                if j != k {
                    vars.push(x[k].clone());
                    added_vars += 1;
                }
            }
            row.push(calculate_derivative(&search_expres[i], x[j].name.clone(), x[j].value.clone(), Some(Value::Scalar(fx[i])), vars)?.get_scalar().unwrap());
            for _ in 0..added_vars {
                vars.remove(vars.len()-1);
            }
        }
        jacobi.push(row);
    } 

    let mut augmented_matrix = vec![];

    for i in 0..jacobi.len() {
        augmented_matrix.push(jacobi[i].clone());
        augmented_matrix[i].push(-1. * fx[i]);
    }

    let x_new_minus_x = gauss_algorithm(augmented_matrix)?;

    let mut x_new = vec![];

    for i in 0..x.len() {
        x_new.push(Variable::new(x[i].name.clone(), Value::Scalar(x_new_minus_x.get_vector().unwrap()[i] + x[i].value.get_scalar().unwrap())));
    }

    return Ok(x_new);
}

fn newton(search_expres: &Vec<Binary>, check_expres: &Vec<Binary> , x: &Vec<Variable>, vars: &mut Vec<Variable>) -> Result<(bool, Vec<Variable>), SolveError> {
    let mut fx = vec![];
    for i in x {
        vars.push(i.clone());
    }
    for i in search_expres {
        fx.push(eval(i, vars)?.get_scalar().unwrap());
    }
    for _ in x {
        vars.remove(vars.len()-1);
    }

    if -10f64.powf(-PREC) < abs(Value::Vector(fx.clone()))?.get_scalar().unwrap() && abs(Value::Vector(fx.clone()))?.get_scalar().unwrap() < 10f64.powf(-PREC) {
        let mut check_results = vec![];
        for i in x {
            vars.push(i.clone());
        }
        for i in check_expres {
            check_results.push(eval(i, vars)?.get_scalar().unwrap());
        }
        for _ in x {
            vars.remove(vars.len()-1);
        }
        if check_results.is_empty() {
            return Ok((true, x.to_vec()));
        }
        if -10f64.powf(-PREC) < abs(Value::Vector(check_results.clone()))?.get_scalar().unwrap() && abs(Value::Vector(check_results))?.get_scalar().unwrap() < 10f64.powf(-PREC) {
            return Ok((true, x.to_vec()));
        } 
    }

    let new_x = jacobi_and_gauss(search_expres, x, vars, fx)?;

    Ok((false, new_x))
}

#[derive(Debug)]
pub struct RootFinder {
    search_expres: Vec<Binary>,
    check_expres: Vec<Binary>,
    vars: Vec<Variable>,
    search_vars_names: Vec<String>
}

impl RootFinder {
    pub fn new(expressions: Vec<Binary>, mut vars: Vec<Variable>) -> Result<RootFinder, SolveError> {
        if expressions.len() == 0 {
            return Err(SolveError::NothingToDo);
        }

        for i in &expressions {
            match i {
                Binary::Value(_) => return Err(SolveError::NothingToDo),
                Binary::Variable(_) => return Err(SolveError::NothingToDo),
                Binary::Operation(_) => {}
            }
        }

        let vars_in_expr = find_vars_in_expr(&expressions[0], vec![]);

        let mut search_vars_names = vec![];

        let mut var_names = vec![];

        for var in &vars {
            var_names.push(var.name.clone());
        }

        for var in vars_in_expr {
            if !var_names.contains(&var) {
                if !search_vars_names.contains(&var) {
                    search_vars_names.push(var);
                }
            }
        }

        if search_vars_names.len() > expressions.len() {
            return Err(SolveError::UnderdeterminedSystem);
        }

        for i in &search_vars_names {
            vars.push(Variable::new(i.to_string(), Value::Scalar(2.5690823)));
        }

        let initial_res = eval(&expressions[0], &vars)?;

        for _ in &search_vars_names {
            vars.remove(vars.len()-1);
        }

        match initial_res {
            Value::Scalar(_) => {},
            Value::Vector(_) => return Err(SolveError::VectorInEq),
            Value::Matrix(_) => return Err(SolveError::MatrixInEq)
        }

        return Ok(RootFinder { search_expres: expressions[0..search_vars_names.len()].to_vec(), check_expres: vec![], vars, search_vars_names });
    }

    pub fn find_roots(&self) -> Result<Vec<Value>, SolveError> {
        let mut local_vars = self.vars.clone();
        let mut results = vec![];
        for j in -1000..1000 {
            let mut x = vec![];
            for k in &self.search_vars_names {
                x.push(Variable::new(k.to_string(), Value::Scalar(j as f64)));
            }

            'solve_loop: for _ in 0..1000 {
                let newton_result = newton(&self.search_expres, &self.check_expres, &x, &mut local_vars)?;

                if newton_result.0 {
                    let mut result_vec = vec![];
                    for i in newton_result.1 {
                        result_vec.push(i.value.get_scalar().unwrap());
                    }
                    if result_vec.len() == 1 {
                        results.push(Value::Scalar(result_vec[0].clone()));
                    } else {
                        results.push(Value::Vector(result_vec));
                    }
                    break;
                } else {
                    x = newton_result.1;
                }

                for i in &x {
                    if i.value.get_scalar().unwrap().is_nan() || i.value.get_scalar().unwrap().is_infinite() {
                        break 'solve_loop;
                    }
                }
            }
        }

        let cleaned_results = clean_results(results);

        return Ok(cleaned_results);
    }
}
