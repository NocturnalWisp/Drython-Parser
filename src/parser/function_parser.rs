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

    if expressions.len() == 0
    {
        return Err(("No function body.".to_string(), 1));
    }

    let parsed_expressions =
        parse_expressions(&expressions, 0, None, error_list);

    Ok(Func {name: name.to_string(), size: lines.len()-1, parameters: params, expressions: parsed_expressions})
}

fn parse_expressions(expressions: &Vec<String>, line_start:usize, if_expression:Option<String>, error_list:&mut LinkedHashMap<usize, String>) -> ExpressionList
{
    let mut vars: HashMap<usize, (String, String)> = HashMap::new();
    let mut calls: HashMap<usize, (String, Vec<String>)> = HashMap::new();
    let mut internal_expressions: HashMap<usize, ExpressionList> = HashMap::new();

    let mut inside_if = false;
    let mut if_start = 0;
    let mut if_internal_count = 0;

    // The actual order index that each expression will use.
    // (i in the enumerate is unrealiable with nested blocks.)
    let mut operation_index = 0;

    for (i, exp) in expressions.iter().enumerate()
    {
        if inside_if
        {
            if utility::check_for_if(exp)
            {
                if_internal_count += 1;
            }

            if exp.to_lowercase() == "end"
            {
                if if_internal_count == 0
                {
                    let if_str = expressions[if_start].clone();
                    let if_expression = if_str
                        .trim_start_matches("if").trim_end_matches(":");
                    internal_expressions.insert(operation_index, 
                        parse_expressions(
                            &expressions[if_start+1..i].to_vec(),
                            if_start+1+line_start,
                            Some(if_expression.to_string()),
                            error_list
                        )
                    );
                    inside_if = false;
                    operation_index += 1;
                }
                else
                {
                    if_internal_count -= 1;
                }
            }
        }
        else
        {
            // Conditional (If).
            if utility::check_for_if(exp)
            {
                if_start = i;
                inside_if = true;
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

    ExpressionList { variables: vars, calls, internal_expressions, if_expression }
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