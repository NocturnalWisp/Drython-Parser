use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

use crate::drython::{types::{Token, Runner}, parser::operation_parser};

// recursive function that runs the operation from the reverse polish notation.
pub fn run_operation(runner: &Runner, operations: &Vec<Token>,
    parent_vars: &HashMap<&String, &Token>) -> Option<Token>
{
    let mut stack: Vec<Token> = vec![];

    for i in (0..operations.len()).rev()
    {
        if let Token::Operator(operator) = &operations[i]
        {
            let second = stack.pop();
            let first = stack.pop();

            if let (Some(unhandled1), Some(unhandled2)) = (&first, &second)
            {
                let handled_1 = handle_token_type(runner, unhandled1, parent_vars);
                let handled_2 = handle_token_type(runner, unhandled2, parent_vars);

                // Run operations based on whether the token has been handled or not.
                if let Some(result) = match (handled_1, handled_2)
                {
                    (Some(token1), Some(token2)) => run_operation_by_type(&token1, &token2, operator),
                    (None, Some(token2)) => run_operation_by_type(unhandled1, &token2, operator),
                    (Some(token1), None) => run_operation_by_type(&token1, unhandled2, operator),
                    _ => run_operation_by_type(unhandled1, unhandled2, operator)
                }
                {
                    stack.push(result);
                }
            }
        }
        else 
        {
            // Push the last onto the stack.
            stack.push(operations[i].clone());
        }
    }

    if let Some(token) = stack.pop()
    {
        // Secondary handle in case it was a single token and not a full operation.
        if let Some(handled_token) = handle_token_type(runner, &token, parent_vars)
        {
            return Some(handled_token);
        }
        else
        {
            return Some(token);
        }
    }

    None
}

fn handle_token_type(runner: &Runner, token: &Token, parent_vars: &HashMap<&String, &Token>) -> Option<Token>
{
    match token
    {
        Token::Call(name, args) =>
        {
            // Parse arguments and run them recursively.
            let mut parsed_args: Vec<Token> = Vec::new();

            for arg in args.split(",")
            {
                if let Some(ran_token) = run_operation(runner, &operation_parser::parse_operation(arg, &mut LinkedHashMap::new()), parent_vars)
                {
                    parsed_args.push(ran_token);
                }
            }
            
            return runner.call_function(name, parsed_args);
        },
        Token::Operation(op) =>
        {
            // Run operation recursively.
            return run_operation(runner, op, parent_vars);
        },
        Token::Var(name) =>
        {
            if parent_vars.contains_key(name)
            {
                return Some(parent_vars[name].clone());
            }
        },
        _ => { }
    }

    None
}

// Handles the various operations and conversions.
fn run_operation_by_type(a: &Token, b: &Token, operation: &str) -> Option<Token>
{
    match operation
    {
        "+" => a.add(b),
        "-" => a.subtract(b),

        "*" => a.multiply(b),
        "/" => a.divide(b),
        "%" => a.modulos(b),

        "||" => a.or(b),
        "&&" => a.and(b),

        ">=" => a.compare_gte(b),
        ">" => a.compare_gt(b),
        "<=" => a.compare_lte(b),
        "<" => a.compare_lt(b),
        "==" => a.compare_eq(b),
        "!=" => a.compare_neq(b),
        _ => None
    }
}
