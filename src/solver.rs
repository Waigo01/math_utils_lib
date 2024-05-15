use crate::{errors::SolveError, parser::{Binary, Operation, SimpleOpType}, roots::RootFinder, Value, Variable};

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
