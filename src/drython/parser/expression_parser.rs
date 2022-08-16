#[path="scope_parser.rs"]
mod scope_parser;

use std::collections::HashMap;

use crate::drython::types::Token;

use super::{utility::{self, ExpressionType}, operation_parser};
use super::types::ExpressionList;
use super::variable_parser::parse_var;
use scope_parser::parse_scope;

use super::types::error::*;

pub fn parse_expressions(expressions: &Vec<String>, line_start:usize, error_manager: &mut ErrorManager, in_expression: &ExpressionType, in_function: bool) -> ExpressionList
{
    let mut single_op: HashMap<usize, (String, Vec<Token>, usize)> = HashMap::new();
    let mut multi_ops: HashMap<usize, (String, Vec<Vec<Token>>, usize)> = HashMap::new();
    let mut internal_expressions: HashMap<usize, (ExpressionList, usize)> = HashMap::new();
    let mut includes: HashMap<String, usize> = HashMap::new(); 

    // For internal expressions lists.
    // Will be split out for parsing once the end is found.
    let mut inside_scope = false;
    let mut scope_start = 0;
    let mut scope_count = 0;
    let mut scope_expression = in_expression.clone();

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

        let expression_type_result = utility::get_expression_type(exp);
        let expression_type: ExpressionType;

        match expression_type_result
        {
            Ok(result) =>
            {
                expression_type = result;
            },
            Err(message) =>
            {
                if in_function
                {
                    push_error!(error_manager, ParseError::new(line_start+i, message.as_str()));
                }
                continue;
            }
        }

        if inside_scope
        {
            if utility::expression_is_scope(&expression_type)
            {
                scope_count += 1;
            }

            if expression_type == ExpressionType::End || expression_type == ExpressionType::Elif || expression_type == ExpressionType::Else
            {
                if scope_count == 0
                {
                    let mut internal_expression = parse_expressions(
                        &expressions[scope_start+1..i].to_vec(),
                        scope_start+1+line_start,
                        error_manager,
                        &scope_expression,
                        true
                    );

                    match parse_scope(&expressions[scope_start].split_once(")").unwrap().1, &scope_expression)
                    {
                        Ok(result) => {internal_expression.scope_info = result;}
                        Err(error) => {push_error!(error_manager, ParseError::new(line_start+scope_start, error.as_str()));}
                    }

                    internal_expressions.insert(operation_index, (internal_expression, line_start+i));

                    if expression_type == ExpressionType::Elif || expression_type == ExpressionType::Else
                    {
                        scope_start = i;
                        scope_expression = expression_type.clone();
                    }
                    else
                    {
                        inside_scope = false;
                        scope_expression = ExpressionType::None;
                    }

                    operation_index += 1;
                }
                else if expression_type == ExpressionType::End
                {
                    scope_count -= 1;

                    // Handle error when reached end of expression but no 'end' in sight.
                    // But still within a scope.
                    if i == expressions.len()-1
                    {
                        push_error!(error_manager, 
                            ParseError::new(line_start+i,
                                format!("Scope starting at '{}' was not closed with an 'end' statement.", &expressions[scope_start].split_once(")").unwrap().1).as_str()));
                    }
                }
            }
            // Handle error when reached end of expression but no 'end' in sight.
            else if i == expressions.len()-1
            {
                push_error!(error_manager, 
                    ParseError::new(line_start+i,
                        format!("Scope starting at '{}' was not closed with an 'end' statement.", &expressions[scope_start].split_once(")").unwrap().1).as_str()));
            }
        }
        else
        {            
            // Scope change (if/loop).
            if utility::expression_is_scope(&expression_type)
            {
                scope_start = i;
                scope_expression = expression_type.clone();
                inside_scope = true;

                if i == expressions.len()-1
                {
                    push_error!(error_manager, 
                        ParseError::new(line_start+i,
                            format!("Scope starting at '{}' was not closed with an 'end' statement.", &expressions[scope_start].split_once(")").unwrap().1).as_str()));
                }
            }
            // Return operation.
            else if expression_type == ExpressionType::Return
            {
                if in_function
                {
                    match parse_return(exp)
                    {
                        Ok(statement) => 
                        {
                            let operation = operation_parser::parse_operation(statement, error_manager, line_start+i);

                            single_op.insert(operation_index, ("return".to_string(), operation, line_start+i));
                            operation_index += 1;
                        }
                        Err(error) => {push_error!(error_manager, ParseError::new(line_start+i, error.as_str()));}
                    }
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i, "Return statement unexpected outside function definition."));
                }
            }
            // Variable assignment.
            else if expression_type == ExpressionType::Assignment
            {
                match parse_var(exp)
                {
                    Ok(result) => 
                    {
                        let operation = operation_parser::parse_operation(&result.1, error_manager, line_start+i);

                        single_op.insert(operation_index, (result.0, operation, line_start+i));
                        operation_index += 1;
                    },
                    Err(error) => {push_error!(error_manager, ParseError::new(line_start+i, error.as_str()));}
                }
            }
            // Function call.
            else if expression_type == ExpressionType::Call
            {
                if in_function
                {
                    match parse_call(exp)
                    {
                        Ok(result) => 
                        {
                            let mut operations: Vec<Vec<Token>> = Vec::new();

                            for statement in result.1
                            {
                                operations.push(operation_parser::parse_operation(&statement, error_manager, line_start+i));
                            }

                            multi_ops.insert(operation_index, (result.0, operations, line_start+i));
                            operation_index += 1;
                        },
                        Err(error) => {push_error!(error_manager, ParseError::new(line_start+i, error.as_str()));}
                    }
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i, "Unexpected function call in script. Did you mean to call it inside a function?"));
                }
            }
            // loop control functions
            else if expression_type == ExpressionType::Break
            {
                if let ExpressionType::Loop = scope_expression
                {
                    single_op.insert(operation_index, ("break".to_string(), vec![], line_start+i));
                    operation_index += 1;
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i, "Break statement used outside loop."));
                }
            }
            else if expression_type == ExpressionType::Continue
            {
                if let ExpressionType::Loop = scope_expression
                {
                    single_op.insert(operation_index, ("continue".to_string(), vec![], line_start+i));
                    operation_index += 1;
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i, "Continue statement used outside loop."));
                }
            }
            // Importing external functions
            else if expression_type == ExpressionType::Library
            {
                if !in_function
                {
                    let library = exp.trim_start_matches(match exp
                    {
                        _ if exp.starts_with("use") => "use",
                        _ if exp.starts_with("using") => "using",
                        _ if exp.starts_with("import") => "import",
                        _ if exp.starts_with("include") => "include",
                        _ => ""
                    });
                    
                    includes.insert(library.to_string(), line_start+i);
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i, "Library includes are not allowed within a scope."));
                }
            }
            else if expression_type == ExpressionType::End
            {
                push_error!(error_manager, ParseError::new(line_start+i, "Too many 'end' statements are present. Are you missing a ':' after a function decleration?"));
            }

        }
    }

    ExpressionList
    {
        scope_info: (None, None),
        size: operation_index,
        line_start,
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
        if first.1.len() > 0 && first.1.as_bytes()[first.1.len()-1] == b')'
        {
            function = first.0.to_string();
            
            match utility::split_by(&first.1[0..first.1.len()-1], ',')
            {
                Ok(result) =>
                {
                    arguments = result;
                }
                Err(error) =>
                {
                    return Err(error);
                }
            }
        }
        else
        {
            return Err("No closing parenthesis found on function call.".to_string());
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
