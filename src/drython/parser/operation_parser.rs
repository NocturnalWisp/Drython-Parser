use std::{collections::VecDeque};

use linked_hash_map::LinkedHashMap;

use crate::drython::types::Token;

use crate::drython::utility;

// Used internally for parsing tokens from an operation.
// (Without the values associated with Types::Token)
#[derive(PartialEq, Clone)]
enum ParseTokenType
{
    None,
    Value,
    Operator,
    Parenth,
    //Function Call
    Call
}

// Hybrid polish notation/ast tree. Internal operations (Expressed in parentheses)
// are put into a recursive calculation.
pub fn parse_operation<'a>(string: & str, warning_list: &mut LinkedHashMap<usize, String>) -> Vec<Token>
{
    let mut last_token_type = ParseTokenType::None;
    let mut token_start: usize = 0;
    let mut token_end: usize = 0;
    
    let mut inner_parenth_count: i8 = 0;

    // optional char is for the char operation leading to the next option.
    let mut tokens: Vec<Token> = Vec::new();
    let tokens_ptr = &mut tokens;

    // Resurive check for parentheses
    for (i, c) in string.chars().enumerate()
    {
        let current_char_type =
        {
            if c.is_alphanumeric() || c == '.' { ParseTokenType::Value }
            else if utility::operations_contains(c) { ParseTokenType::Operator }
            else if c == '(' || c == ')' { ParseTokenType::Parenth }
            else { ParseTokenType::None }
        };

        if last_token_type != ParseTokenType::None
        {
            // Continue to punt token_end until we've found a different type of token.
            if (current_char_type == last_token_type ||
                last_token_type == ParseTokenType::Parenth || last_token_type == ParseTokenType::Call)
                    && current_char_type != ParseTokenType::Parenth && i != string.len()-1
            {
                token_end = i+1;
                continue;
            }
            // The following else ifs' handle the last token value base on what kind of change happened.

            // Finish off value.
            if last_token_type == ParseTokenType::Value && current_char_type != ParseTokenType::Value
            {
                // If current is parenth, it's the start of a call.
                if current_char_type == ParseTokenType::Parenth
                {
                    // Start call but don't reset token_start.
                    last_token_type = ParseTokenType::Call;
                }
                // Otherwise it's just a value.
                else
                {
                    tokens_ptr.push(parse_token_value(&string[token_start..token_end]));

                    last_token_type = ParseTokenType::None;
                    token_start = i;
                }
            }
            // Finish off operator.
            else if last_token_type == ParseTokenType::Operator && current_char_type != ParseTokenType::Operator
            {
                let found_operator = &string[token_start..token_end];
                if utility::OPERATIONS.iter().any(|x| x == &found_operator)
                {
                    tokens_ptr.push(Token::Operator(found_operator.to_string()));
                }
                else
                {
                    //TODO: Throw error that the operator doesn't exist.
                }

                last_token_type = ParseTokenType::None;
                token_start = i;
            }
            // Handle an internal operation using recursion on the current function.
            else if last_token_type == ParseTokenType::Parenth
            {
                // Found an inner-inner operation.
                if c == '('
                {
                    inner_parenth_count += 1;
                }
                else if c == ')'
                {
                    if inner_parenth_count == 0
                    {
                        tokens_ptr.push(Token::Operation(parse_operation(&string[token_start..token_end], warning_list)));

                        last_token_type = ParseTokenType::None;
                        token_start = i;
                    }
                    else
                    {
                        inner_parenth_count -= 1;
                    }
                }
            }
            else if last_token_type == ParseTokenType::Call
            {
                // Found an inner-call operation.
                if c == '('
                {
                    inner_parenth_count += 1;
                }
                else if c == ')'
                {
                    if inner_parenth_count == 0
                    {
                        // Split call into function and arguments.
                        let call = &string[token_start..i];

                        if let Some(result) = call.split_once("(")
                        {
                            tokens_ptr.push(Token::Call(result.0.to_string(), result.1[..result.1.len()].to_string()));
                        }

                        last_token_type = ParseTokenType::None;
                        token_start = i;
                    }
                    else
                    {
                        inner_parenth_count -= 1;
                    }
                }
            }
        }

        if last_token_type == ParseTokenType::None || i == string.len()-1
        {
            if current_char_type == ParseTokenType::Value
            {
                last_token_type = ParseTokenType::Value;

                // This might be the last value.
                if i == string.len()-1
                {
                    tokens_ptr.push(parse_token_value(&string[token_start..i+1]));
                }

                token_start = i;
                token_end = i+1;
            }
            else if current_char_type == ParseTokenType::Operator
            {
                last_token_type = ParseTokenType::Operator;
                token_start = i;
                token_end = i+1;
            }
            else if c == '('
            {
                last_token_type = ParseTokenType::Parenth;
                token_start = i+1;
                token_end = i+2;
            }
        }
    }

    // If still in parenthises, there is an error.
    if inner_parenth_count > 0
    {
        //TODO: warning_list.push("Failed to parse operation.\nClosing parentheses not found.");
    }
    
    // Handle operation order and populating.
    handle_populating_operation(tokens)
    
}

// Allows for the conversion from string to different types.
fn parse_token_value<'a>(value: &'a str) -> Token
{
    if let Ok(result) = value.parse::<i32>()
    {
        Token::Int(result)
    }
    else if let Ok(result) = value.parse::<f32>()
    {
        Token::Float(result)
    }
    else if let Ok(result) = value.parse::<bool>()
    {
        Token::Bool(result)
    }
    // Parsed tokens that don't fit another type are considered variables.
    else
    {
        Token::Var(value.to_string())
    }
}

// Handles creating the various types of operations to be merged into one big operation later.
// Uses the shunting algorithm based on the reverse polish technique.
// (Doesn't deal with brackets because Operations all ready nest themselves.)
// Followed: https://brilliant.org/wiki/shunting-yard-algorithm/
fn handle_populating_operation(tokens: Vec<Token>) -> Vec<Token>
{
    let mut stack: Vec<&Token> = Vec::new();
    let mut queue: VecDeque<&Token> = VecDeque::new();

    let mut final_tokens: Vec<Token> = Vec::new();

    for token in tokens.iter()
    {
        if let Token::Operator(op) = token
        {
            while operator_a_gte_b(stack.last(), &op)
            {
                if let Some(result) = stack.pop()
                {
                    queue.push_back(result);
                }
            }
            stack.push(token);
        }
        else
        {
            queue.push_back(token);
        }
    }

    // Put everything left from the stack onto the queue.
    while !stack.is_empty()
    {
        if let Some(result) = stack.pop()
        {
            queue.push_back(result);
        }
    }
    
    // Finalize and move everything to the final tokens vector.
    while !queue.is_empty()
    {
        if let Some(result) = queue.pop_back()
        {
            final_tokens.push(result.clone());
        }
    }
    
    final_tokens
}

// Check if operator worth is greater than or equal to another.
// Equal to allows for processing left most operators first that share the same value as another.
// (Eg. x*2/7: x*2 should go first, than divide that by 7.)
pub fn operator_a_gte_b(a: Option<&&Token>, b: &str) -> bool
{
    if let Some(token) = a
    {
        if let Token::Operator(str) = token
        {
            return utility::get_operator_worth(str) >= utility::get_operator_worth(b)
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
