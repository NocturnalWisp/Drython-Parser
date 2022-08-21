pub mod operation_runner;
pub mod tester;
mod token_impl;

mod internal_function;

use std::collections::HashMap;

use crate::{types::{Runner, Token, ExpressionList, RegisteredFunction, RegisteredVariable}, utility, external};
use crate::external::auto;
use crate::types::Parser;
use crate::types::error::*;

use self::operation_runner::run_operation;

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
    
    pub fn run_setup(&mut self, error_manager: &mut ErrorManager,
                     additional_libraries: Vec<(Vec<RegisteredFunction>, Vec<RegisteredVariable>)>)
    {
        // Include base external functions and vars.
        let mut functions: Vec<RegisteredFunction> = Vec::new();
        let mut vars: Vec<RegisteredVariable> = Vec::new();

        auto::register_functs(&mut functions);
        auto::register_vars(&mut vars);

        while let Some(function) = functions.pop()
        {
            self.external_functions.insert(function.0, function.1);
        }

        while let Some(var) = vars.pop()
        {
            self.vars.insert(var.0, var.1);
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

        for mut additional_lib in additional_libraries
        {
            while let Some(function) = additional_lib.0.pop()
            {
                self.external_functions.insert(function.0, function.1);
            }

            while let Some(var) = additional_lib.1.pop()
            {
                self.vars.insert(var.0, var.1);
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
                Err(error) => Err(error)
            }
        }
        else if self.external_functions.contains_key(function_name)
        {
            if let Some(call) = &self.external_functions[function_name]
            {
                match call(args)
                {
                    Ok(result) => Ok(result),
                    Err(error) => Err((error, 0))
                }
            }
            else { Ok(None) }
        }
        else
        {
            Err((format!("No function called '{}' exists.", function_name), 0))
        }
    }

    // Called by a function pointer from a registered internal function.
    fn call_internal(&self, function: &ExpressionList, arguments: Vec<Token>) -> Result<Option<Token>, (String, usize)>
    {
        let return_result: Option<Token>;

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
                        return Err((format!("Expected {} arguments, but only recieved {}.", parsed_arg_names.len(), arguments.len()), function.line_start));
                    }
                }
                Err(error) => {return Err((error, function.line_start));}
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
    
    pub fn regiser_external_function(&mut self, function_name: &str, function: fn(Vec<Token>) -> Result<Option<Token>, String>)
    {
        self.external_functions.insert(function_name.to_string(), Some(Box::new(function)));
    }
}
