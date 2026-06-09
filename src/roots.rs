use crate::{Context, PREC, basetypes::{AST, Value, Variable}, errors::EvalError, evaluator::eval, maths::{calculus::calculate_derivative_newton, num_trait::Number}};

fn clean_results<N: Number>(res: &[Value<N>]) -> Vec<Value<N>> {
    if res.len() == 0 {
        return vec![];
    }
    let mut new_res: Vec<Value<N>> = vec![];
    for i in res {
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

fn gauss_algorithm<N: Number>(v: &mut Vec<Vec<N>>) -> Result<Value<N>, EvalError> {
    if v.len()+1 != v[0].len() {
        return Err(EvalError::UnderdeterminedSystem);
    }

    for i in 0..v.len() - 1 {
        for j in (i+1)..v.len() {
            let divisor = v[i][i]/v[j][i];
            let mut zero_line = true;
            for k in i..v[j].len() {
                v[j][k] = v[j][k] - v[i][k]/divisor; 
                if v[j][k] != N::from(0.) {
                    zero_line = false;
                }
            }
            if zero_line {
                return Err(EvalError::InfiniteSolutions);
            }
        }
    } 

    v.reverse();

    v.iter_mut().for_each(|x| x.reverse());

    let aug_col: Vec<N> = v.iter().map(|x| x[0]).collect();

    for i in 0..v.len() {
        v[i].remove(0);
        v[i].push(aug_col[i]);
    }

    for i in 0..v.len() - 1 {
        for j in (i+1)..v.len() {
            let divisor = v[i][i]/v[j][i];
            let mut zero_line = true;
            for k in i..v[j].len() {
                v[j][k] = v[j][k] - v[i][k]/divisor;
                if v[j][k] != N::from(0.) {
                    zero_line = false;
                }
            }
            if zero_line {
                return Err(EvalError::InfiniteSolutions);
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

fn jacobi_and_gauss<N: Number>(search_expres: &[AST], x: &[Variable<N>], context: &mut Context<N>, fx: &Vec<N>) -> Result<Vec<Variable<N>>, EvalError> {
    let mut jacobi: Vec<Vec<N>> = vec![];

    let mut vars: Vec<&Variable<N>> = context.vars.iter().collect();

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
            let derivative = calculate_derivative_newton(&search_expres[i], &x[j].name, x[j].values.get(0).unwrap(), Some(Value::Scalar(fx[i])), &mut Context::new(&vars.iter().map(|v| v.to_owned().to_owned()).collect::<Vec<Variable<N>>>(), &context.funs))?.get_scalar().unwrap();
            row.push(derivative);
            for _ in 0..added_vars {
                vars.remove(vars.len()-1);
            }
        }
        jacobi.push(row);
    } 

    for i in 0..jacobi.len() {
        jacobi[i].push(N::from(-1.) * fx[i]);
    }

    let x_new_minus_x = gauss_algorithm(&mut jacobi)?;

    let mut x_new = vec![];

    for i in 0..x.len() {
        x_new.push(Variable::new(&x[i].name, vec![Value::Scalar(x_new_minus_x.get_vector().unwrap()[i] + x[i].values.get(0).unwrap().get_scalar().unwrap())]));
    }

    return Ok(x_new);
}

enum NewtonReturn<N: Number> {
    NextX(Vec<Variable<N>>),
    FinishedX(Vec<Variable<N>>) 
}

fn newton<N: Number>(search_expres: &Vec<AST>, check_expres: &Vec<AST> , x: &Vec<Variable<N>>, context: &mut Context<N>) -> Result<NewtonReturn<N>, EvalError> {
    let mut fx = vec![];
    for i in x {
        context.add_var(i);
    }
    for i in search_expres {
        fx.push(eval(i, context)?.get(0).unwrap().get_scalar().unwrap());
    }
    for i in x {
        context.remove_var(&i.name);
    }

    if N::from(-1.)*N::from(10f64).powi(-(PREC as i32)) < fx.iter().map(|f| f.powi(2)).sum::<N>().sqrt() && fx.iter().map(|f| f.powi(2)).sum::<N>().sqrt() < N::from(10f64).powi(-(PREC as i32)) {
        let mut check_results = vec![]; 
        for i in x {
            context.add_var(i);
        }
        for i in check_expres {
            check_results.push(eval(i, context)?.get(0).unwrap().get_scalar().unwrap());
        }
        for i in x {
            context.remove_var(&i.name);
        }
        if check_results.is_empty() {
            return Ok(NewtonReturn::FinishedX(x.to_vec()));
        }
        if N::from(-1.)*N::from(10f64).powi(-(PREC as i32)) < check_results.iter().map(|f| f.powi(2)).sum::<N>().sqrt() && check_results.iter().map(|f| f.powi(2)).sum::<N>().sqrt() < N::from(10f64).powi(-(PREC as i32)) {
            return Ok(NewtonReturn::FinishedX(x.to_vec()));
        } else {
            return Err(EvalError::ExpressionCheckFailed);
        } 
    }

    let new_x = jacobi_and_gauss(search_expres, x, context, &fx)?;

    for i in &new_x {
        if i.values.get(0).unwrap().is_inf_or_nan() {
            return Err(EvalError::NaNOrInf);
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
pub struct RootFinder<N: Number> {
    expressions: Vec<AST>,
    combinations: Vec<Vec<usize>>,
    context: Context<N>,
    search_vars_names: Vec<String>
}

impl<N: Number> RootFinder<N> {
    /// creates a new [RootFinder](struct@crate::roots::RootFinder) using a vec of expressions which represents
    /// the functions that you want the roots to be found of. Multiple expressions act as a system
    /// of equations. Additionally you have to pass the context and the variables in terms of which
    /// should be solved.
    ///
    /// This functionality has been implemented into the eval process using the
    /// [Equation](crate::basetypes::AdvancedOpType::Equation) operator.
    pub fn new(expressions: Vec<AST>, mut context: Context<N>, search_vars_names: Vec<String>) -> Result<RootFinder<N>, EvalError> {
        if expressions.len() == 0 {
            return Err(EvalError::NothingToDoEq);
        }

        for i in &expressions {
            match i {
                AST::Vector(_) => return Err(EvalError::NothingToDoEq),
                AST::Scalar(_) => return Err(EvalError::NothingToDoEq),
                AST::Matrix(_) => return Err(EvalError::NothingToDoEq),
                AST::List(_) => return Err(EvalError::NothingToDoEq),
                AST::Variable(_) => return Err(EvalError::NothingToDoEq),
                AST::Function {..} => return Err(EvalError::NothingToDoEq),
                AST::Operation(_) => {}
            }
        }

        for i in &search_vars_names {
            if context.vars.iter().map(|v| v.name.clone()).collect::<Vec<String>>().contains(&i) {
                return Err(EvalError::SearchVarsInVars);
            }
        }

        if search_vars_names.len() > expressions.len() {
            return Err(EvalError::UnderdeterminedSystem.into());
        }

        for i in &search_vars_names {
            context.add_var(&Variable::new(i, vec![Value::Scalar(N::from(8.21785))]));
        }

        let initial_res = eval(&expressions[0], &mut context)?;

        for i in &search_vars_names {
            context.remove_var(i);
        }

        match initial_res.get(0).unwrap() {
            Value::Scalar(_) => {},
            Value::Vector(_) => return Err(EvalError::VectorInEq),
            Value::Matrix(_) => return Err(EvalError::MatrixInEq)
        }

        let combs;

        if search_vars_names.len() < expressions.len() {
            combs = generate_combinations((0..expressions.len()).collect::<Vec<usize>>(), search_vars_names.len(), vec![]);
        } else {
            combs = vec![(0..expressions.len()).collect::<Vec<usize>>()];
        }

        return Ok(RootFinder { expressions, combinations: combs, context, search_vars_names });
    }
    /// starts the root finding process.
    /// 
    /// In the case of a system of equations results will be represented as a vector with the
    /// result order being that in which the search_vars_names have been passed to the
    /// [RootFinder::new] function.
    pub fn find_roots(&self) -> Result<Vec<Value<N>>, EvalError> {
        for i in &self.combinations {
            let mut search_expres = vec![];
            let mut check_expres = self.expressions.clone();
            let mut removed = 0;
            for j in i {
                search_expres.push(check_expres.remove(*j-removed));
                removed += 1;
            } 
            let mut local_context = self.context.clone();
            let mut results = vec![];
            'solve_loop_0: for j in -1000..1000 {
                let mut x = vec![];
                for k in &self.search_vars_names {
                    x.push(Variable::new(k, vec![Value::Scalar(N::from(j as f64))]));
                }

                'solve_loop_1: for _ in 0..1000 {
                    let newton_result = newton(&search_expres, &check_expres, &x, &mut local_context);

                    match newton_result {
                        Ok(o) => {
                            match o {
                                NewtonReturn::NextX(next_x) => {x = next_x},
                                NewtonReturn::FinishedX(fin_x) => {
                                    let mut result_vec = vec![];
                                    for i in fin_x {
                                        result_vec.push(i.values.get(0).unwrap().get_scalar().unwrap());
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
                                EvalError::InfiniteSolutions => break 'solve_loop_0,
                                EvalError::NaNOrInf => break 'solve_loop_1,
                                EvalError::ExpressionCheckFailed => break 'solve_loop_1,
                                _ => return Err(e.into())
                            }
                        }
                    }
                }
            }

            let cleaned_results = clean_results(&results);

            if !cleaned_results.is_empty() {
                return Ok(cleaned_results);
            }
        }

        return Ok(vec![]);
    }
}
