use crate::{Context, MathLibError, Values, Variable, quick_eval, value};

#[test]
fn mutable_context_variable1() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("x = 10", &mut c)?;

    assert_eq!(c.get_var("x"), Some(Variable::new("x", vec![value!(10)])));

    let res = quick_eval!("x", &mut c)?.to_vec();

    assert_eq!(res[0], value!(10));

    Ok(())
}

#[test]
fn mutable_context_variable2() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("x = eq(x^2=9, x)", &mut c)?;

    let res = quick_eval!("x", &mut c)?.round(3).to_vec();

    assert_eq!(res, vec![value!(-3), value!(3)]);

    Ok(())
}

#[test]
fn mutable_context_function() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("f(x) = x^2", &mut c)?;

    let res = quick_eval!("f(5)", &mut c)?.to_vec();

    assert_eq!(res[0], value!(25));

    Ok(())
}

#[test]
fn mutable_context_multi_variable() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("{x, y, z} = {1, 2, 3}", &mut c)?;

    let res = quick_eval!("x", &mut c)?.to_vec();
    assert_eq!(res[0], value!(1));

    let res = quick_eval!("y", &mut c)?.to_vec();
    assert_eq!(res[0], value!(2));

    let res = quick_eval!("z", &mut c)?.to_vec();
    assert_eq!(res[0], value!(3));

    Ok(())
}

#[test]
fn mutable_context_variable_list() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("x = {1, 2, 3}", &mut c)?;

    let res = quick_eval!("x", &mut c)?.to_vec();
    assert_eq!(res, vec![value!(1), value!(2), value!(3)]);

    Ok(())
}

#[test]
fn function_side_effects() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("f(x) = y=x", &mut c)?;

    quick_eval!("f(5)", &mut c)?;

    assert_eq!(c.get_var("y".to_string()).unwrap().values.to_vec()[0], value!(5));

    Ok(())
}

#[test]
fn function_side_multi_effects() -> Result<(), MathLibError> {
    let mut c = Context::default();

    quick_eval!("f(x) = z=y=x*2", &mut c)?;

    quick_eval!("f(5)", &mut c)?;

    assert_eq!(c.get_var("y".to_string()), Some(Variable { name: "y".to_string(), values: Values::from_vec(vec![value!(10)]) }));
    assert_eq!(c.get_var("z".to_string()), Some(Variable { name: "z".to_string(), values: Values::from_vec(vec![value!(10)]) }));

    Ok(())
}
