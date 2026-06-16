use crate::{Context, Function, MathLibError, errors::{EvalError, QuickEvalError}, parse, quick_eval, value};

#[test]
fn solve_parabolic() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(x^2=9, x)")?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(-3), value!(3)]);

    Ok(())
}

#[test]
fn solve_polynomial() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(3x^2+2x-1=0, x)")?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(-1), value!(1./3.).round(3)]);

    Ok(())
}

#[test]
fn solve_linear_system1() -> Result<(), MathLibError> {
    let equation = "eq(2x+5y+2z=-38, 3x-2y+4z=17, -6x+y-7z=-12, x, y, z)";

    let res = quick_eval!(equation)?.round(3).to_vec();

    assert_eq!(res, vec![value!(3, -8, -2)]);

    Ok(())
}

#[test]
fn solve_linear_system2() -> Result<(), MathLibError> {
    let equation = "eq(3x-9z=33, 7x-4y-z=-15, 4x+6y+5z=-6, x, y, z)";

    let res = quick_eval!(equation)?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1, 3, -4)]);

    Ok(())
}

#[test]
fn solve_custom_function() -> Result<(), MathLibError> {
    let function = parse("4x^2-9")?;
    let function_var = Function::new("f", function, vec!["x"]);

    let res = quick_eval!("eq(f(x)=0, x)", &mut Context::from_funs(vec![function_var]))?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1.5), value!(1.5)]);

    Ok(())
}

#[test]
fn solve_extremum() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(D(3x^2+2x-1, x, k)=0, k)")?.round(3).to_vec();

    assert_eq!(res, vec![value!(-1./3.).round(3)]);

    Ok(())
}

#[test]
fn crazy_solve() -> Result<(), MathLibError> {
    let equation = "eq(((25x^3-96x^2+512x+384)/(x^4+2x^3+90x^2-128x+1664)^(1.5))/(-sqrt(1-((32-x+x^2)/(((x-1)^2+25)(x^2+64)))^2))=0, x)";

    let res = quick_eval!(equation)?.round(3).to_vec();

    assert_eq!(res.contains(&value!(-0.656)), true);

    Ok(())
}

#[test]
fn solve_vector() -> Result<(), MathLibError> {
    let res = quick_eval!("eq(x*[3, 4, 5]=[6, 8, 10], x)", &mut Context::empty());

    assert_eq!(res.unwrap_err(), QuickEvalError::EvalError(EvalError::VectorInEq));

    Ok(())
}

#[test]
fn solve_overdefined_system1() -> Result<(), MathLibError> {
    let equation = "eq(400-100x=600-100x, -600-100x=-400-100x, 1000-100x=0+100x, x)";

    let res = quick_eval!(equation)?.to_vec();

    assert_eq!(res, vec![]);

    Ok(())
}

#[test]
fn solve_overdefined_system2() -> Result<(), MathLibError> {
    let equation = "eq(400-100g=600-100k, -600-100g=-400-100k, 1000-100g=100k, g, k)";

    let res = quick_eval!(equation)?.round(3).to_vec();
    
    assert_eq!(res, vec![value!(4, 6)]);

    Ok(())
}

#[test]
fn solve_nonlinear_system1() -> Result<(), MathLibError> {
    let equation = "eq(y=x^2+6x-8, y=4x+7, x, y)";

    let res = quick_eval!(equation)?.round(3).to_vec();

    assert_eq!(res, vec![value!(-5, -13), value!(3, 19)]);

    Ok(())
}

#[test]
fn solve_nonlinear_system2() -> Result<(), MathLibError> {
    let equation = "eq(y=1-3x, x^2/4+y^2=1, x, y)";

    let res = quick_eval!(equation)?.round(3).to_vec();

    assert_eq!(res, vec![value!(24./37., -35./37.).round(3), value!(0, 1)]);

    Ok(())
}
