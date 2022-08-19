use std::collections::VecDeque;

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

use ParseTokenType as PTT;

// Hybrid polish notation/ast tree. Internal operations (Expressed in parentheses)
// are put into a recursive calculation.
pub fn parse_operation<'a>(string: & str) -> Result<Vec<Token>, String>
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

    // Allows for skipping parsing of the next character in the loop.
    // Currently used by string literals if they have an accessor emmediately after.
    let mut skip_once = false;

    // Resurive check for parentheses
    for (i, c) in string.chars().enumerate()
    {
        if skip_once
        {
            skip_once = false;
            continue;
        }

        let current_char_type =
        {
            if c.is_alphanumeric() || c == '_' { ParseTokenType::Value }
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
                    (PTT::Value, PTT::StringLiteral) =>
                        {return Err("Unexpected string literal after value type.".to_string());},
                    (PTT::Value, PTT::Collection) =>
                        {return Err("Unexpected collection after value type.".to_string());},
                    (PTT::Value, PTT::Operator) => false,
                    (PTT::Value, PTT::Parenth) => false,
                    (PTT::Value, PTT::Accessor) =>
                        if string[token_start..token_end].chars().all(|c| c.is_numeric()) { true } else { false },

                    (PTT::Operator, PTT::Value) => false,
                    (PTT::Operator, PTT::StringLiteral) => false,
                    (PTT::Operator, PTT::Collection) => false,
                    (PTT::Operator, PTT::Parenth) => false,
                    
                    (PTT::StringLiteral, PTT::StringLiteral) => false,

                    (PTT::Collection, PTT::Collection) => false,

                    (PTT::Parenth, PTT::Parenth) => false,

                    (PTT::Call, PTT::Parenth) => false,

                    (PTT::Accessor, PTT::Accessor) => false,
                    (PTT::Accessor, PTT::Collection) => false,
                    (PTT::Accessor, PTT::Operator) => false,
                    (PTT::Accessor, PTT::Value) => false,
                    (PTT::Accessor, PTT::Parenth) => false,

                    _ => true
                };

                if skip_current
                {
                    token_end = i+1;
                    continue;
                }
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
                    // Check for accessor on current char or after in the case of a string literal.
                    if used_accessor || (is_literal && i+1 <string.len() && &string[i+1..i+2] == ".")
                    {
                        current_accessor_token = token;
                        
                        last_token_type = ParseTokenType::Accessor;
                        token_start = i+1;
                        token_end = i+1;

                        if is_literal
                        {
                            token_start += 1;
                            token_end += 1;
                            skip_once = true;
                        }
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
                    return Err(format!("{} Operation was not recognized.", found_operator));
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
                        match utility::split_by(&string[token_start..token_end], ',')
                        {
                            Ok(result) =>
                            {
                                let mut collection_operations: Vec<Token> = Vec::new();
                                for collection_operation in result.iter()
                                {
                                    let operation = parse_operation(&collection_operation);

                                    match operation
                                    {
                                        Ok(op) =>
                                        {
                                            match op.len()
                                            {
                                                0 => collection_operations.push(Token::Null),
                                                1 => collection_operations.push(op[0].clone()),
                                                _ => collection_operations.push(Token::Operation(op))
                                            }
                                        },
                                        Err(error) =>
                                        {
                                            return Err(error);
                                        }
                                    }
                                }
                                 
                                let token = Token::Collection(collection_operations);
                                // Check if using an accessor after this call.
                                if string.len() > i+1 && &string[i+1..i+2] == "."
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
                            }
                            Err(error) =>
                            {
                                return Err(error);
                            }
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
                        match parse_operation(&string[token_start..token_end])
                        {
                            Ok(op) =>
                            {
                                let token = Token::Operation(op);
                                
                                tokens_ptr.push(token);

                                last_token_type = ParseTokenType::None;
                                token_start = i;
                            },
                            Err(error) =>
                            {
                                return Err(error);
                            }
                        }
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
        return Err("Parenthesis were not closed during operation.".to_string());
    }
    else if inner_parenth_count < 0
    {
        return Err("Too many closing parenthesis found.".to_string());
    }
    
    // Handle operation order and populating.
    Ok(handle_populating_operation(tokens))
    
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
