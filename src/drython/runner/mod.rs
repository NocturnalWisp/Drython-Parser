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
            let mut lib = external::get_lib(library);

            while let Some(function) = lib.0.pop()
            {
                self.external_functions.insert(function.0.clone(), function.1);
            }

            while let Some(var) = lib.1.pop()
            {
                self.vars.insert(var.0.clone(), var.1.clone());
            }
        }

        // Register all functions.
        for func in &self.parser.global_expressions.internal_expressions
        {
            let scope_info = &func.1.scope_info;

            if let Some(func_name) = &scope_info.0
            {
                self.internal_functions.insert(func_name.clone(), Runner::call_internal);
            }
        }
        
        // Register all variables.
        for var in &self.parser.global_expressions.single_op
        {
            //TODO: Find a way to inject the proper line number.
            let operation = run_operation(self, &var.1.1, &HashMap::new(), &mut (0, None, error_manager));

            if let Some(result) = operation
            {
                self.vars.insert(var.1.0.to_string(), result);
            }
        }
    }

    pub fn call_function(&self, function_name: &str, args: Vec<Token>) -> Option<Token>
    {
        if self.internal_functions.contains_key(function_name)
        {
            return self.internal_functions[function_name](self, function_name, args);
        }

        if self.external_functions.contains_key(function_name)
        {
            if let Some(call) = &self.external_functions[function_name]
            {
                return call(args);
            }
        }

        None
    }

    // Called by a function pointer from a registered internal function.
    fn call_internal(&self, function_name: &str, arguments: Vec<Token>) -> Option<Token>
    {
        let mut error_manager = ErrorManager::new();

        // Try and find if the function exists
        let found_func = self.parser.global_expressions.internal_expressions.iter()
            .find(|x| 
                if let Some(name) = &x.1.scope_info.0
                {
                    name == function_name
                } else {false}
            );

        let mut return_result: Option<Token> = None;

        if let Some(function) = found_func
        {
            let mut scope_vars: HashMap<String, Token> = HashMap::new();

            let mut arg_vars: HashMap<String, Token> = HashMap::new();
            if let Some(expected_args) = &function.1.scope_info.1
            {
                match utility::split_by(expected_args, ',')
                {
                    Ok(mut parsed_arg_names) =>
                    {
                        for i in 0..parsed_arg_names.len()
                        {
                            arg_vars.insert(
                                parsed_arg_names.pop().unwrap_or("null".to_string()),
                                arguments[i].clone()
                            );
                        }
                    }
                    Err(error) =>
                    {
                        //TODO: Figure out a way to inject the line number to the error.
                        push_error!(error_manager, RuntimeError::new(0, Some(function_name.to_string()), error.as_str()));
                    }
                }
            }

            scope_vars.extend(self.vars.clone());
            scope_vars.extend(arg_vars);

            //TODO: Line injection
            return_result = self.handle_scope(function.1, &mut scope_vars, false, &mut (0, Some(function_name.to_string()), &mut error_manager));
        }

        return_result
    }

    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    fn handle_scope(&self, function: &ExpressionList, vars: &mut HashMap<String, Token>, is_loop: bool, errors_args: &mut RuntimeErrorArguments) -> Option<Token>
    {
        let mut return_result: Option<Token> = None;
        
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
                    let operation = run_operation(self, &expression.1, &vars, errors_args);

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
                                return_result = Some(Token::Break);
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
                            if let Some(result) = operation
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
                        if let Some(token) = run_operation(self, &tokens, &vars, errors_args)
                        {
                            args.push(token);
                        }
                    }

                    self.call_function(&expression.0, args);
                },
                // Internal scope.
                _ if function.internal_expressions.contains_key(&i) =>
                {
                    let function = &function.internal_expressions[&i];
                    if let Some(scope_name) = &function.scope_info.0
                    {
                        match scope_name.as_str()
                        {
                            "if" => 
                            {
                                if let Some(if_op) = &function.scope_info.1
                                {
                                    // Only handle scope if the "if operation" is true.
                                    let operation = parse_operation(if_op, errors_args.2, 0);

                                    if let Some(Token::Bool(true)) = run_operation(self, &operation, &vars, errors_args)
                                    {
                                        return_result = self.handle_scope(function, vars, is_loop, errors_args);
                                        previous_if_failed = false;
                                    }
                                    else
                                    {
                                        previous_if_failed = true;
                                    }
                                }
                            },
                            // Only run if "if" failed
                            "elif" =>
                            {
                                if previous_if_failed
                                {
                                    if let Some(if_op) = &function.scope_info.1
                                    {
                                        // Only handle scope if the "elif operation" is true.
                                        let operation = parse_operation(if_op, errors_args.2, 0);

                                        if let Some(Token::Bool(true)) = run_operation(self, &operation, &vars, errors_args)
                                        {
                                            return_result = self.handle_scope(function, vars, is_loop, errors_args);
                                            previous_if_failed = false;
                                        }
                                        else
                                        {
                                            previous_if_failed = true;
                                        }
                                    }
                                }
                            },
                            // Only run if "if" failed
                            "else" =>
                            {
                                if previous_if_failed
                                {
                                    return_result = self.handle_scope(function, vars, is_loop, errors_args);
                                }
                            },
                            "loop" =>
                            {
                                loop
                                {
                                    return_result = self.handle_scope(function, vars, true, errors_args);

                                    if let Some(Token::Break) = return_result
                                    {
                                        return_result = None;
                                        break;
                                    }
                                    else if let Some(_) = return_result
                                    {
                                        break;
                                    }
                                }
                            }
                            _ => return_result = self.handle_scope(function, vars, is_loop, errors_args)
                        }
                    }
                    else
                    {
                        return_result = self.handle_scope(function, vars, is_loop, errors_args);
                    }
                },
                _ => ()
            }

            // If found a return statement, break out of the expression loop.
            if let Some(_) = return_result
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
