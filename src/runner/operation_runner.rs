use crate::{types::{Token, Runner, VarMap}, parser::operation_parser, utility};

// recursive function that runs the operation from the reverse polish notation.
pub fn run_operation(runner: &mut Runner, operations: Vec<Token>,
    vars: &VarMap) -> Result<Option<Token>, String>
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
                let handled_1 = handle_token_type(runner, unhandled1, vars, false);
                let handled_2 = handle_token_type(runner, unhandled2, vars, false);

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
                let ran_operation = match (handled_1.unwrap(), handled_2.unwrap())
                {
                    (Some(token1), Some(token2)) => run_operation_by_type(&token1, &token2, operator),
                    (None, Some(token2)) => run_operation_by_type(unhandled1, &token2, operator),
                    (Some(token1), None) => run_operation_by_type(&token1, unhandled2, operator),
                    _ => run_operation_by_type(unhandled1, unhandled2, operator)
                };
                match ran_operation
                {
                    Some(result) =>
                    {
                        stack.push(result);
                    }
                    None => { return Err(format!("Cannot apply operation '{}' to '{}' and '{}'.", operator, unhandled1, unhandled2)); }
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
        match handle_token_type(runner, &token, vars, false)
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

fn handle_token_type(runner: &mut Runner, token: &Token, vars: &VarMap, return_original: bool) -> Result<Option<Token>, String>
{
    // Check for chain of vars first.
    if let Token::Accessor(p, a) = token {
    if let Token::Var(_) = **p { if let Token::Accessor(_, _) = **a {
    if let Some(Ok(Some(result))) = check_var_chain(token)
    {
        return handle_token_type(runner, &result, vars, false);
    }
    }}}

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
                                    run_operation(runner, operation.clone(), vars)
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
            return run_operation(runner, op.clone(), vars);
        },
        Token::Var(name) =>
        {
            // Deal with any accessors.
            let var: Token;

            if vars.contains_key(name)
            {
                var = vars[name].0.clone();
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
                if let Ok(Some(token_result)) = handle_token_type(runner, item, vars, false)
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
            match (handle_token_type(runner, prev_token, vars, false), handle_token_type(runner, accessor, vars, true))
            {
                // Collection.Int -> index
                (Ok(Some(Token::Collection(collection))), Ok(Some(Token::Int(i)))) =>
                {
                    let token = &collection[i as usize];
                    if let Ok(Some(result)) = handle_token_type(runner, token, vars, false)
                    {
                        Ok(Some(result))
                    }
                    else
                    {
                        Ok(Some(token.clone()))
                    }
                },
                // String.String -> variable name
                (Ok(Some(Token::String(value))), Ok(Some(Token::String(accessor_str)))) =>
                {
                    handle_token_type(runner, &Token::Var(format!("{}.{}", value, accessor_str.clone())), vars, false)
                },
                // String.Call() -> call with longer name
                (Ok(Some(Token::String(value))), Ok(Some(Token::Call(name, args)))) =>
                {
                    handle_token_type(runner, &Token::Call(format!("{}.{}", value, name), args), vars, false)
                },
                (Ok(Some(Token::String(value))), Ok(Some(Token::Int(i)))) =>
                {
                    if (i as usize) < value.len() && i >= 0
                    {
                        Ok(Some(Token::String(value.as_str()[(i as usize)..((i+1) as usize)].to_string())))
                    }
                    else
                    {
                        Err("Tried to access a string index out of range.".to_string())
                    }
                },
                (Err(error), Ok(Some(Token::Var(var2)))) =>
                {
                    if let Token::Var(var1) = &**prev_token
                    {
                        let var = Token::Var(format!("{}.{}", var1, var2));
                        handle_token_type(runner, &var, vars, false)
                    }
                    else
                    {
                        Err(error)
                    }
                }
                // Remainder functions that won't be handled properly until conversion.
                (prev, current) =>
                {
                    match &**accessor
                    {
                        Token::Call(name, args) =>
                        {
                            if let Ok(Some(actual_prev)) = &prev
                            {
                                if let Ok(result) = handle_token_type(runner, &Token::Call(name.to_string(), format!("{},{}", actual_prev, args)), vars, return_original)
                                {
                                    return Ok(result);
                                }
                            }
                        }
                        Token::Var(var2) =>
                        {
                            if let Token::Var(var1) = &**prev_token
                            {
                                let var = Token::Var(format!("{}.{}", var1, var2));
                                let handling = handle_token_type(runner, &var, vars, false);
                                if let Err(_) = handling
                                {
                                    return Ok(Some(var));
                                }
                            }
                        }
                        _ => ()
                    }
                    
                    match (prev, current)
                    {
                        (Err(error), _) => Err(error),
                        (_, Err(error)) => Err(error),
                        (_, _) => Ok(None)
                    }
                },
            }
        }
        _ => Ok(if return_original { Some(token.clone()) } else { None })
    }
}

fn check_var_chain(token: &Token) -> Option<Result<Option<Token>, String>>
{
    let mut var = String::new();
    if let Ok(i) = check_var_chain_recursive(token, &mut var)
    {
        var = var[1..].to_string();
        Some(Ok(Some(Token::Accessor(Box::new(Token::Var(var)), Box::new(Token::Int(i))))))
    }
    else { None }
}

fn check_var_chain_recursive(token: &Token, built_string: &mut String) -> Result<i32, ()>
{
    match token
    {
        Token::Accessor(prev, accessor) =>
        {
            if let Token::Var(var) = &**prev
            {
                built_string.push_str(format!(".{}", var).as_str());

                if let Token::Accessor(_, _) = &**accessor
                {
                    check_var_chain_recursive(&accessor, built_string)
                }
                else if let Token::Int(i) = &**accessor
                {
                    Ok(*i)
                }
                else
                {
                    Err(())
                }
            }
            else
            {
                Err(())
            }
        },
        _ => Err(())
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
