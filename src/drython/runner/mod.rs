use std::{str::FromStr, collections::HashMap, hash::Hash};

use crate::Parser;

use super::types::{Runner, Token, ExpressionList};

impl<'a> Runner<'a>
{
    pub fn run_setup()
    {

    }

    pub fn call_function(&self, function_name: &str, args: &str)
    {
        if self.functions.contains_key(function_name)
        {
            self.functions[function_name](args);
        }
    }

    fn call_internal<T: FromStr>(&self, function_name: &str, arguments: Option<&str>) -> Option<T>
    {
        // Try and find if the function exists
        let found_func = self.parser.global_expressions.internal_expressions.iter()
            .find(|x| 
                if let Some(name) = &x.1.scope_info.0
                {
                    name == function_name
                } else {false}
            );

        let mut return_result: Option<T> = None;

        if let Some(function) = found_func
        {
            return_result = Runner::handle_scope(function.1);
        }

        return_result
    }

    // Resursive function to handle calling order in internal scopes.
    // Originally called by call_internal for a parsed function.
    fn handle_scope<T>(function: &ExpressionList) -> Option<T>
    {
        let mut return_result: Option<T> = None;
        
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
                },
                // Function call.
                _ if function.multi_ops.contains_key(&i) =>
                {

                },
                // Internal scope.
                _ if function.internal_expressions.contains_key(&i) =>
                {
                    return_result = Runner::handle_scope(&function.internal_expressions[&i]);
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
}