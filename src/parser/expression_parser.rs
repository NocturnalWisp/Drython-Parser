#[path="scope_parser.rs"]
mod scope_parser;
#[path="call_parser.rs"]
mod call_parser;

use std::collections::HashMap;

use crate::utility;
use crate::types::{Token, ExpressionList, error::*, ExpressionListType, SingleOp, Internal, MultiOp};

use super::variable_parser::parse_var;
use super::{operation_parser, ExpressionType};
use scope_parser::parse_scope;

pub fn parse_expressions(expressions: &Vec<String>, line_start:usize, error_manager: &mut ErrorManager, in_expression: &ExpressionType, in_function: bool) -> ExpressionList
{
    let mut expression_order: Vec<ExpressionListType> = Vec::new();
    let mut order_pushed_flag = false;

    let mut single_op: Vec<SingleOp> = Vec::new();
    let mut multi_ops: Vec<MultiOp> = Vec::new();
    let mut internal_expressions: Vec<Internal> = Vec::new();
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
        let exp;

        if let Some(result) = expression.split_once(")")
        {
            exp = result.1;
        }
        else {continue;}

        let expression_type_result = get_expression_type(exp);
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
                    push_error!(error_manager, ParseError::new(line_start+i+1, message.as_str()));
                }
                continue;
            }
        }

        if inside_scope
        {
            order_pushed_flag = true;

            if expression_type.is_scope()
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

                    internal_expressions.push((internal_expression, line_start+scope_start+1));

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
                            ParseError::new(line_start+scope_start+1,
                                format!("Scope starting at '{}' was not closed with an 'end' statement internally.", &expressions[i].split_once(")").unwrap().1).as_str()));
                    }
                }
            }
            // Handle error when reached end of expression but no 'end' in sight.
            else if i == expressions.len()-1
            {
                push_error!(error_manager, 
                    ParseError::new(line_start+i,
                        format!("Scope starting at '{}' was not closed with an 'end' statement.", &expressions[line_start+scope_start].split_once(")").unwrap().1).as_str()));
            }
        }
        else
        {            
            // Scope change (if/loop).
            if expression_type.is_scope()
            {
                scope_start = i;
                scope_expression = expression_type.clone();
                inside_scope = true;

                expression_order.push(ExpressionListType::Internal);
                order_pushed_flag = true;

                if i == expressions.len()-1
                {
                    push_error!(error_manager, 
                        ParseError::new(line_start+i+1,
                            format!("Scope starting at '{}' was not closed with an 'end' statement.", &expressions[line_start+scope_start].split_once(")").unwrap().1).as_str()));
                }
            }
            // Return operation.
            else if expression_type == ExpressionType::Return
            {
                if in_function
                {
                    match operation_parser::parse_operation(exp.trim_start_matches("return"))
                    {
                        Ok(operation) =>
                        {
                            single_op.push(("return".to_string(), vec!(), operation, line_start+i));
                            expression_order.push(ExpressionListType::Single);
                            order_pushed_flag = true;
                            operation_index += 1;
                        }
                        Err(error) =>
                        {
                            parse_error!(error_manager, line_start+i+1, error.as_str());
                        }
                    }
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i+1, "Return statement unexpected outside function definition."));
                }
            }
            // Variable assignment.
            else if expression_type == ExpressionType::Assignment
            {
                match parse_var(exp)
                {
                    Ok(result) => 
                    {
                        match operation_parser::parse_operation(&result.2)
                        {
                            Ok(operation) =>
                            {
                                single_op.push((result.1, result.0, operation, line_start+i+1));
                                expression_order.push(ExpressionListType::Single);
                                order_pushed_flag = true;
                                operation_index += 1;
                            }
                            Err(error) => {push_error!(error_manager, ParseError::new(line_start+i+1, error.as_str()));}
                        }
                    },
                    Err(error) => {push_error!(error_manager, ParseError::new(line_start+i+1, error.as_str()));}
                }
            }
            // Function call.
            else if expression_type == ExpressionType::Call
            {
                if in_function
                {
                    match call_parser::parse_call(exp)
                    {
                        Ok(result) => 
                        {
                            let mut operations: Vec<Vec<Token>> = Vec::new();

                            for statement in result.1
                            {
                                match operation_parser::parse_operation(&statement)
                                {
                                    Ok(operation) => operations.push(operation),
                                    Err(error) => {push_error!(error_manager, ParseError::new(line_start+i+1, error.as_str()));}
                                }
                            }

                            multi_ops.push((result.0, operations, line_start+i+1));
                            expression_order.push(ExpressionListType::Multi);
                            order_pushed_flag = true;

                            operation_index += 1;
                        },
                        Err(error) => {push_error!(error_manager, ParseError::new(line_start+i+1, error.as_str()));}
                    }
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i+1, "Unexpected function call in script. Did you mean to call it inside a function?"));
                }
            }
            // loop control functions
            else if expression_type == ExpressionType::Break
            {
                if let ExpressionType::Loop = scope_expression
                {
                    single_op.push(("break".to_string(), vec![], vec![], line_start+i+1));
                    expression_order.push(ExpressionListType::Single);
                    order_pushed_flag = true;

                    operation_index += 1;
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i+1, "Break statement used outside loop."));
                }
            }
            else if expression_type == ExpressionType::Continue
            {
                if let ExpressionType::Loop = scope_expression
                {
                    single_op.push(("continue".to_string(), vec![], vec![], line_start+i+1));
                    expression_order.push(ExpressionListType::Single);
                    order_pushed_flag = true;

                    operation_index += 1;
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i+1, "Continue statement used outside loop."));
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
                    
                    includes.insert(library.to_string(), line_start+i+1);
                    expression_order.push(ExpressionListType::Library);
                    order_pushed_flag = true;
                }
                else
                {
                    push_error!(error_manager, ParseError::new(line_start+i+1, "Library includes are not allowed within a scope."));
                }
            }
            else if expression_type == ExpressionType::End
            {
                push_error!(error_manager, ParseError::new(line_start+i+1, "Too many 'end' statements are present. Are you missing a ':' after a function decleration?"));
                order_pushed_flag = true;
            }
        }

        // Handle expression order flagging for empty lines.
        if order_pushed_flag { order_pushed_flag = false; }
        else { expression_order.push(ExpressionListType::Null); }
    }

    ExpressionList
    {
        scope_info: (None, None),
        size: operation_index,
        line_start,

        expression_order,
        single_op,
        multi_ops,
        internal_expressions,
        includes
    }
}

pub fn get_expression_type(string: &str) -> Result<ExpressionType, String>
{
    let mut buffer = String::new();
    let mut started_call_or_function = false;

    if string.is_empty()
    {
        return Ok(ExpressionType::None);
    }

    match string.to_lowercase().as_str()
    {
        "break" => {return Ok(ExpressionType::Break);}
        "continue" => {return Ok(ExpressionType::Continue);}
        "end" => {return Ok(ExpressionType::End);}
        _ => ()
    };

    // Comments
    if string.starts_with("//") || string.starts_with("#")
    {
        return Ok(ExpressionType::Comment);
    }

    let mut peekable = string.chars().peekable();
    while let Some(c) = peekable.next()
    {
        if !started_call_or_function
        {
            match c
            {
                c if c.is_alphanumeric() || c == '.' || c == '_' || c == '!' => buffer.push(c),
                '=' => return Ok(ExpressionType::Assignment),
                '(' => started_call_or_function = true,
                c if utility::operations_contains(c) => (),
                '"' | '\'' | ')' => (),
                _ => return Err(format!("Failed to recognize character: '{}'", c))
            }
        }
        // Check for the end of a function creation.
        else
        {
            if let ':' = c
            {
                return Ok(ExpressionType::Function);
            }
        }

        // Buffer check for the start of an expression type.
        // Later parsing will check for errors in missing colons or arguments.
        match buffer.to_lowercase().as_str()
        {
            "loop" => return Ok(ExpressionType::Loop),
            "if" => return Ok(ExpressionType::If),
            "elif"|"elseif" => return Ok(ExpressionType::Elif),
            "else" => {if let Some('i') = peekable.peek() {} else { return Ok(ExpressionType::Else)}},
            "return" => return Ok(ExpressionType::Return),
            "use"|"import"|"include"|"using" => return Ok(ExpressionType::Library),
            _ => ()
        }
    }

    // Found a call but not a function.
    // (Char search closed out before finding a colon.)
    if started_call_or_function
    {
        return Ok(ExpressionType::Call);
    }

    // check for additional ++ or --
    if string.ends_with("++") || string.ends_with("--")
    {
        return Ok(ExpressionType::Assignment);
    }

    Err("Unkown expression.".to_string())
}
