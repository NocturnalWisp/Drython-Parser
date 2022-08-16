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
            internal_functions: HashMap::new(),
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

        // Register all functions.
        for func in &self.parser.global_expressions.internal_expressions
        {
            let scope_info = &func.1.0.scope_info;

            if let Some(func_name) = &scope_info.0
            {
                self.internal_functions.insert(func_name.clone(), Runner::call_internal);
            }
        }
        
        // Register all variables.
        for var in &self.parser.global_expressions.single_op
        {
            let operation = run_operation(self, &var.1.1, &HashMap::new(), error_manager, var.1.2);

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

    pub fn call_function(&self, function_name: &str, args: Vec<Token>, error_manager: &mut ErrorManager) -> Option<Token>
    {
        if self.internal_functions.contains_key(function_name)
        {
            let result = self.internal_functions[function_name](self, function_name, args);
            
            error_manager.merge(result.1);

            return result.0;
        }
        else if self.external_functions.contains_key(function_name)
        {
            if let Some(call) = &self.external_functions[function_name]
            {
                return call(args);
            }
        }
        else
        {
            push_error!(error_manager, RuntimeError::new(0, Some(function_name.to_string()), format!("No function called '{}' exists.", function_name).as_str()));
        }
        
        None
    }

    // Called by a function pointer from a registered internal function.
    fn call_internal(&self, function_name: &str, arguments: Vec<Token>) -> (Option<Token>, ErrorManager)
    {
        let mut error_manager = ErrorManager::new();

        // Try and find if the function exists
        let found_func = self.parser.global_expressions.internal_expressions.iter()
            .find(|x| 
                if let Some(name) = &x.1.0.scope_info.0
                {
                    name == function_name
                } else {false}
            );

        let mut return_result: Option<Token> = None;

        if let Some(function) = found_func
        {
            let mut scope_vars: HashMap<String, Token> = HashMap::new();

            let mut arg_vars: HashMap<String, Token> = HashMap::new();
            if let Some(expected_args) = &function.1.0.scope_info.1
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
                            push_error!(error_manager,
                                RuntimeError::new(function.1.1, Some(function_name.to_string()),
                                    format!("Expected {} arguments, but only recieved {}.", parsed_arg_names.len(), arguments.len()).as_str()));
                        }
                    }
                    Err(error) =>
                    {
                        push_error!(error_manager, RuntimeError::new(function.1.1, Some(function_name.to_string()), error.as_str()));
                    }
                }
            }

            scope_vars.extend(self.vars.clone());
            scope_vars.extend(arg_vars);

            match self.handle_scope(&function.1.0, &mut scope_vars, false, &mut error_manager)
            {
                Ok(result) =>
                {
                    return_result = result;
                }
                Err(error) =>
                {
                    push_error!(error_manager, RuntimeError::new(function.1.1, Some(function_name.to_string()), error.as_str()));
                }
            }
        }

        (return_result, error_manager)
    }

    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    fn handle_scope(&self, function: &ExpressionList, vars: &mut HashMap<String, Token>, is_loop: bool, error_manager: &mut ErrorManager) -> Result<Option<Token>, String>
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
                    let operation = run_operation(self, &expression.1, &vars, error_manager, expression.2);

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
                        match run_operation(self, &tokens, &vars, error_manager, expression.2)
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

                    self.call_function(&expression.0, args, error_manager);
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
                                    let operation = parse_operation(if_op, error_manager, 0);

                                    match run_operation(self, &operation, &vars, error_manager, function.1)
                                    {
                                        Ok(Some(Token::Bool(true))) =>
                                        {
                                            return_result = self.handle_scope(&function.0, vars, is_loop, error_manager);
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
                            },
                            // Only run if "if" failed
                            "elif" =>
                            {
                                if previous_if_failed
                                {
                                    if let Some(if_op) = &function.0.scope_info.1
                                    {
                                        // Only handle scope if the "elif operation" is true.
                                        let operation = parse_operation(if_op, error_manager, 0);

                                        match run_operation(self, &operation, &vars, error_manager, function.1)
                                        {
                                            Ok(Some(Token::Bool(true))) =>
                                            {
                                                return_result = self.handle_scope(&function.0, vars, is_loop, error_manager);
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
                                }
                            },
                            // Only run if "if" failed
                            "else" =>
                            {
                                if previous_if_failed
                                {
                                    return_result = self.handle_scope(&function.0, vars, is_loop, error_manager);
                                }
                            },
                            "loop" =>
                            {
                                loop
                                {
                                    return_result = self.handle_scope(&function.0, vars, true, error_manager);

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
                            _ => return_result = self.handle_scope(&function.0, vars, is_loop, error_manager)
                        }
                    }
                    else
                    {
                        return_result = self.handle_scope(&function.0, vars, is_loop, error_manager);
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
