pub mod operation_runner;
pub mod tester;
mod token_impl;

use std::collections::HashMap;

use crate::Parser;

use self::operation_runner::run_operation;

use super::{types::{Runner, Token, ExpressionList, RegisteredFunction, RegisteredVariable}, parser::operation_parser::parse_operation, utility, external};

use super::external::auto;

use crate::drython::types::error::*;

impl Runner
{
    pub fn new(parser: Parser) -> Runner
    {
        Runner
        {
            parser,
            external_functions: HashMap::new(),
            vars: HashMap::new(),
        }
    }
    
    pub fn run_setup(& mut self, error_manager: &mut ErrorManager)
    {
        // Include base external functions and vars.
        let mut functions: Vec<RegisteredFunction> = Vec::new();
        let mut vars: Vec<RegisteredVariable> = Vec::new();

        auto::register_functs(&mut functions);
        auto::register_vars(&mut vars);

        while let Some(function) = functions.pop()
        {
            self.external_functions.insert(function.0.clone(), function.1);
        }

        while let Some(var) = vars.pop()
        {
            self.vars.insert(var.0.clone(), var.1.clone());
        }

        // Include libs
        for library in &self.parser.global_expressions.includes
        {
            let found_lib = external::get_lib(library.0);

            match found_lib
            {
                Ok(mut lib) =>
                {
                    while let Some(function) = lib.0.pop()
                    {
                        self.external_functions.insert(function.0.clone(), function.1);
                    }

                    while let Some(var) = lib.1.pop()
                    {
                        self.vars.insert(var.0.clone(), var.1.clone());
                    }
                },
                Err(error) =>
                {
                    push_error!(error_manager, RuntimeError::new(*library.1, None, error.as_str()));
                }
            }
        }

        // Register all variables.
        for var in &self.parser.global_expressions.single_op
        {
            let operation = run_operation(self, &var.1.1, &HashMap::new());

            match operation
            {
                Ok(Some(result)) =>
                {
                    self.vars.insert(var.1.0.to_string(), result);
                }
                Err(error) =>
                {
                    push_error!(error_manager, RuntimeError::new(var.1.2, None, error.as_str()));
                }
                _ => ()
            }
        }
    }

    pub fn  call_function(&self, function_name: &str, args: Vec<Token>, error_manager: &mut ErrorManager) -> Option<Token>
    {
        match self.call(function_name, args)
        {
            Ok(result) => {return result;}
            Err((error, line_number)) => {push_error!(error_manager, RuntimeError::new(line_number, Some(function_name.to_string()), error.as_str())); return None;}
        }
    }

    fn call(&self, function_name: &str, args: Vec<Token>) -> Result<Option<Token>, (String, usize)>
    {
        let found_internal = self.parser.global_expressions.internal_expressions.iter()
            .find(|x| 
                if let Some(name) = &x.1.0.scope_info.0
                {
                    name == function_name
                } else {false}
            );
        if let Some(function) = found_internal
        {
            match self.call_internal(&function.1.0, args)
            {
                Ok(result) => Ok(result),
                Err(error) => Err((error, function.1.1))
            }
        }
        else if self.external_functions.contains_key(function_name)
        {
            if let Some(call) = &self.external_functions[function_name]
            {
                Ok(call(args))
            }
            else { Ok(None) }
        }
        else
        {
            Err((format!("No function called '{}' exists.", function_name), 0))
        }
    }

    // Called by a function pointer from a registered internal function.
    fn call_internal(&self, function: &ExpressionList, arguments: Vec<Token>) -> Result<Option<Token>, String>
    {
        let mut return_result: Option<Token> = None;

        let mut scope_vars: HashMap<String, Token> = HashMap::new();

        let mut arg_vars: HashMap<String, Token> = HashMap::new();
        if let Some(expected_args) = &function.scope_info.1
        {
            match utility::split_by(expected_args, ',')
            {
                Ok(mut parsed_arg_names) =>
                {
                    if parsed_arg_names.len() == arguments.len()
                    {
                        for i in 0..parsed_arg_names.len()
                        {
                            arg_vars.insert(
                                parsed_arg_names.pop().unwrap_or("null".to_string()),
                                arguments[i].clone()
                            );
                        }
                    }
                    else
                    {
                        return Err(format!("Expected {} arguments, but only recieved {}.", parsed_arg_names.len(), arguments.len()));
                    }
                }
                Err(error) => {return Err(error);}
            }
        }

        scope_vars.extend(self.vars.clone());
        scope_vars.extend(arg_vars);

        match self.handle_scope(&function, &mut scope_vars, false)
        {
            Ok(result) =>
            {
                return_result = result;
            }
            Err(error) => {return Err(error);}
        }

        Ok(return_result)
    }

    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    fn handle_scope(&self, function: &ExpressionList, vars: &mut HashMap<String, Token>, is_loop: bool) -> Result<Option<Token>, String>
    {
        let mut return_result: Result<Option<Token>, String> = Ok(None);
        
        let mut local_var_refs: Vec<&str> = Vec::new();

        let mut previous_if_failed = false;

        // Follow through and run every expression.
        for i in 0..function.size
        {
            match i
            {
                // Return, Assignement, loop controls.
                _ if function.single_op.contains_key(&i) =>
                {
                    let expression = &function.single_op[&i];
                    let operation = run_operation(self, &expression.1, &vars);

                    match expression.0.as_str()
                    {
                        "return" =>
                        {
                            return_result = operation;
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
                                    // Check if any of the parent hashmaps contains this var.
                                    if vars.contains_key(string)
                                    {
                                        vars.entry(string.to_string()).and_modify(
                                            |x|
                                            {
                                                *x = result
                                            });
                                    }
                                    else
                                    {
                                        vars.insert(string.to_string(), result);
                                        local_var_refs.push(string)
                                    }
                                },
                                Ok(None) => (),
                                Err(error) =>
                                {
                                    return Err(error);
                                }
                            }
                        }
                    }
                },
                // Function call.
                _ if function.multi_ops.contains_key(&i) =>
                {
                    let expression = &function.multi_ops[&i];
                    let mut args: Vec<Token> = vec![];

                    for tokens in expression.1.iter()
                    {
                        match run_operation(self, &tokens, &vars)
                        {
                            Ok(Some(result)) =>
                            {
                                args.push(result);
                            }
                            Ok(None) => (),
                            Err(error) =>
                            {
                                return Err(error);
                            }
                        }
                    }

                    match self.call(&expression.0, args)
                    {
                        Ok(_) => (),
                        Err(error) => {return Err(error.0);}
                    }
                },
                // Internal scope.
                _ if function.internal_expressions.contains_key(&i) =>
                {
                    let function = &function.internal_expressions[&i];
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
                                            match run_operation(self, &operation, &vars)
                                            {
                                                Ok(Some(Token::Bool(true))) =>
                                                {
                                                    return_result = self.handle_scope(&function.0, vars, is_loop);
                                                    previous_if_failed = false;
                                                }
                                                Err(error) =>
                                                {
                                                    return Err(error);
                                                }
                                                _ =>
                                                {
                                                    previous_if_failed = true;
                                                }
                                            }
                                        }
                                        Err(error) => {return Err(error);}
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
                                                match run_operation(self, &operation, &vars)
                                                {
                                                    Ok(Some(Token::Bool(true))) =>
                                                    {
                                                        return_result = self.handle_scope(&function.0, vars, is_loop);
                                                        previous_if_failed = false;
                                                    }
                                                    Err(error) =>
                                                    {
                                                        return Err(error);
                                                    }
                                                    _ =>
                                                    {
                                                        previous_if_failed = true;
                                                    }
                                                }
                                            }
                                            Err(error) => {return Err(error);}
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
        }

        for local_var in local_var_refs
        {
            vars.remove(local_var);
        }

        return_result
    }
    
    pub fn regiser_external_function(&mut self, function_name: &str, function: fn(Vec<Token>) -> Option<Token>)
    {
        self.external_functions.insert(function_name.to_string(), Some(Box::new(function)));
    }
}
