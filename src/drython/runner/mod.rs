pub mod operation_runner;

use std::{str::FromStr, collections::HashMap, hash::Hash};

use crate::Parser;

use self::operation_runner::run_operation;

use super::types::{Runner, Token, ExpressionList};

impl<'a> Runner<'a>
{
    pub fn run_setup()
    {

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
            let mut local_vars: HashMap<String, Token> = HashMap::new();

            let mut arg_vars: HashMap<String, Token> = HashMap::new();
            if let Some(expected_args) = &function.1.scope_info.1
            {
                let parsed_arg_names: Vec<&str> = expected_args.split(",").collect();

                for i in 0..parsed_arg_names.len()
                {
                    arg_vars.insert(parsed_arg_names[i].to_string(), arguments[i].clone());
                }
            }

            return_result = self.handle_scope(function.1, local_vars);
        }

        return_result
    }

    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    fn handle_scope(&self, function: &ExpressionList, parent_vars: &HashMap<String, Token>) -> Option<Token>
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
                    let mut arguments: Vec<Token> = vec![];

                    for tokens in expression.1.iter()
                    {
                        if let Some(token) = run_operation(&tokens, parent_vars)
                        {
                            arguments.push(token);
                        }
                    }

                    self.call_function(&expression.0, arguments)
                },
                // Internal scope.
                _ if function.internal_expressions.contains_key(&i) =>
                {
                    return_result = self.handle_scope(&function.internal_expressions[&i], parent_vars);
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

    fn convert_hashmap_reference() -> HashMap<&String, &Token>
    {
        
    }
}