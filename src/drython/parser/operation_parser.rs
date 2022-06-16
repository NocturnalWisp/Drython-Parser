use std::{collections::VecDeque};

use crate::drython::types::Token;

use crate::drython::utility;

// Hybrid polish notation/ast tree. Internal operations (Expressed in parenthesis)
// are put into a recursive calculation.
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
    let tokens_ptr = &mut tokens;

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
                        tokens_ptr.push(Token::Call(&string[value_start..value_end], &string[parenth_start..i]));
                        in_value = false;
                    }
                    // Internal operation.
                    else
                    {
                        tokens_ptr.push(Token::Operation(parse_operation(&string[parenth_start..i], error_list)));
                    }

                    // In case there is an operation just after this inner parenthesis.
                    just_found_operation = true;
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
                just_found_operation = true;

                // This might be the last value.
                if i == string.len()-1
                {
                    parse_op_value(&string[value_start..i+1], tokens_ptr);
                    in_value = false;
                }
            }
            // Handle operation value parsing.
            else if utility::OPERATIONS.contains(&c)
            {
                // Handle previous value grabbing
                if in_value
                {
                    parse_op_value(&string[value_start..i], tokens_ptr);
                    in_value = false;
                }

                // Handle operation.
                if just_found_operation
                {
                    tokens_ptr.push(Token::Operator(c));
                    just_found_operation = false;
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

// Allows for the conversion from string to different types.
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
    // Parsed tokens that don't fit another type are considered variables.
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
                while operator_a_gt_b(stack.last(), op)
                {
                    if let Some(result) = stack.pop()
                    {
                        queue.push_back(result);
                    }
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

pub fn operator_a_gt_b(a: Option<&Token>, b: char) -> bool
{
    if let Some(token) = a
    {
        if let Token::Operator(c) = token
        {
            return utility::get_operator_worth(*c) > utility::get_operator_worth(b)
        }
        else
        {
            return false;
        }
    }
    else
    {
        return false;
    }
}
