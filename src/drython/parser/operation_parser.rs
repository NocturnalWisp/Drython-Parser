use std::{error, option};

use crate::drython::{types::{Operation, OperationValue, OperationOption}, utility::{self, operations, operation_order}};

pub fn parse_operation<'a, 'b>(string: &'a str, error_list: &'a mut Vec<&'b str>) -> Option<Operation<'a>>
{
    parse_individual(string, error_list)
}

fn parse_individual<'a, 'b>(string: &'a str, error_list: &mut Vec<&'b str>) -> Option<Operation<'a>>
{
    let mut in_parenth = false;
    
    let mut parenth_start: usize = 0;
    let mut inner_parenth_count: usize = 0;

    let mut in_value = false;
    let mut value_start: usize = 0;
    let mut value_end: usize = 0;

    // optional char is for the char operation leading to the next option.
    let mut options_main: Vec<(Option<char>, Option<OperationOption>)> = vec![];
    let mut options = &mut options_main;

    let mut just_found_operation = false;

    // Resurive check for parenthesis
    for (i, c) in string.chars().enumerate()
    {
        if in_parenth
        {
            if c == '('
            {
                inner_parenth_count += 1;
            }

            if c == ')'
            {
                // Closing operation/function call.
                if inner_parenth_count == 0
                {
                    in_parenth = false;
                    inner_parenth_count = 0;

                    // Function call.
                    if in_value
                    {
                        options.push((None, Some(OperationOption::Value(OperationValue::Call(&string[value_start..value_end], &string[parenth_start..i])))));
                    }
                    // Internal operation.
                    else
                    {
                        if let Some(internal) = parse_individual(&string[parenth_start..i], error_list)
                        {
                            options.push((None, Some(OperationOption::Op(Box::new(internal)))));
                        }
                    }
                }
                else
                {
                    inner_parenth_count -= 1;
                }
            }
        }
        else
        {
            // Handle parenthises operation parsing.
            if c == '('
            {
                in_parenth = true;
                parenth_start = i+1;

                // If was in value and didn't find operation, this is likely a function call.
                if in_value
                {
                    value_end = i;
                }
                just_found_operation = false;
            }
            // Handle value parsing.
            else if c.is_alphanumeric()
            {
                if !in_value
                {
                    in_value = true;
                    value_start = i;
                }
                // Check for end of value.
                else
                {
                    if in_value && i == string.len()-1
                    {
                        parse_op_value(&string[value_start..i-1], options);
                    }
                }
                just_found_operation = true;
            }
            // Handle operation value parsing.
            else if utility::operations.contains(&c)
            {
                // Handle previous value grabbing
                let value = &string[value_start..i-1];

                parse_op_value(value, options);

                // Handle operation.
                if just_found_operation
                {
                    if let Some(last) = options.last()
                    {
                        if let None = last.0
                        {
                            last.0 = Some(c);
                        }
                        // if the tip of the options all ready has a char;
                        // There was a second operator placed here for some reason.
                        else
                        {
                            error_list.push("Operation parse error.\nA second operator was included without a value between.");
                        }
                    }
                }
                else
                {
                    error_list.push("Operation parse error.\nPlease have a left and right side to the operation.");
                }
            }
        }
    }

    // If still in parenthises, there is an error.
    if in_parenth
    {
        error_list.push("Failed to parse operation.\nClosing brace not found.");
    }
    
    // Handle operation order and populating.
    handle_populating_operation(options)
    
}

fn parse_op_value<'a>(value: &'a str, options: &mut Vec<(Option<char>, Option<OperationOption<'a>>)>)
{
    if let Ok(result) = value.parse::<i32>()
    {
        options.push((None, Some(OperationOption::Value(OperationValue::Int(result)))));
    }
    else if let Ok(result) = value.parse::<f32>()
    {
        options.push((None, Some(OperationOption::Value(OperationValue::Float(result)))));
    }
    else if let Ok(result) = value.parse::<bool>()
    {
        options.push((None, Some(OperationOption::Value(OperationValue::Bool(result)))));
    }
    else
    {
        options.push((None, Some(OperationOption::Value(OperationValue::Var(value)))));
    }
}

// Handles creating the various types of operations to be merged into one big operation later.
fn handle_populating_operation<'a>(operation_options: &'a Vec<(Option<char>, Option<OperationOption<'a>>)>) -> Option<Operation<'a>>
{
    let mut i = 0;

    let mut result_operations: Vec<Option<Operation<'a>>> = vec![];

    let mut op1 = &operation_options[i];
    let mut op2 = &operation_options[i+1];
    let ops = (op1, op2);

    let mut handled: Vec<Operation> = vec![];

    // Loop that handles operations by pairs.
    while i+1 < operation_options.len()
    {
        // Create an operation based on the two sides of the operation_option looped through.
        match ops
        {
            ((_, OperationOption::Op(_)), (_, OperationOption::Op(_))) => 
                if let Some(op) = ops.0.0 {result_operations.push(Some(Operation::new(op, ops.0.1, ops.1.1, ops.1.0)))},
            ((_, OperationOption::Value(_)), (_, OperationOption::Value(_))) => 
                if let Some(op) = ops.0.0 { result_operations.push(Some(Operation::new(op, ops.0.1, ops.1.1, ops.1.0)))},
            ((_, OperationOption::Value(_)), (_, OperationOption::Op(_))) => 
                if let Some(op) = ops.0.0 { result_operations.push(Some(Operation::new(op, ops.0.1, ops.1.1, ops.1.0)))},
            ((_, OperationOption::Op(_)), (_, OperationOption::Value(_))) => 
                if let Some(op) = ops.0.0 { result_operations.push(Some(Operation::new(op, ops.0.1, ops.1.1, ops.1.0)))},
        }

        i += 1;
    }

    if handled.len() == 1
    {
        return Some(handled[0]);
    }
    else if handled.len() == 0
    {
        return None;
    }
    // Create operation options based on the compiled operations,
    // based on operation (Prioritize BEDMAS)
    handle_populating_operation(handle_combining_operations(&handled))
}

// Orders 
fn handle_combining_operations<'a>(handled: &'a Vec<Operation<'a>>) -> &'a Vec<(Option<char>, Option<OperationOption<'a>>)>
{
    let mut combination_result: Vec<(Option<char>, Option<OperationOption<'a>>)> = vec![];

    for i in 0..handled.len()-1
    {
        combination_result.push((None, None));
    }

    for ops in operation_order
    {
        let mut i: usize = 0;
        while i+1 < handled.len()
        {
            if let Some(next_op) = handled[i].next_op
            {
                if ops.contains(next_op)
                {
                    combination_result[i] = (handled[0].next_op, Some(OperationOption::Op(Box::new(handled[0]))))
                }
            }

            i += 1;
        }
    }

    &combination_result
}

impl<'a> Operation<'a>
{
    pub fn new(op: char, a: OperationOption<'a>, b: OperationOption<'a>, next_op: Option<char>) -> Operation<'a>
    {
        Operation
        {
            a,
            b,
            op,
            next_op
        }
    }
}