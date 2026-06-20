use crate::{MathLibError, quick_eval, value};

#[test]
fn simple_multiplication() -> Result<(), MathLibError> {
    let res = quick_eval!("3*3")?.to_vec();
    
    assert_eq!(res[0], value!(9));

    Ok(())
}

#[test]
fn factorial1() -> Result<(), MathLibError> {
    let res = quick_eval!("fact(3)")?.to_vec();

    assert_eq!(res[0], value!(6));

    Ok(())
}

#[test]
fn factorial2() -> Result<(), MathLibError> {
    let res = quick_eval!("fact(3.5)")?.round(3).to_vec();

    assert_eq!(res[0], value!(11.632));

    Ok(())
}

#[test]
fn tetration1() -> Result<(), MathLibError> {
    let res = quick_eval!("2^^4")?.to_vec();

    assert_eq!(res[0], value!(65536));

    Ok(())
}
//
// #[test]
// fn sum1() -> Result<(), MathLibError> {
//     let res = quick_eval!("S((-1)^k/(2k+1), k, 0, inf)*4")?.round(3).to_vec();
//
//     assert_eq!(res[0], value!(3.142));
//
//     Ok(())
// }
