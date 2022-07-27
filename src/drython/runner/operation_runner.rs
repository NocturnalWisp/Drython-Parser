use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;

use crate::drython::{types::{Token, Runner}, parser::operation_parser, utility};

// recursive function that runs the operation from the reverse polish notation.
pub fn run_operation(runner: &Runner, operations: &Vec<Token>,
    vars: &HashMap<String, Token>) -> Option<Token>
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
                let handled_1 = handle_token_type(runner, unhandled1, vars);
                let handled_2 = handle_token_type(runner, unhandled2, vars);

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
        if let Some(handled_token) = handle_token_type(runner, &token, vars)
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

fn handle_token_type(runner: &Runner, token: &Token, vars: &HashMap<String, Token>) -> Option<Token>
{
    // Stores a string buffer of an accessors string.
    let mut accessor_string = String::new();

    match token
    {
        Token::Call(name, args) =>
        {
            // Parse arguments and run them recursively.
            let mut parsed_args: Vec<Token> = Vec::new();

            for arg in utility::split_by(args, ',')
            {
                if let Some(ran_token) = run_operation(runner, &operation_parser::parse_operation(&arg, &mut LinkedHashMap::new()), vars)
                {
                    parsed_args.push(ran_token);
                }
            }
            
            return runner.call_function(name, parsed_args);
        },
        Token::Operation(op) =>
        {
            // Run operation recursively.
            return run_operation(runner, op, vars);
        },
        Token::Var(name) =>
        {
            // Deal with any accessors.
            let mut var: Token = Token::Null;

            if vars.contains_key(name)
            {
                var = vars[name].clone();
            }
            else
            {
                var = Token::String(name.clone());
            }

            return Some(var);
        },
        Token::Collection(items) =>
        {
            let mut new_items: Vec<Token> = Vec::new();

            for item in items
            {
                if let Some(token_result) = handle_token_type(runner, item, vars)
                {
                    new_items.push(token_result);
                }
                else
                {
                    new_items.push(item.clone());
                }
            }

            return Some(Token::Collection(new_items));
        },
        Token::Accessor(accessor, prev_token) =>
        {
            match handle_token_type(runner, prev_token, vars)
            {
                Some(Token::Collection(collection)) =>
                {
                    if let Token::Int(i) = **accessor
                    {
                        let token = &collection[i as usize];
                        if let Some(result) = handle_token_type(runner, token, vars)
                        {
                            return Some(result);
                        }
                        else
                        {
                            return Some(token.clone());
                        }
                    }
                    else
                    {
                        return None;
                    }
                },
                Some(Token::Var(value)) =>
                {
                    accessor_string.push_str(&value);
                }
                Some(Token::Accessor(second_accessor, second_prev)) =>
                {
                    match (handle_token_type(runner, &second_accessor, vars), handle_token_type(runner, &second_prev, vars))
                    {
                        (Some(Token::String(next_var)), Some(Token::String(prev_var))) =>
                        {
                            accessor_string.push_str(string);
                        }
                        (a, b) => { return b; }
                    }
                },
                _ => { return None; }
            }

            // Handle call.

        }
        _ => { return None; }
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
