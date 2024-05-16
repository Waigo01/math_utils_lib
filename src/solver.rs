use crate::{errors::SolveError, parser::{Binary, Operation, SimpleOpType}, roots::RootFinder, Value, Variable};

/// used to solve an equation or a system of equations.
///
/// This function takes a Vec of a Tuple of Binaries, which are the parsed left and right sides of
/// the equation(s). Multiple Tuples signify a system of equations. It also takes the user defined global variables.
///
/// e and pi need to be provided as variables.
///
/// If you are searching for an easy way of directly solving equations, have a look at
/// [quick_solve()](fn@crate::quick_solve())
pub fn solve(equations: Vec<(Binary, Binary)>, vars: Vec<Variable>) -> Result<Vec<Value>, SolveError> {
    let mut final_expressions = vec![];

    for i in &equations {
        let root_b = Binary::from_operation(Operation::SimpleOperation {
            op_type: SimpleOpType::Sub,
            left: i.0.clone(),
            right: i.1.clone()
        });

        final_expressions.push(root_b);
    }
    let root_finder = RootFinder::new(final_expressions, vars)?;
    return root_finder.find_roots();
}
