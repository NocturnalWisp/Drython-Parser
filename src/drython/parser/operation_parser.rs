use std::{error, option, collections::VecDeque};

use crate::drython::{types::Token};

use crate::drython::utility;

pub fn parse_operation<'a, 'b>(string: &'a str, error_list: &mut Vec<&'b str>) -> VecDeque<Token<'a>>
{
    let mut in_parenth = false;
    
    let mut parenth_start: usize = 0;
    let mut inner_parenth_count: usize = 0;

    let mut in_value = false;
    let mut value_start: usize = 0;
    let mut value_end: usize = 0;

    // optional char is for the char operation leading to the next option.
    let mut tokens: Vec<Token<'a>> = vec![];
    let mut tokens_ptr = &mut tokens;

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
                        tokens_ptr.push(Token::Call(&string[value_start..value_end], &string[parenth_start..i-1]));
                    }
                    // Internal operation.
                    else
                    {
                        tokens_ptr.push(Token::Operation(parse_operation(&string[parenth_start..i-1], error_list)));
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
                        parse_op_value(&string[value_start..i-1], tokens_ptr);
                    }
                }
                just_found_operation = true;
            }
            // Handle operation value parsing.
            else if utility::operations.contains(&c)
            {
                // Handle previous value grabbing
                let value = &string[value_start..i-1];

                parse_op_value(value, tokens_ptr);

                // Handle operation.
                if just_found_operation
                {
                    tokens_ptr.push(Token::Operator(c));
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
    handle_populating_operation(tokens)
    
}

fn parse_op_value<'a>(value: &'a str, options: &mut Vec<Token<'a>>)
{
    if let Ok(result) = value.parse::<i32>()
    {
        options.push(Token::Int(result));
    }
    else if let Ok(result) = value.parse::<f32>()
    {
        options.push(Token::Float(result));
    }
    else if let Ok(result) = value.parse::<bool>()
    {
        options.push(Token::Bool(result));
    }
    else
    {
        options.push(Token::Var(value));
    }
}

// Handles creating the various types of operations to be merged into one big operation later.
// Uses the shunting algorithm based on the reverse polish technique.
// (Doesn't deal with brackets because Operations all ready nest themselves.)
// Followed: https://brilliant.org/wiki/shunting-yard-algorithm/
fn handle_populating_operation<'a>(tokens: Vec<Token<'a>>) -> VecDeque<Token<'a>>
{
    let mut stack: Vec<Token<'a>> = vec![];
    let mut queue: VecDeque<Token<'a>> = VecDeque::new();

    for token in tokens
    {
        match token
        {
            Token::Operator(op) => 
            {
                let mut stack_last = skip_fail_result!(stack.last());
                let mut stack_op = *skip_fail_operator!(stack_last);

                while utility::operator_a_gt_b(stack_op, op)
                {
                    if let Some(result) = stack.pop()
                    {
                        queue.push_back(result);
                    }
                    
                    stack_last = skip_fail_result!(stack.last());
                    stack_op = *skip_fail_operator!(stack_last);
                }
                stack.push(token);
            },
            _ =>
            {
                queue.push_back(token);
            }
        }
    }

    while !stack.is_empty()
    {
        if let Some(result) = stack.pop()
        {
            queue.push_back(result);
        }
    }
    
    queue
}
