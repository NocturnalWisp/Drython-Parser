use std::collections::HashMap;

use crate::drython::{types::{Token, Runner}, parser::operation_parser, utility};

use crate::drython::types::error::*;

// recursive function that runs the operation from the reverse polish notation.
pub fn run_operation(runner: &Runner, operations: &Vec<Token>,
    vars: &HashMap<String, Token>, error_args: &mut RuntimeErrorArguments) -> Option<Token>
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
                let handled_1 = handle_token_type(runner, unhandled1, vars, error_args);
                let handled_2 = handle_token_type(runner, unhandled2, vars, error_args);

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
        if let Some(handled_token) = handle_token_type(runner, &token, vars, error_args)
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

fn handle_token_type(runner: &Runner, token: &Token, vars: &HashMap<String, Token>, error_args: &mut RuntimeErrorArguments) -> Option<Token>
{
    match token
    {
        Token::Call(name, args) =>
        {
            // Parse arguments and run them recursively.
            let mut parsed_args: Vec<Token> = Vec::new();

            match utility::split_by(args, ',')
            {
                Ok(result) =>
                {
                    for arg in result
                    {
                        if let Some(ran_token) = 
                            run_operation(runner,
                                &operation_parser::parse_operation(&arg, &mut error_args.2, 0),
                                vars, error_args
                            )
                        {
                            parsed_args.push(ran_token);
                        }
                    }
                }
                Err(error) =>
                {
                    push_error!(error_args.2, RuntimeError::new(error_args.0, error_args.1.clone(), error.as_str()));
                }
            }
            
            let call_result = runner.call_function(name, parsed_args, error_args.2);
            
            if let None = call_result
            {
                return Some(token.clone());
            }
            else
            {
                return call_result;
            }
        },
        Token::Operation(op) =>
        {
            // Run operation recursively.
            return run_operation(runner, op, vars, error_args);
        },
        Token::Var(name) =>
        {
            // Deal with any accessors.
            let var: Token;

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
                if let Some(token_result) = handle_token_type(runner, item, vars, error_args)
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
        Token::Accessor(prev_token, accessor) =>
        {
            let handled_accessor = handle_token_type(runner, accessor, vars, error_args);
            match handle_token_type(runner, prev_token, vars, error_args)
            {
                Some(Token::Collection(collection)) =>
                {
                    match handled_accessor
                    {
                        Some(Token::Int(i)) =>
                        {
                            let token = &collection[i as usize];
                            if let Some(result) = handle_token_type(runner, token, vars, error_args)
                            {
                                return Some(result);
                            }
                            else
                            {
                                return Some(token.clone());
                            }
                        }
                        _ => { return None; }
                    }
                },
                Some(Token::String(value)) =>
                {
                    match handled_accessor
                    {
                        Some(Token::String(accessor_str)) =>
                        {
                            return handle_token_type(runner, &Token::Var(format!("{}.{}", value, accessor_str.clone())), vars, error_args);
                        }
                        Some(Token::Call(name, args)) =>
                        {
                            return handle_token_type(runner, &Token::Call(format!("{}.{}", value, name), args), vars, error_args);
                        }
                        _ => { return None; }
                    }
                }
                _ => { return None; }
            }

            // Handle call.

        }
        _ => { return None; }
    }
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
