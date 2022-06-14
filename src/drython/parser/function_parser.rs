use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

use super::utility;
use super::types::ExpressionList;
use super::types::Func;
use super::variable_parser::parse_var;

// Function stored format: Name = (Parameters, Expressions)
// Error format (message, local line, character)
pub fn parse_func(lines: &Vec<&str>, error_list: &mut LinkedHashMap<usize, String>) -> Result<Func, (String, usize)>
{
    if lines.len() == 0
    {
        return Err(("Failed to parse function.".to_string(), 0));
    }
    
    let init: Vec<&str> = lines[0].split("(").collect();
    let name = init[0];
    let params: Vec<String> = if let Some(p) = init[1].split_once(")")
    {
        // Seperate params if there are any.
        p.0.split(",").into_iter().map(|x|x.to_string()).collect()
    }
    else { return Err(("Failed to parse function parameters.".to_string(), 0)); };

    let expressions: Vec<String> = lines.split_at(1).1.iter().map(|x|x.to_string()).collect();

    let parsed_expressions =
        parse_expressions(&expressions, 0, None, error_list);

    Ok(Func {name: name.to_string(), size: lines.len()-1, parameters: params, expressions: parsed_expressions})
}

fn parse_expressions(expressions: &Vec<String>, line_start:usize, if_expression:Option<String>, error_list:&mut LinkedHashMap<usize, String>) -> ExpressionList
{
    let mut vars: HashMap<usize, (String, String)> = HashMap::new();
    let mut calls: HashMap<usize, (String, Vec<String>)> = HashMap::new();
    let mut internal_expressions: HashMap<usize, ExpressionList> = HashMap::new();

    // For internal expressions lists.
    // Will be split out for parsing once the end is found.
    let mut inside_scope = false;
    let mut scope_start = 0;
    let mut scope_count = 0;

    // The actual order index that each expression will use.
    // (i in the enumerate is unrealiable with nested blocks.)
    let mut operation_index = 0;

    for (i, exp) in expressions.iter().enumerate()
    {
        if inside_scope
        {
            if utility::check_for_scope(exp)
            {
                scope_count += 1;
            }

            println!("{}", exp.to_lowercase());
            if exp.to_lowercase() == "end"
            {
                if scope_count == 0
                {
                    let scope_str = expressions[scope_start].clone();
                    let scope_expression = scope_str.trim_end_matches(":");
                    internal_expressions.insert(operation_index, 
                        parse_expressions(
                            &expressions[scope_start+1..i].to_vec(),
                            scope_start+1+line_start,
                            Some(scope_expression.to_string()),
                            error_list
                        )
                    );
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
                        calls.insert(operation_index, ("return".to_string(), vec![result.to_string()]));
                        operation_index += 1;
                    }
                    Err(error) => {error_list.insert(line_start+i, error);}
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
                    Err(error) => {error_list.insert(line_start+i, error);}
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
                    Err(error) => {error_list.insert(line_start+i, error);}
                }
            }
        }
    }

    ExpressionList { variables: vars, calls, internal_expressions, scope_expression: if_expression, size: expressions.len() }
}

// Parses a function call into the name of the function, and the arguments.
fn parse_call(call: &str) -> Result<(String, Vec<String>), String>
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

fn parse_return(statement: &str) -> Result<&str, String>
{
    let expression: &str;
    
    expression = statement.trim_start_matches("return");

    Ok(expression)
}