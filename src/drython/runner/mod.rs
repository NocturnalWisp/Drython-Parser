pub mod operation_runner;

use std::{str::FromStr, collections::HashMap, hash::Hash};

use crate::Parser;

use self::operation_runner::run_operation;

use super::types::{Runner, Token, ExpressionList};

impl<'d> Runner<'d>
{
    pub fn run_setup(& mut self, parser: Parser)
    {
        //TODO: Support function calls outside any scopes.

        // Register all functions.
        for func in &parser.global_expressions.internal_expressions
        {
            let scope_info = &func.1.scope_info;

            if let Some(func_name) = &scope_info.0
            {
                let closure = |args| self.call_internal(&func_name, args);

                self.functions.insert(func_name.clone(), &closure);
            }
        }
        
        // Register all variables.
        for var in &parser.global_expressions.single_op
        {
            let operation = run_operation(&var.1.1, &HashMap::new());

            if let Some(result) = operation
            {
                self.vars.insert(var.1.0.to_string(), result);
            }
        }

        // Attach parser.
        self.parser = parser;
    }

    pub fn call_function(&self, function_name: &str, args: Vec<Token>)
    {
        if self.functions.contains_key(function_name)
        {
            self.functions[function_name](args);
        }
    }

    // Called by a function pointer from a registered internal function.
    fn call_internal(&self, function_name: &str, arguments: Vec<Token>) -> Option<Token>
    {
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
            let mut scope_vars: HashMap<&String, &Token> = HashMap::new();

            let mut arg_vars: HashMap<String, Token> = HashMap::new();
            if let Some(expected_args) = &function.1.scope_info.1
            {
                let parsed_arg_names: Vec<&str> = expected_args.split(",").collect();

                for i in 0..parsed_arg_names.len()
                {
                    arg_vars.insert(parsed_arg_names[i].to_string(), arguments[i].clone());
                }
            }

            Runner::extend_ref_hash_map(&mut scope_vars, &arg_vars);
            Runner::extend_ref_hash_map(&mut scope_vars, &self.vars);

            return_result = self.handle_scope(function.1, &mut scope_vars);
        }

        return_result
    }

    fn extend_ref_hash_map<'a>(target_map: &mut HashMap<&'a String, &'a Token>, map: &'a HashMap<String, Token>)
    {
        for item in map.iter()
        {
            target_map.insert(item.0, item.1);
        }
    }

    fn extend_hash_map<'a>(target_map: &mut HashMap<&'a String, &'a Token>, map: &'a HashMap<&String, &Token>)
    {
        for item in map.iter()
        {
            target_map.insert(item.0, item.1);
        }
    }

    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    fn handle_scope(&self, function: &ExpressionList, parent_vars: &mut HashMap<&String, &Token>) -> Option<Token>
    {
        let mut return_result: Option<Token> = None;
        
        let mut local_vars: HashMap<String, Token> = HashMap::new();

        // Follow through and run every expression.
        for i in 0..function.size
        {
            match i
            {
                // Return, Assignement.
                _ if function.single_op.contains_key(&i) =>
                {
                    let expression = &function.single_op[&i];
                    let operation = run_operation(&expression.1, parent_vars);

                    match expression.0.as_str()
                    {
                        "return" =>
                        {
                            return_result = operation;
                        },
                        string =>
                        {
                            if let Some(result) = operation
                            {
                                local_vars.insert(string.to_string(), result);
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
                        if let Some(token) = run_operation(&tokens, parent_vars)
                        {
                            args.push(token);
                        }
                    }

                    self.call_function(&expression.0, args)
                },
                // Internal scope.
                _ if function.internal_expressions.contains_key(&i) =>
                {
                    //TODO: Add local vars to parent vars to pass through next scope.
                    let mut new_parent_vars = HashMap::new();

                    // Keep references to allow for altering the variables.
                    Runner::extend_hash_map(&mut new_parent_vars, parent_vars);
                    Runner::extend_ref_hash_map(&mut new_parent_vars, &local_vars);

                    return_result = self.handle_scope(&function.internal_expressions[&i], &mut new_parent_vars);
                },
                _ => ()
            }

            // If found a return statement, break out of the expression loop.
            if let None = return_result
            {
                break;
            }
        }

        return_result
    }

    // Called from a function within a language to access an external function.
    fn call_external(&self, function_name: &str, args: &str)
    {

    }
    
    pub fn regiser_external_function(&self, function_name: &str, function: fn())
    {

    }

    // pub fn get_variable(variable_name: &str) -> Token
    // {
    //     let found_global_var 
    // }
}