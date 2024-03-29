pub mod operation_runner;
pub mod modifier_runner;
pub mod tester;
mod token_impl;

mod internal_function;

use crate::types::VariableModifier;
use crate::types::ExFnRef;
use std::collections::HashMap;

use crate::{types::{Runner, Token, RegisteredFunction, RegisteredVariable, VarMap, BoxedCall}, utility, external};
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
            var_indexes_changed: Vec::new(),
        }
    }
    
    pub fn run_setup(&mut self, error_manager: &mut ErrorManager) -> &mut Self
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
            self.vars.insert(var.0, (var.1, true, vec![]));
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
                        self.external_functions.insert(function.0, function.1);
                    }

                    while let Some(var) = lib.1.pop()
                    {
                        self.vars.insert(var.0, (var.1, true, vec![]));
                    }
                },
                Err(error) =>
                {
                    push_error!(error_manager, RuntimeError::new(*library.1, None, error.as_str()));
                }
            }
        }

        // Register all variables.
        self.parser.global_expressions.single_op.clone().iter().for_each(|x|
            {
                let operation = run_operation(self, &x.2, &HashMap::new());

                match operation
                {
                    Ok(Some(result)) =>
                    {
                        let modifier_list = x.1.iter().map(|x| From::from(x.as_str())).collect::<Vec<VariableModifier>>();

                        if let Err(error) = self.handle_variable_modifiers(x, result, modifier_list, false, None)
                        {
                            push_error!(error_manager,
                                RuntimeError::new(x.3, None, error.as_str()));
                        }
                    }
                    Err(error) =>
                    {
                        push_error!(error_manager, RuntimeError::new(x.3, None, error.as_str()));
                    }
                    _ => ()
                }
            });

        self
    }

    pub fn  call_function(&mut self, function_name: &str, args: Vec<Token>, error_manager: &mut ErrorManager) -> Option<Token>
    {
        match self.call(function_name, args, 0)
        {
            Ok(result) => {return result;}
            Err((error, line_number)) => {push_error!(error_manager, RuntimeError::new(line_number, Some(function_name.to_string()), error.as_str())); return None;}
        }
    }

    fn call(&mut self, function_name: &str, args: Vec<Token>, line_number: usize) -> Result<Option<Token>, (String, usize)>
    {
        if self.external_functions.contains_key(function_name)
        {
            let function = &self.external_functions[function_name];
            if let Some(call) = &function.1
            {
                match call(self.external_functions[function_name].0, args)
                {
                    Ok(result) => Ok(result),
                    Err(error) => Err((error, 0))
                }
            }
            else { Ok(None) }
        }
        else
        {
            let index = self.parser.global_expressions.internal_expressions.iter()
                .position(|x| 
                    if let Some(name) = &x.0.scope_info.0
                    {
                        if name == function_name
                        {
                            true
                        }
                        else { false }
                    } else { false }
                );

            if let Some(index) = index
            {
                self.call_internal(&index, args)
            }
            else
            {
                Err((format!("No function called '{}' exists.", function_name), line_number))
            }
        }
    }

    // Called by a function pointer from a registered internal function.
    fn call_internal(&mut self, expression_index: &usize, mut arguments: Vec<Token>) -> Result<Option<Token>, (String, usize)>
    {
        let return_result: Option<Token>;

        let mut scope_vars: VarMap = HashMap::new();

        let mut arg_vars: VarMap = HashMap::new();

        let function = self.parser.global_expressions.internal_expressions[*expression_index].0.clone();
        if let Some(expected_args) = &function.scope_info.1
        {
            match utility::split_by(expected_args.as_str(), ',')
            {
                Ok(parsed_arg_names) =>
                {
                    if parsed_arg_names.len() == arguments.len()
                    {
                        arguments.reverse();
                        arg_vars.extend(parsed_arg_names.into_iter()
                            .map(|x| (x, (arguments.pop().unwrap(), false, vec![])))
                        );
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

        self.vars = scope_vars;

        Ok(return_result)
    }
    
    pub fn register_external_function(&mut self, function_name: &str,
        optional_identifier: Option<*mut dyn ExFnRef>, function: BoxedCall) -> &mut Self
    {
        self.external_functions.insert(function_name.to_string(), (optional_identifier, Some(Box::new(function))));

        self
    }

    pub fn register_library(&mut self, mut additional_library: (Vec<RegisteredFunction>, Vec<RegisteredVariable>)) -> &mut Self
    {
        while let Some(function) = additional_library.0.pop()
        {
            self.external_functions.insert(function.0, function.1);
        }

        while let Some(var) = additional_library.1.pop()
        {
            self.vars.insert(var.0, (var.1, true, vec![]));
        }
        
        self
    }

    pub fn register_variable(&mut self, name: &str, initial_value: Token) -> &mut Self
    {
        self.vars.insert(name.to_string(), (initial_value, true, vec![]));

        self
    }

    pub fn register_variables(&mut self, map: HashMap<String, Token>) -> &mut Self
    {
        self.vars.extend(map.into_iter().map(|x| (x.0, (x.1, true, vec![]))));

        self
    }
    
    pub fn update_variable<T>(&mut self, external_var: (&str, &mut T)) -> & mut Self
        where T: From<Token>
    {
        if self.var_indexes_changed.iter().any(|x| x == external_var.0)
        {
            *external_var.1 = From::from(self.vars[external_var.0].0.clone());
        }

        self
    }

    pub fn update_variable_conversion<T>(&mut self, external_var: (&str, &mut T), conversion_function: fn(&Token) -> T) -> &mut Self
    {
        if self.var_indexes_changed.iter().any(|x| x == external_var.0)
        {
            *external_var.1 = conversion_function(&self.vars[external_var.0].0);
        }

        self
    }

    
}
