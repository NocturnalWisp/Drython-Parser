use std::{collections::VecDeque};

use linked_hash_map::LinkedHashMap;

use crate::drython::types::Token;

use crate::drython::utility;

// Used internally for parsing tokens from an operation.
// (Without the values associated with Types::Token)
#[derive(PartialEq, Clone, Debug)]
enum ParseTokenType
{
    None,
    Value,
    StringLiteral,
    Collection,

    Operator,
    Parenth,
    //Function Call
    Call,
    Accessor
}

// Hybrid polish notation/ast tree. Internal operations (Expressed in parentheses)
// are put into a recursive calculation.
pub fn parse_operation<'a>(string: & str, warning_list: &mut LinkedHashMap<usize, String>) -> Vec<Token>
{
    let mut last_token_type = ParseTokenType::None;
    let mut token_start: usize = 0;
    let mut token_end: usize = 0;
    
    let mut inner_parenth_count: i8 = 0;
    let mut inner_collection_count: i8 = 0;

    // optional char is for the char operation leading to the next option.
    let mut tokens: Vec<Token> = Vec::new();
    let tokens_ptr = &mut tokens;

    // Holds the token attached to an accessor.
    let mut current_accessor_token = Token::Null;

    // Resurive check for parentheses
    for (i, c) in string.chars().enumerate()
    {
        let current_char_type =
        {
            if c.is_alphanumeric() { ParseTokenType::Value }
            else if c == '\'' || c == '"' { ParseTokenType::StringLiteral }
            else if utility::operations_contains(c) { ParseTokenType::Operator }
            else if c == '(' || c == ')' { ParseTokenType::Parenth }
            else if c == '[' || c == ']' { ParseTokenType::Collection }
            else if c == '.' { ParseTokenType::Accessor }
            else { ParseTokenType::None }
        };

        if last_token_type != ParseTokenType::None
        {
            // Determine if the for loop should continue to the next character.
            // Current is still apart of the last type.
            if i != string.len()-1
            {
                // Enum values not present in match are skipped.
                // Follow the "a encounters b" pattern.
                let skip_current = match (&last_token_type, &current_char_type)
                {
                    (ParseTokenType::Value, ParseTokenType::StringLiteral) => false,
                    (ParseTokenType::Value, ParseTokenType::Collection) => false,
                    (ParseTokenType::Value, ParseTokenType::Operator) => false,
                    (ParseTokenType::Value, ParseTokenType::Parenth) => false,

                    (ParseTokenType::Operator, ParseTokenType::Value) => false,
                    (ParseTokenType::Operator, ParseTokenType::StringLiteral) => false,
                    (ParseTokenType::Operator, ParseTokenType::Collection) => false,
                    (ParseTokenType::Operator, ParseTokenType::Parenth) => false,
                    
                    (ParseTokenType::StringLiteral, ParseTokenType::StringLiteral) => false,
                    (ParseTokenType::Parenth, ParseTokenType::Parenth) => false,
                    (ParseTokenType::Call, ParseTokenType::Parenth) => false,
                    (ParseTokenType::Collection, ParseTokenType::Collection) => false,

                    (ParseTokenType::Value, ParseTokenType::Accessor) => false,
                    (ParseTokenType::StringLiteral, ParseTokenType::Accessor) => false,
                    (ParseTokenType::Collection, ParseTokenType::Accessor) => false,

                    (ParseTokenType::Accessor, ParseTokenType::Accessor) => false,
                    (ParseTokenType::Accessor, ParseTokenType::Collection) => false,
                    (ParseTokenType::Accessor, ParseTokenType::Operator) => false,
                    (ParseTokenType::Accessor, ParseTokenType::Value) => false,
                    (ParseTokenType::Accessor, ParseTokenType::Parenth) => false,

                    _ => true
                };

                if skip_current
                {
                    token_end = i+1;
                    continue;
                }
            }
            else
            {
                ()
            }
            // The following else ifs' handle the last token value base on what kind of change happened.

            // Finish off value or string literal.
            let is_literal = last_token_type == ParseTokenType::StringLiteral;
            let used_accessor = current_char_type == ParseTokenType::Accessor;

            if (last_token_type == ParseTokenType::Value && current_char_type != ParseTokenType::Value) ||
               is_literal
            {
                // If current is parenth and this is not a string literal, it's the start of a call.
                if current_char_type == ParseTokenType::Parenth &&
                    last_token_type != ParseTokenType::StringLiteral && last_token_type != ParseTokenType::Collection
                {
                    // Start call but don't reset token_start.
                    last_token_type = ParseTokenType::Call;
                }
                // Otherwise it's just a value.
                else
                {
                    let token = parse_token_value(&string[token_start..token_end], is_literal);
                    if used_accessor
                    {
                        current_accessor_token = token;
                        
                        last_token_type = ParseTokenType::Accessor;
                        token_start = i+1;
                        token_end = i+1;
                    }
                    else
                    {
                        tokens_ptr.push(token);

                        last_token_type = ParseTokenType::None;
                        token_start = i;
                    }

                    if is_literal || used_accessor
                    {
                        continue;
                    }
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
            // Finish off collection.
            else if last_token_type == ParseTokenType::Collection
            {
                if c == '['
                {
                    inner_collection_count += 1;
                }
                else if c == ']'
                {
                    if inner_collection_count == 0
                    {
                        let collection_operations = utility::split_by(&string[token_start..token_end], ',').iter()
                            // Use map to make sure multi-operations are kept in a Token::Operation.
                            .map(|x| {
                                let operation = parse_operation(x, warning_list);

                                match operation.len()
                                {
                                    0 => Token::Null,
                                    1 => operation[0].clone(),
                                    _ => Token::Operation(operation)
                                }
                            }).collect::<Vec<Token>>();
                        
                        let token = Token::Collection(collection_operations);
                        // Check if using an accessor after this call.
                        if string.len() >= i+2 && &string[i+1..i+2] == "."
                        {
                            current_accessor_token = token;
                            
                            last_token_type = ParseTokenType::None;
                            token_start = i+2;
                            token_end = i+3;
                        }
                        else
                        {
                            tokens_ptr.push(token);
    
                            last_token_type = ParseTokenType::None;
                            token_start = i;
                        }

                        continue;
                    }
                    else
                    {
                        inner_collection_count -= 1;
                    }
                }
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
                        let token = Token::Operation(parse_operation(&string[token_start..token_end], warning_list));
                        
                        tokens_ptr.push(token);

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
                            let mut token = Token::Call(result.0.to_string(), result.1[..result.1.len()].to_string());

                            // Handle accessor.
                            match &current_accessor_token
                            {
                                Token::Null => (),
                                _ => { token = handle_accessor(&current_accessor_token, Box::new(token)); }
                            }

                            // Check if using an accessor after this call.
                            if string.len() >= i+2 && &string[i+1..i+2] == "."
                            {
                                current_accessor_token = token.clone();
                                
                                last_token_type = ParseTokenType::None;
                                token_start = i+2;
                                token_end = i+3;
                            }
                            else
                            {
                                tokens_ptr.push(token);
        
                                last_token_type = ParseTokenType::None;
                                token_start = i;
                            }
                        }
                    }
                    else
                    {
                        inner_parenth_count -= 1;
                    }
                }
            }
            else if last_token_type == ParseTokenType::Accessor
            {
                let mut was_last = false;

                // Load accessor with previously grabbed token into the tokens_ptr.
                if i == string.len()-1
                {
                    token_end += 1;
                    was_last = true;
                }

                let token = handle_accessor(&current_accessor_token, Box::new(parse_token_value(&string[token_start..token_end], is_literal)));

                if current_char_type == ParseTokenType::Accessor
                {
                    // Continue to be an accessor, just recursive with the last accessor.
                    current_accessor_token = token;
                    token_start = i+1;
                    token_end = i+1;

                    continue;
                }
                // Just continue through when the current is just another value.
                else if current_char_type == ParseTokenType::Value
                {
                    token_end = i+1;
                }
                else if current_char_type == ParseTokenType::Parenth
                {
                    last_token_type = ParseTokenType::Call;
                }
                else
                {
                    tokens_ptr.push(token);
                    current_accessor_token = Token::Null;
    
                    last_token_type = ParseTokenType::None;
                    token_start = i;

                    if was_last
                    {
                        continue;
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
                    let mut token = parse_token_value(&string[token_start..i+1], false);

                    if let Token::Null = current_accessor_token {}
                    else
                    {
                        token = handle_accessor(&current_accessor_token, Box::new(token));
                    }
                    tokens_ptr.push(token);
                }

                token_start = i;
                token_end = i+1;
            }
            else if current_char_type == ParseTokenType::Collection
            {
                last_token_type = ParseTokenType::Collection;

                token_start = i+1;
                token_end = i+2;
            }
            else if current_char_type == ParseTokenType::StringLiteral
            {
                last_token_type = ParseTokenType::StringLiteral;

                token_start = i+1;
                token_end = i+2;
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
            else if current_char_type == ParseTokenType::Accessor
            {
                last_token_type = ParseTokenType::Accessor;
                token_start = i+1;
                token_end = i+1;
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

fn handle_accessor(prev_token: &Token, new_token: Box<Token>) -> Token
{
    if let Token::Accessor(prev, accessor) = &*prev_token
    {
        Token::Accessor(prev.clone(), Box::new(handle_accessor(&*accessor, new_token)))
    }
    else
    {
        Token::Accessor(Box::new(prev_token.clone()), new_token)
    }
}

// Allows for the conversion from string to different types.
fn parse_token_value<'a>(value: &'a str, literal: bool) -> Token
{
    if literal
    {
        return Token::String(value.to_string());
    }

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
