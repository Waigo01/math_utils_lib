use crate::{Context, Function, MathLibError, Variable, errors::{EvalError, QuickEvalError}, parse, quick_eval, value};

#[test]
fn root_function() -> Result<(), MathLibError> {
    let res = quick_eval!("root(8, 3)")?.to_vec();

    assert_eq!(res[0], value!(2));

    Ok(())
}

#[test]
fn custom_function() -> Result<(), MathLibError> {
    let function = parse("5x^2+2x+x")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval!("f(5)", &mut Context::from_funs(vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(140));

    Ok(())
}

#[test]
fn custom_function_vector_input() -> Result<(), MathLibError> {
    let function = parse("x-A")?;
    let function_var = Function::new("f", function, vec!["x"]);
    let a = Variable::new("A", value!(3., 4., 5.));

    let res = quick_eval!("f([3, 4, 5])", &mut Context::new(vec![a], vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(0, 0, 0));

    Ok(())
}

#[test]
fn custom_function_recursion() -> Result<(), MathLibError> {
    let function = parse("3*f(x)")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval!("f(5)", &mut Context::new(vec![], vec![function_var]));

    assert_eq!(res.err().unwrap(), QuickEvalError::EvalError(EvalError::RecursiveFunction));

    Ok(())
}

#[test]
fn custom_function_of_function() -> Result<(), MathLibError> {
    let function = parse("3*x")?;
    let function_var = Function::new("f", function, vec!["x"]);
    
    let res = quick_eval!("f(f(6))", &mut Context::new(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0], value!(54));

    Ok(())
}

#[test]
fn custom_function_list_input_output() -> Result<(), MathLibError> {
    let function = parse("+-sqrt(x)+y")?;
    let function_var = Function::new("f", function, vec!["x", "y"]);

    let res = quick_eval!("f(+-sqrt(16), +-sqrt(9))", &mut Context::with(vec![], vec![function_var]))?.to_vec();

    assert_eq!(res[0..4].to_vec(), vec![value!(5), value!(1), value!(-1), value!(-5)]);

    Ok(())
}
