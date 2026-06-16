use crate::{Complex, MathLibError, quick_eval, value};

#[test]
fn complex_mult() -> Result<(), MathLibError> {
    let res = quick_eval!("(3+2i)*(4+2i)"; Complex<f64>)?.to_vec();

    assert_eq!(res[0], value!(Complex::new(8., 14.)));

    Ok(())
}

#[test]
fn euler() -> Result<(), MathLibError> {
    let res = quick_eval!("e^(i*pi)"; Complex<f64>)?.round(3).to_vec();

    assert_eq!(res[0], value!(-1));

    Ok(())
}

#[test]
fn complex_sqrt() -> Result<(), MathLibError> {
    let res = quick_eval!("sqrt(3+2i)"; Complex<f64>)?.round(4).to_vec();

    assert_eq!(res[0], value!(Complex::new(1.8174, 0.5503)));

    Ok(())
}
