use crate::{MathLibError, quick_eval, value};

#[test]
fn list_in_matrix() -> Result<(), MathLibError> {
    let res = quick_eval!("[[+-sqrt(9), 0, 0], [0, 1, 0], [0, 0, 1]]")?.to_vec();

    assert_eq!(res, vec![value!(3, 0, 0; 0, 1, 0; 0, 0, 1), value!(-3, 0, 0; 0, 1, 0; 0, 0, 1)]);

    Ok(())
}

#[test]
fn cartesian_vector_from_lists() -> Result<(), MathLibError> {
    let res = quick_eval!("[+-sqrt(9), +-sqrt(9), 0]")?.to_vec();

    assert_eq!(res, vec![value!(3, 3, 0), value!(3, -3, 0), value!(-3, 3, 0), value!(-3, -3, 0)]);

    Ok(())
}

#[test]
fn list_eval() -> Result<(), MathLibError> {
    let res = quick_eval!("2*(+-sqrt(9))")?.to_vec();

    assert_eq!(res, vec![value!(6), value!(-6)]);

    Ok(())
}

#[test]
fn plus_minus_root() -> Result<(), MathLibError> {
    let res = quick_eval!("+-root(9, 2)")?.to_vec();

    assert_eq!(res, vec![value!(3), value!(-3)]);

    Ok(())
}

#[test]
fn list_in_vector() -> Result<(), MathLibError> {
    let res = quick_eval!("[{3, 5}, 0, 0]")?.to_vec();

    assert_eq!(res, vec![value!(3, 0, 0), value!(5, 0, 0)]);

    Ok(())
}

#[test]
fn list_mult() -> Result<(), MathLibError> {
    let res = quick_eval!("{3, 2}*{7, 3, 2}")?.to_vec();

    assert_eq!(res, vec![value!(21), value!(9), value!(6), value!(14), value!(6), value!(4)]);

    Ok(())
}
