use std::collections::{HashMap, VecDeque};
use linked_hash_map::LinkedHashMap;

use crate::drython::types::Token;

use super::utility;
use super::types::ExpressionList;
use super::variable_parser::parse_var;
use scope_parser::parse_scope;

// Parses various types of expressions (if, return, function, variable assignment)
pub fn parse_expressions<'a>(expressions: &'a Vec<String>, line_start:usize, warning_list:&mut LinkedHashMap<usize, String>) -> ExpressionList<'a>
{
    let mut vars: HashMap<usize, (String, VecDeque<Token<'a>>)> = HashMap::new();
    let mut calls: HashMap<usize, (String, VecDeque<Token<'a>>)> = HashMap::new();
    let mut internal_expressions: HashMap<usize, ExpressionList> = HashMap::new();

    // For internal expressions lists.
    // Will be split out for parsing once the end is found.
    let mut inside_scope = false;
    let mut scope_start = 0;
    let mut scope_count = 0;

    // The actual order index that each expression will use.
    // (i in the enumerate is unrealiable with nested blocks.)
    let mut operation_index = 0;
    
    for (i, expression) in expressions.iter().enumerate()
    {
        if expression.len() <= 2 {continue;}

        let exp;

        if let Some(result) = expression.split_once(")")
        {
            exp = result.1;
        }
        else {continue;}

        if inside_scope
        {
            if utility::check_for_scope(exp)
            {
                scope_count += 1;
            }

            if exp.to_lowercase() == "end"
            {
                if scope_count == 0
                {
                    let mut internal_expression = parse_expressions(
                        &expressions[scope_start+1..i].to_vec(),
                        scope_start+1+line_start,
                        warning_list
                    );

                    match parse_scope(&expressions[scope_start])
                    {
                        Ok(result) => {internal_expression.scope_info = result;}
                        Err(error) => {warning_list.insert(line_start, error);}
                    }

                    internal_expressions.insert(operation_index, internal_expression);

                    inside_scope = false;
                    operation_index += 1;
                }
                else
                {
                    scope_count -= 1;
                }
            }
        }
        else
        {
            // Scope change (if/loop).
            if utility::check_for_scope(exp)
            {
                scope_start = i;
                inside_scope = true;
            }
            // Return operation.
            else if utility::check_for_return(exp)
            {
                match parse_return(exp)
                {
                    Ok(result) => 
                    {
                        calls.insert(operation_index, ("return".to_string(), result));
                        operation_index += 1;
                    }
                    Err(error) => {warning_list.insert(line_start+i, error);}
                }
            }
            // Function call.
            else if utility::check_for_call(exp)
            {
                match parse_call(exp)
                {
                    Ok(result) => 
                    {
                        calls.insert(operation_index, (result.0, result.1));
                        operation_index += 1;
                    },
                    Err(error) => {warning_list.insert(line_start+i, error);}
                }
            }
            // Variable assignment.
            else if exp.contains("=")
            {
                match parse_var(exp)
                {
                    Ok(result) => 
                    {
                        vars.insert(operation_index, (result.0, result.1));
                        operation_index += 1;
                    },
                    Err(error) => {warning_list.insert(line_start+i, error);}
                }
            }
        }
    }

    ExpressionList
    {
        scope_info: (None, None),
        size: operation_index,
        variables: vars,
        calls,
        internal_expressions,
    }
}

// Parses a function call into the name of the function, and the arguments.
fn parse_call<'a>(call: &'a str) -> Result<(String, VecDeque<Token<'a>>), String>
{
    let function: String;
    let arguments: Vec<String>;
    
    if let Some(first) = call.split_once("(")
    {
        function = first.0.to_string();
        
        if let Some(second) = first.1.split_once(")")
        {
            arguments = second.0.split(",").map(|x|x.to_string()).collect()
        }
        else
        {
            return Err("Failed to parse call.\nInvalid function arguments.".to_string());
        }
    }
    else
    {
        return Err("Failed to parse call.\nInvalid function.".to_string());
    }

    Ok((function, arguments))
}

fn parse_return<'a>(statement: &'a str) -> Result<&str, VecDeque<Token<'a>>>
{
    let expression: &str;
    
    expression = statement.trim_start_matches("return");

    Ok(expression)
}