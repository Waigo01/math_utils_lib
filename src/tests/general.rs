use crate::{MathLibError, quick_eval, value};

#[test]
fn simple_multiplication() -> Result<(), MathLibError> {
    let res = quick_eval!("3*3")?.to_vec();
    
    assert_eq!(res[0], value!(9));

    Ok(())
}
