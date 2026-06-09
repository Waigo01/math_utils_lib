use crate::{Context, Value, basetypes::{AST, Operation, SimpleOpType}, errors::EvalError, maths::num_trait::Number, roots::RootFinder};

#[deprecated(since="0.4.0", note="This functionality has been directly implemented into the eval process using the eq AdvancedOperator")]
/// used to solve an equation or a system of equations.
///
/// This function takes a Vec of a Tuple of ASTs, which are the parsed left and right sides of
/// the equation(s). Multiple Tuples describe a system of equations. It also takes the global context.
pub fn solve<N: Number>(equations: Vec<(AST, AST)>, context: &Context<N>, search_vars: Vec<String>) -> Result<Vec<Value<N>>, EvalError> {
    let mut final_expressions = vec![];

    for i in &equations {
        let root_b = AST::from_operation(Operation::SimpleOperation {
            op_type: SimpleOpType::Sub,
            left: i.0.clone(),
            right: i.1.clone()
        });

        final_expressions.push(root_b);
    }
    let root_finder = RootFinder::new(final_expressions, context.to_owned(), search_vars)?;
    return root_finder.find_roots();
}
