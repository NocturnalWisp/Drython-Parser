use crate::types::{Runner, Token, ExpressionList, ExpressionListType, VarMap};

use super::operation_runner::run_operation;
use crate::parser::operation_parser::parse_operation;

impl Runner
{
    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    pub fn handle_scope(&mut self, function: &ExpressionList, vars: &mut VarMap, is_loop: bool) -> Result<Option<Token>, (String, usize)>
    {
        let mut return_result: Result<Option<Token>, (String, usize)> = Ok(None);
        
        let mut local_var_refs: Vec<&str> = Vec::new();

        let mut previous_if_failed = false;

        // Follow through and run every expression.
        
        let mut single_index: usize = 0;
        let mut multi_index: usize = 0;
        let mut internal_index: usize = 0;

        let mut i: usize = 0;

        for expression_type in &function.expression_order
        {
            match expression_type
            {
                // Return, Assignement, loop controls.
                ExpressionListType::Single =>
                {
                    let expression = &function.single_op[single_index];
                    let operation = run_operation(self, expression.1.clone(), &vars);

                    single_index += 1;

                    match expression.0.as_str()
                    {
                        "return" =>
                        {
                            match operation
                            {
                                Ok(result) => { return_result = Ok(result); }
                                Err(error) => { return Err((error, function.line_start+i)); }
                            }
                        },
                        "break" =>
                        {
                            if is_loop
                            {
                                // Break completely out of outer scope loop.
                                return_result = Ok(Some(Token::Break));
                                break;
                            }
                        }
                        "continue" =>
                        {
                            if is_loop
                            {
                                // Break out of current loop. Which continues in the outer scope loop.
                                break;
                            }
                        },
                        string =>
                        {
                            match operation
                            {
                                Ok(Some(result)) =>
                                {
                                    // Check if any of the parent hashmap contains this var.
                                    if vars.contains_key(string)
                                    {
                                        vars.entry(string.to_string()).and_modify(
                                            |x|
                                            {
                                                // Make sure not external or change has the same type.
                                                if !x.1 || Token::variant_equal(&result, &x.0)
                                                {
                                                    *x = (result.clone(), x.1);
                                                }

                                                // Add an indicator if an external var has changed.
                                                if x.1 { self.var_indexes_changed.push(string.to_string()); }
                                            }
                                        );
                                    }
                                    else
                                    {
                                        vars.insert(string.to_string(), (result, false));
                                        local_var_refs.push(string)
                                    }
                                },
                                Ok(None) => (),
                                Err(error) =>
                                {
                                    return Err((error, function.line_start+i));
                                }
                            }
                        }
                    }
                },
                // Function call.
                ExpressionListType::Multi =>
                {
                    let expression = &function.multi_ops[multi_index];
                    let mut args: Vec<Token> = vec![];

                    multi_index += 1;

                    for tokens in expression.1.iter()
                    {
                        match run_operation(self, tokens.clone(), &vars)
                        {
                            Ok(Some(result)) =>
                            {
                                args.push(result);
                            }
                            Ok(None) => (),
                            Err(error) =>
                            {
                                return Err((error, function.line_start+i));
                            }
                        }
                    }

                    match self.call(&expression.0, args)
                    {
                        Ok(_) => (),
                        Err(error) => {return Err(error);}
                    }
                },
                // Internal scope.
                ExpressionListType::Internal =>
                {
                    let function = &function.internal_expressions[internal_index];

                    internal_index += 1;

                    if let Some(scope_name) = &function.0.scope_info.0
                    {
                        match scope_name.as_str()
                        {
                            "if" => 
                            {
                                if let Some(if_op) = &function.0.scope_info.1
                                {
                                    // Only handle scope if the "if operation" is true.
                                    match parse_operation(if_op)
                                    {
                                        Ok(operation) =>
                                        {
                                            match run_operation(self, operation.clone(), &vars)
                                            {
                                                Ok(Some(Token::Bool(true))) =>
                                                {
                                                    return_result = self.handle_scope(&function.0, vars, is_loop);
                                                    previous_if_failed = false;
                                                }
                                                Err(error) =>
                                                {
                                                    return Err((error, function.1));
                                                }
                                                _ =>
                                                {
                                                    previous_if_failed = true;
                                                }
                                            }
                                        }
                                        Err(error) => {return Err((error, function.1));}
                                    }
                                }
                            },
                            // Only run if "if" failed
                            "elif" =>
                            {
                                if previous_if_failed
                                {
                                    if let Some(if_op) = &function.0.scope_info.1
                                    {
                                        // Only handle scope if the "elif operation" is true.
                                        match parse_operation(if_op)
                                        {
                                            Ok(operation) =>
                                            {
                                                match run_operation(self, operation.clone(), &vars)
                                                {
                                                    Ok(Some(Token::Bool(true))) =>
                                                    {
                                                        return_result = self.handle_scope(&function.0, vars, is_loop);
                                                        previous_if_failed = false;
                                                    }
                                                    Err(error) =>
                                                    {
                                                        return Err((error, function.1));
                                                    }
                                                    _ =>
                                                    {
                                                        previous_if_failed = true;
                                                    }
                                                }
                                            }
                                            Err(error) => {return Err((error, function.1));}
                                        }
                                    }
                                }
                            },
                            // Only run if "if" failed
                            "else" =>
                            {
                                if previous_if_failed
                                {
                                    return_result = self.handle_scope(&function.0, vars, is_loop);
                                }
                            },
                            "loop" =>
                            {
                                loop
                                {
                                    return_result = self.handle_scope(&function.0, vars, true);

                                    match return_result
                                    {
                                        Ok(Some(Token::Break)) =>
                                        {
                                            return_result = Ok(None);
                                            break;
                                        }
                                        Ok(Some(_)) =>
                                        {
                                            break;
                                        }
                                        _ => ()
                                    }
                                }
                            }
                            _ => return_result = self.handle_scope(&function.0, vars, is_loop)
                        }
                    }
                    else
                    {
                        return_result = self.handle_scope(&function.0, vars, is_loop);
                    }
                },
                _ => ()
            }

            // If found a return statement, break out of the expression loop.
            if let Ok(Some(_)) = return_result
            {
                break;
            }

            i += 1;
        }

        for local_var in local_var_refs
        {
            vars.remove(local_var);
        }

        return_result
    }
}
