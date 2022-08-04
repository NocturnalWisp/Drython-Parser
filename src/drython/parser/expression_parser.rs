#[path="scope_parser.rs"]
mod scope_parser;

use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;

use crate::drython::types::Token;

use super::{utility::{self, ExpressionType}, operation_parser};
use super::types::ExpressionList;
use super::variable_parser::parse_var;
use scope_parser::parse_scope;

pub fn parse_expressions(expressions: &Vec<String>, line_start:usize, warning_list:&mut LinkedHashMap<usize, String>) -> ExpressionList
{
    let mut single_op: HashMap<usize, (String, Vec<Token>)> = HashMap::new();
    let mut multi_ops: HashMap<usize, (String, Vec<Vec<Token>>)> = HashMap::new();
    let mut internal_expressions: HashMap<usize, ExpressionList> = HashMap::new();
    let mut includes: Vec<String> = Vec::new();

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

        let expression_type = utility::get_expression_type(exp);

        if inside_scope
        {
            if utility::expression_is_scope(exp)
            {
                scope_count += 1;
            }

            let exp_lower = exp.to_lowercase();

            if exp_lower == "end" || exp_lower.starts_with("elif") || exp_lower.starts_with("elseif")  || exp_lower.starts_with("else")
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

                    if exp_lower.starts_with("elif") || exp_lower.starts_with("elseif") || exp_lower.starts_with("else")
                    {
                        scope_start = i;
                    }
                    else
                    {
                        inside_scope = false;
                    }

                    operation_index += 1;
                }
                else if exp_lower == "end"
                {
                    scope_count -= 1;
                }
            }
        }
        else
        {            
            // Scope change (if/loop).
            if utility::expression_is_scope(exp)
            {
                scope_start = i;
                inside_scope = true;
            }
            // Return operation.
            else if expression_type == ExpressionType::Return
            {
                match parse_return(exp)
                {
                    Ok(statement) => 
                    {
                        let operation = operation_parser::parse_operation(statement, warning_list, line_start+i);

                        single_op.insert(operation_index, ("return".to_string(), operation));
                        operation_index += 1;
                    }
                    Err(error) => {warning_list.insert(line_start+i, error);}
                }
            }
            // Variable assignment.
            else if expression_type == ExpressionType::Assignment
            {
                match parse_var(exp)
                {
                    Ok(result) => 
                    {
                        let operation = operation_parser::parse_operation(&result.1, warning_list, line_start+i);

                        single_op.insert(operation_index, (result.0, operation));
                        operation_index += 1;
                    },
                    Err(error) => {warning_list.insert(line_start, error);}
                }
            }
            // Function call.
            else if expression_type == ExpressionType::Call
            {
                match parse_call(exp)
                {
                    Ok(result) => 
                    {
                        let mut operations: Vec<Vec<Token>> = Vec::new();

                        for statement in result.1
                        {
                            operations.push(operation_parser::parse_operation(&statement, warning_list, line_start+i));
                        }

                        multi_ops.insert(operation_index, (result.0, operations));
                        operation_index += 1;
                    },
                    Err(error) => {warning_list.insert(line_start+i, error);}
                }
            }
            // loop control functions
            else if expression_type == ExpressionType::Break
            {
                single_op.insert(operation_index, ("break".to_string(), vec![]));
                operation_index += 1;
            }
            else if expression_type == ExpressionType::Continue
            {
                single_op.insert(operation_index, ("continue".to_string(), vec![]));
                operation_index += 1;
            }
            // Importing external functions
            else if expression_type == ExpressionType::Library
            {
                let library = exp.trim_start_matches(match exp
                {
                    _ if exp.starts_with("use") => "use",
                    _ if exp.starts_with("using") => "using",
                    _ if exp.starts_with("import") => "import",
                    _ if exp.starts_with("include") => "include",
                    _ => ""
                });
                
                includes.push(library.to_string());
            }
            else if expression_type == ExpressionType::None
            {
                warning_list.insert(line_start+i, "Could not parse the expression due to unrecognizable characters.".to_string());
            }
        }
    }

    ExpressionList
    {
        scope_info: (None, None),
        size: operation_index,
        single_op,
        multi_ops,
        internal_expressions,
        includes
    }
}

// Parses a function call into the name of the function, and the arguments.
fn parse_call(call: &str) -> Result<(String, Vec<String>), String>
{
    let function: String;
    let arguments: Vec<String>;
    
    if let Some(first) = call.split_once("(")
    {
        function = first.0.to_string();
        
        arguments = utility::split_by(&first.1[0..first.1.len()-1], ',');
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
