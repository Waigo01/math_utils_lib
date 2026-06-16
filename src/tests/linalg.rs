use crate::{Context, MathLibError, Value, Variable, quick_eval, value};

#[test]
fn matrix_mult() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(6, 1, 13, 8; 4, 3, 9, 10; 4, 0, 14, 8));
    let b = Variable::new("B", value!(3, 0; 1, 1; 0, 6; 2.5, 2));
    let res = quick_eval!("A*B", &mut Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(39, 95; 40, 77; 32, 100));

    Ok(())
}

#[test]
fn matrix_inverse1() -> Result<(), MathLibError> {
    let a = Variable::new("A", Value::Matrix(vec![vec![2.]]));

    let res = quick_eval!("inv(A)", &mut Context::with_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], Value::Matrix(vec![vec![0.5]]));

    Ok(())
}

#[test]
fn matrix_vector_mult1() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3, 5, 8));
    let b = Variable::new("B", value!(2, 0, 0; 0, 2, 0; 0, 0, 1));
    let res = quick_eval!("B*A", &mut Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(6, 10, 8));

    Ok(())
}

#[test]
fn large_matrix_mult() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(3, 5, 7; 4, 8, 2; 1, 9, 2));
    let b = Variable::new("B", value!(7, 9, 10; 1, 55, 8; 22, 9, 2));
    let res = quick_eval!("A*B", &mut Context::from_vars(vec![a, b]))?.to_vec();

    assert_eq!(res[0], value!(180, 365, 84; 80, 494, 108; 60, 522, 86));

    Ok(())
}

#[test]
fn matrix_vector_mult2() -> Result<(), MathLibError> {
    let res = quick_eval!("[[1, 0], [0, 6], [0, 0]]*[3, 4, 5]")?.to_vec();

    assert_eq!(res[0], value!(3, 24));

    Ok(())
}

#[test]
fn medium_eval19() -> Result<(), MathLibError> {
    let res = quick_eval!("[0, 0.5, 0]#[-0.8, 0, 0.6]")?.to_vec();

    assert_eq!(res[0], value!(0.3, 0, 0.4));

    Ok(())
}

#[test]
fn matrix_det() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(1, 2, 3; 4, 5, 6; 7, 8, 9));
    let res = quick_eval!("det(A)", &mut Context::with_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(0));

    Ok(())
}

#[test]
fn large_matrix_det() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(0, 6, -2, -1, 5; 0, 0, 0, -9, -7; 0, 15, 35, 0, 0; 0, -1, -11, -2, 1; -2, -2, 3, 0, -2));
    let res = quick_eval!("det(A)", &mut Context::with_vars(vec![a]))?.to_vec();

    assert_eq!(res[0], value!(2480));

    Ok(())
}

#[test]
fn matrix_inverse2() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(-1, 3./2.; 1, -1));

    let res = quick_eval!("inv(A)", &mut Context::with_vars(vec![a]))?.round(3).to_vec();

    assert_eq!(res[0], value!(2, 3; 2, 2));

    Ok(())
}

#[test]
fn matrix_power1() -> Result<(), MathLibError> {
    let m = Variable::new("M", value!(0.7, 0.1, 0.3; 0.1, 0.5, 0.1; 0.2, 0.4, 0.6));

    let res = quick_eval!("M^20", &mut Context::from_vars(vec![m]))?.round(3).to_vec();

    assert_eq!(res[0], value!(0.444, 0.444, 0.444; 0.167, 0.167, 0.167; 0.389, 0.389, 0.389));

    Ok(())
}

#[test]
fn matrix_power2() -> Result<(), MathLibError> {
    let m = Variable::new("M", value!(0., 2., 0.; 0.5, 0., 0.; 0., 0.8, 0.));

    let res = quick_eval!("M^2", &mut Context::from_vars(vec![m]))?.round(3).to_vec();

    assert_eq!(res[0], value!(1, 0, 0; 0, 1, 0; 0.4, 0, 0));

    Ok(())
}

#[test]
fn positional_get() -> Result<(), MathLibError> {
    let x = Variable::new("x", value!(3));
    let a = Variable::new("A", value!(3, 2, 1));
    let b = Variable::new("B", value!(2, 3, 4; 5, 1, 7; 2, 3, 6));
    let res = quick_eval!("(x*B*A)@1", &mut Context::from_vars(vec![a, x, b]))?.to_vec();

    assert_eq!(res[0], value!(72));

    Ok(())
}

#[test]
fn matrix_inverse3() -> Result<(), MathLibError> {
    let a = Variable::new("A", value!(-3, -1, 2, -3; -3, 1, 2, -2; -2, 3, 0, 1; 1, -2, -3, 1));
    let res = quick_eval!("inv(A)", &mut Context::with_vars(vec![a]))?.round(3).to_vec();

    assert_eq!(res[0], value!(-11./7., 2, -1, 2./7.; -15./7., 3, -1, 4./7.; 2, -3, 1, -1; 23./7., -5, 2, -8./7.).round(3));

    Ok(())
}
