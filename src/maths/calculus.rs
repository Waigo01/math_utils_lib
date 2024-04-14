use crate::{errors::EvalError, eval, parser::{Binary, Operation, SimpleOpType}, Value, Variable, PREC};

use super::{add, mult};

pub fn calculate_integral(expr: &Binary, in_terms_of: String, lower_bound: Value, upper_bound: Value, vars: &Vec<Variable>) -> Result<Value, EvalError> {
    let mut mut_vars = vars.to_owned();
    for i in 0..mut_vars.len() {
        if mut_vars[i].name == in_terms_of {
            mut_vars.remove(i);
            break;
        }
    }
    match (lower_bound, upper_bound) {
        (Value::Scalar(mut lb), Value::Scalar(mut ub)) => {
            if lb == ub {
                return Ok(Value::Scalar(0.))
            }
            if ub < lb {
                let temp = ub;
                ub = lb;
                lb = temp;
            }
            let dx = (ub-lb)/10f64.powf(PREC-2.);
            let mut sum = Value::Scalar(0.);
            let mut b = lb;
            while b < ub {
                mut_vars.push(Variable::new(in_terms_of.clone(), Value::Scalar(b)));
                sum = add(sum, eval(expr, &mut_vars)?)?;
                mut_vars.remove(mut_vars.len()-1);
                b += dx;
            }
            return Ok(mult(sum, Value::Scalar(dx))?);
        }
        _ => {return Err(EvalError::MathError("Only scalar bounds are allowed!".to_string()))}
    }
}

pub fn calculate_derivative(expr: &Binary, in_terms_of: String, at: Value, mut fx: Option<Value>, vars: &Vec<Variable>) -> Result<Value, EvalError> {
    let mut mut_vars = vars.to_owned();
    for i in 0..mut_vars.len() {
        if mut_vars[i].name == in_terms_of {
            mut_vars.remove(i);
            break;
        }
    }
    match at {
        Value::Scalar(s) => {
            if fx.is_none() {
                mut_vars.push(Variable::new(in_terms_of.clone(), at));
                fx = Some(eval(expr, &mut_vars)?);
                mut_vars.remove(mut_vars.len()-1);
            } 
            mut_vars.push(Variable::new(in_terms_of, Value::Scalar(s+10f64.powf(-(PREC)))));
            let fxh = eval(expr, &mut_vars)?;
            let h = Binary::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Div,
                left: Binary::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: Binary::Value(fxh),
                    right: Binary::Value(fx.unwrap())
                }),
                right: Binary::Value(Value::Scalar(10f64.powf(-(PREC))))
            });
            return eval(&h, &mut_vars);
        },
        Value::Vector(v) => {
            if fx.is_none() {
                mut_vars.push(Variable::new(in_terms_of.clone(), Value::Vector(v.clone())));
                fx = Some(eval(&expr, &mut_vars)?);
                mut_vars.remove(mut_vars.len()-1);
            } 
            let mut h_vec = vec![];
            for i in v {
                h_vec.push(i+10f64.powf(-(PREC)));
            }
            mut_vars.push(Variable::new(in_terms_of.clone(), Value::Vector(h_vec)));
            let fxh = eval(&expr, &mut_vars)?;
            let h = Binary::from_operation(Operation::SimpleOperation {
                op_type: SimpleOpType::Div,
                left: Binary::from_operation(Operation::SimpleOperation {
                    op_type: SimpleOpType::Sub,
                    left: Binary::from_value(fxh),
                    right: Binary::from_value(fx.unwrap())
                }),
                right: Binary::from_value(Value::Scalar(10f64.powf(-(PREC))))
            });
            return Ok(eval(&h, vars)?);
        } 
        _ => {return Err(EvalError::MathError("Only scalar and vector positions are allowed!".to_string()))}
    }
}
