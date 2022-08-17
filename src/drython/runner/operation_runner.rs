use std::collections::HashMap;

use crate::drython::{types::{Token, Runner}, parser::operation_parser, utility};

// recursive function that runs the operation from the reverse polish notation.
pub fn run_operation(runner: &Runner, operations: &Vec<Token>,
    vars: &HashMap<String, Token>) -> Result<Option<Token>, String>
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

                if let Err(error) = handled_1
                {
                    return Err(error);
                }
                if let Err(error) = handled_2
                {
                    return Err(error);
                }

                // Run operations based on whether the token has been handled or not.
                // Unwrap because errors automatically return from this function before.
                if let Some(result) = match (handled_1.unwrap(), handled_2.unwrap())
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
        match handle_token_type(runner, &token, vars)
        {
            Ok(Some(result)) =>
            {
                Ok(Some(result))
            }
            Ok(None) =>
            {
                Ok(Some(token))
            }
            Err(error) =>
            {
                Err(error)
            }
        }
    }
    else
    {
            Err("Failed to parse operation. Try breaking down the statement into steps.".to_string())
    }
}

fn handle_token_type(runner: &Runner, token: &Token, vars: &HashMap<String, Token>) -> Result<Option<Token>, String>
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
                        match &operation_parser::parse_operation(&arg)
                        {
                            Ok(operation) =>
                            {
                                if let Ok(Some(ran_token)) = 
                                    run_operation(runner, operation, vars)
                                {
                                    parsed_args.push(ran_token);
                                }
                            }
                            Err(error) => {return Err(error.clone());}
                        }
                    }
                }
                Err(error) =>
                {
                    return Err(error);
                }
            }
            
            let call_result = runner.call(name, parsed_args);
            
            match &call_result
            {
                Ok(None) =>
                {
                    return Ok(Some(token.clone()));
                }
                Ok(Some(result)) =>
                {
                    return Ok(Some(result.clone()));
                }
                Err(error) => {return Err(error.0.clone());}
            }
        },
        Token::Operation(op) =>
        {
            // Run operation recursively.
            return run_operation(runner, op, vars);
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
                return Err(format!("Could not find a variable by the name: {}", name));
            }

            return Ok(Some(var));
        },
        Token::Collection(items) =>
        {
            let mut new_items: Vec<Token> = Vec::new();

            for item in items
            {
                if let Ok(Some(token_result)) = handle_token_type(runner, item, vars)
                {
                    new_items.push(token_result);
                }
                else
                {
                    new_items.push(item.clone());
                }
            }

            return Ok(Some(Token::Collection(new_items)));
        },
        Token::Accessor(prev_token, accessor) =>
        {
            let handled_accessor = handle_token_type(runner, accessor, vars);
            match handle_token_type(runner, prev_token, vars)
            {
                Ok(Some(Token::Collection(collection))) =>
                {
                    match handled_accessor
                    {
                        Ok(Some(Token::Int(i))) =>
                        {
                            let token = &collection[i as usize];
                            if let Ok(Some(result)) = handle_token_type(runner, token, vars)
                            {
                                return Ok(Some(result));
                            }
                            else
                            {
                                return Ok(Some(token.clone()));
                            }
                        }
                        Ok(None) =>
                        {
                            if let Token::Int(i) = &**accessor
                            {
                                let token = &collection[*i as usize];
                                if let Ok(Some(result)) = handle_token_type(runner, token, vars)
                                {
                                    return Ok(Some(result));
                                }
                                else
                                {
                                    return Ok(Some(token.clone()));
                                }
                            }
                            else
                            {
                                return Ok(None);
                            }
                        }
                        Err(error) =>
                        {
                            return Err(error);
                        }
                        _ => { return Ok(None); }
                    }
                },
                Ok(Some(Token::String(value))) =>
                {
                    match handled_accessor
                    {
                        Ok(Some(Token::String(accessor_str))) =>
                        {
                            return handle_token_type(runner, &Token::Var(format!("{}.{}", value, accessor_str.clone())), vars);
                        }
                        Ok(Some(Token::Call(name, args))) =>
                        {
                            return handle_token_type(runner, &Token::Call(format!("{}.{}", value, name), args), vars);
                        }
                        Err(error) =>
                        {
                            return Err(error);
                        }
                        _ => { return Ok(None); }
                    }
                }
                Ok(None) =>
                {
                    if let Token::String(value) = &**prev_token
                    {
                        match handled_accessor
                        {
                            Ok(Some(Token::Int(i))) =>
                            {
                                if (i as usize) < value.len() && i >= 0
                                {
                                    return Ok(Some(Token::String(value.as_str()[(i as usize)..((i+1) as usize)].to_string())));
                                }
                                else
                                {
                                    return Err("Tried to access a string index out of range.".to_string());
                                }
                            }
                            Ok(None) =>
                            {
                                if let Token::Int(i) = &**accessor
                                {
                                    if (*i as usize) < value.len() && *i >= 0
                                    {
                                        return Ok(Some(Token::String(value.as_str()[(*i as usize)..((i+1) as usize)].to_string())));
                                    }
                                    else
                                    {
                                        return Err("Tried to access a string index out of range.".to_string());
                                    }
                                }
                                else
                                {
                                    return Ok(None);
                                }
                            }
                            Err(error) =>
                            {
                                return Err(error);
                            }
                            _ => { return Ok(None); }
                        }
                    }
                    else
                    {
                        return Ok(None);
                    }
                }
                _ => { return Ok(None); }
            }

            // Handle call.

        }
        _ => { return Ok(None); }
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
