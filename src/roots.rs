use crate::{basetypes::{AdvancedOperation, Operation, Value, Variable, AST}, errors::{NewtonError, SolveError}, maths::{abs, calculus::calculate_derivative}, parser::eval, Store, PREC};

fn clean_results(res: Vec<Value>) -> Vec<Value> {
    if res.len() == 0 {
        return vec![];
    }
    let mut new_res: Vec<Value> = vec![];
    for i in &res {
        let mut found = false;
        for j in &new_res {
            if i.round(PREC-2) == j.round(PREC-2) {
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

fn find_vars_in_expr(b: &AST, mut ov: Vec<String>) -> Vec<String> {
    match b {
        AST::Variable(v) => {
            ov.push(v.to_string());
            return ov.to_owned();
        },
        AST::Function { inputs, .. } => {
            let mut found_vars = vec![];
            for i in &**inputs {
                found_vars.append(&mut find_vars_in_expr(i, ov.clone()));
            }
            return found_vars;
        }
        AST::Scalar(_) => {
            return ov.to_owned();
        },
        AST::Vector(v) => {
            let mut found_vars = vec![];
            for i in &**v {
                found_vars.append(&mut find_vars_in_expr(i, ov.clone()));
            }
            return found_vars;
        },
        AST::Matrix(m) => {
            let mut found_vars = vec![];
            for i in &**m {
                for j in i {
                    found_vars.append(&mut find_vars_in_expr(j, ov.clone()));
                }
            }
            return found_vars;
        },
        AST::Operation(o) => {
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

fn gauss_algorithm(v: &mut Vec<Vec<f64>>) -> Result<Value, NewtonError> {
    if v.len()+1 != v[0].len() {
        return Err(NewtonError::UnderdeterminedSystem);
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
                return Err(NewtonError::InfiniteSolutions);
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
                return Err(NewtonError::InfiniteSolutions);
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

fn jacobi_and_gauss(search_expres: &[AST], x: &[Variable], state: &mut Store, fx: &Vec<f64>) -> Result<Vec<Variable>, NewtonError> {
    let mut jacobi: Vec<Vec<f64>> = vec![];

    let mut vars: Vec<&Variable> = state.vars.iter().collect();

    for i in 0..search_expres.len() {
        let mut row = vec![];
        for j in 0..x.len() {
            let mut added_vars = 0;
            for k in 0..x.len() {
                if j != k {
                    vars.push(&x[k]);
                    added_vars += 1;
                }
            }
            let derivative = calculate_derivative(&search_expres[i], &x[j].name, &x[j].value, Some(Value::Scalar(fx[i])), &Store::new(&vars.iter().map(|v| v.to_owned().to_owned()).collect::<Vec<Variable>>(), &state.funs))?.get_scalar().unwrap();
            row.push(derivative);
            for _ in 0..added_vars {
                vars.remove(state.vars.len()-1);
            }
        }
        jacobi.push(row);
    } 

    let mut augmented_matrix = vec![];

    for i in 0..jacobi.len() {
        augmented_matrix.push(jacobi[i].clone());
        augmented_matrix[i].push(-1. * fx[i]);
    }

    let x_new_minus_x = gauss_algorithm(&mut augmented_matrix)?;

    let mut x_new = vec![];

    for i in 0..x.len() {
        x_new.push(Variable::new(&x[i].name, Value::Scalar(x_new_minus_x.get_vector().unwrap()[i] + x[i].value.get_scalar().unwrap())));
    }

    return Ok(x_new);
}

enum NewtonReturn {
    NextX(Vec<Variable>),
    FinishedX(Vec<Variable>) 
}

fn newton(search_expres: &Vec<AST>, check_expres: &Vec<AST> , x: &Vec<Variable>, state: &mut Store) -> Result<NewtonReturn, NewtonError> {
    let mut fx = vec![];
    let mut vars: Vec<&Variable> = state.vars.iter().collect();
    for i in x {
        vars.push(i);
    }
    for i in search_expres {
        fx.push(eval(i, &Store::new(&vars.iter().map(|v| v.to_owned().to_owned()).collect::<Vec<Variable>>(), &state.funs))?.get_scalar().unwrap());
    }
    for _ in x {
        vars.remove(state.vars.len()-1);
    }

    if -10f64.powi(-PREC) < abs(Value::Vector(fx.clone()))?.get_scalar().unwrap() && abs(Value::Vector(fx.clone()))?.get_scalar().unwrap() < 10f64.powi(-PREC) {
        let mut check_results = vec![]; 
        for i in x {
            vars.push(i);
        }
        for i in check_expres {
            check_results.push(eval(i, &Store::new(&vars.iter().map(|v| v.to_owned().to_owned()).collect::<Vec<Variable>>(), &state.funs))?.get_scalar().unwrap());
        }
        for _ in x {
            vars.remove(vars.len()-1);
        }
        if check_results.is_empty() {
            return Ok(NewtonReturn::FinishedX(x.to_vec()));
        }
        if -10f64.powi(-PREC) < abs(Value::Vector(check_results.clone()))?.get_scalar().unwrap() && abs(Value::Vector(check_results))?.get_scalar().unwrap() < 10f64.powi(-PREC) {
            return Ok(NewtonReturn::FinishedX(x.to_vec()));
        } else {
            return Err(NewtonError::ExpressionCheckFailed);
        } 
    }

    let new_x = jacobi_and_gauss(search_expres, x, state, &fx)?;

    for i in &new_x {
        if i.value.is_inf_or_nan() {
            return Err(NewtonError::NaNOrInf);
        }
    }

    return Ok(NewtonReturn::NextX(new_x));
}

fn generate_combinations(arr: Vec<usize>, len: usize, prev_arr: Vec<usize>) -> Vec<Vec<usize>> {
    if prev_arr.len() == len {
        return vec![prev_arr];
    }
    let mut combs = vec![];
    for (i, val) in arr.iter().enumerate() {
        let mut prev_arr_extended = prev_arr.clone();
        prev_arr_extended.push(*val);
        combs.append(&mut generate_combinations(arr[i+1..].to_vec(), len, prev_arr_extended));
    }
    return combs;
}

/// defines a root finder to find the roots of an expression/multiple expressions (system of equations).
#[derive(Debug)]
pub struct RootFinder {
    expressions: Vec<AST>,
    combinations: Vec<Vec<usize>>,
    state: Store,
    search_vars_names: Vec<String>
}

impl RootFinder {
    /// creates a new [RootFinder](struct@crate::roots::RootFinder) using a vec of expressions which represents
    /// the functions that you want the roots to be found of. Multiple expressions act as a system
    /// of equations. Additionally you have to pass the global variables.
    ///
    /// If you want a simpler way of solving equations and systems of equations, have a look at
    /// [solve()](fn@crate::solver::solve) and [quick_solve()](fn@crate::quick_solve).
    pub fn new(expressions: Vec<AST>, state: Store) -> Result<RootFinder, SolveError> {
        if expressions.len() == 0 {
            return Err(SolveError::NothingToDo);
        }

        for i in &expressions {
            match i {
                AST::Vector(_) => return Err(SolveError::NothingToDo),
                AST::Scalar(_) => return Err(SolveError::NothingToDo),
                AST::Matrix(_) => return Err(SolveError::NothingToDo),
                AST::Variable(_) => return Err(SolveError::NothingToDo),
                AST::Function {..} => return Err(SolveError::NothingToDo),
                AST::Operation(_) => {}
            }
        }

        let mut search_vars_names = vec![];

        for i in &expressions {
            let vars_in_expr = find_vars_in_expr(i, vec![]);

            let mut var_names = vec![];

            for var in &state.vars {
                var_names.push(var.name.clone());
            }

            for (i, var) in vars_in_expr.iter().enumerate() {
                if !var_names.contains(&var) {
                    if !search_vars_names.contains(var) {
                        if i > search_vars_names.len() {
                            search_vars_names.push(var.to_string());
                        } else {
                            search_vars_names.insert(i, var.to_string());
                        }
                    }
                }
            }
        }

        let mut vars = state.vars.to_vec();

        if search_vars_names.len() > expressions.len() {
            return Err(NewtonError::UnderdeterminedSystem.into());
        }

        for i in &search_vars_names {
            vars.push(Variable::new(i.to_string(), Value::Scalar(2.5690823)));
        }

        let initial_res = eval(&expressions[0], &Store::new(&vars, &state.funs))?;

        for _ in &search_vars_names {
            vars.remove(state.vars.len()-1);
        }

        match initial_res {
            Value::Scalar(_) => {},
            Value::Vector(_) => return Err(SolveError::VectorInEq),
            Value::Matrix(_) => return Err(SolveError::MatrixInEq)
        }

        let combs;

        if search_vars_names.len() < expressions.len() {
            combs = generate_combinations((0..expressions.len()).collect::<Vec<usize>>(), search_vars_names.len(), vec![]);
        } else {
            combs = vec![(0..expressions.len()).collect::<Vec<usize>>()];
        }

        return Ok(RootFinder { expressions, combinations: combs, state, search_vars_names });
    }
    /// starts the root finding process. It will always search for roots in terms of variables that
    /// have not yet been defined in the global variables passed in
    /// [new()](fn@crate::roots::RootFinder::new).
    /// 
    /// In the case of a system of equations results will be represented as a vector with the order
    /// being that of the variables in the expressions.
    pub fn find_roots(&self) -> Result<Vec<Value>, SolveError> {
        for i in &self.combinations {
            let mut search_expres = vec![];
            let mut check_expres = self.expressions.clone();
            let mut removed = 0;
            for j in i {
                search_expres.push(check_expres.remove(*j-removed));
                removed += 1;
            } 
            let mut local_state = self.state.clone();
            let mut results = vec![];
            'solve_loop_0: for j in -1000..1000 {
                let mut x = vec![];
                for k in &self.search_vars_names {
                    x.push(Variable::new(k, Value::Scalar(j as f64)));
                }

                'solve_loop_1: for _ in 0..1000 {
                    let newton_result = newton(&search_expres, &check_expres, &x, &mut local_state);

                    match newton_result {
                        Ok(o) => {
                            match o {
                                NewtonReturn::NextX(next_x) => x = next_x,
                                NewtonReturn::FinishedX(fin_x) => {
                                    let mut result_vec = vec![];
                                    for i in fin_x {
                                        result_vec.push(i.value.get_scalar().unwrap());
                                    }
                                    if result_vec.len() == 1 {
                                        results.push(Value::Scalar(result_vec[0].clone()));
                                    } else {
                                        results.push(Value::Vector(result_vec));
                                    }
                                    break 'solve_loop_1;
                                },
                            }
                        },
                        Err(e) => {
                            match e {
                                NewtonError::InfiniteSolutions => break 'solve_loop_0,
                                NewtonError::NaNOrInf => break 'solve_loop_1,
                                NewtonError::ExpressionCheckFailed => break 'solve_loop_1,
                                _ => return Err(e.into())
                            }
                        }
                    }
                }
            }

            let cleaned_results = clean_results(results);

            if !cleaned_results.is_empty() {
                return Ok(cleaned_results);
            }
        }

        return Ok(vec![]);
    }
}
