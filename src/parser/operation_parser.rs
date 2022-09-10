use std::collections::{VecDeque, HashMap};

use crate::types::Token;
use crate::utility;

// Used internally for parsing tokens from an operation.
// (Without the values associated with Types::Token)
#[derive(PartialEq, Clone, Debug)]
enum ParseTokenType
{
    None,
    Value,
    StringLiteral,
    CharLiteral,
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
            else if c == '"' { ParseTokenType::StringLiteral }
            else if c == '\'' { ParseTokenType::CharLiteral }
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
                    (PTT::Value, PTT::StringLiteral) |
                    (PTT::Value, PTT::CharLiteral) =>
                        {return Err("Unexpected literal after value type.".to_string());},
                    (PTT::Value, PTT::Collection) =>
                        {return Err("Unexpected collection after value type.".to_string());},
                    (PTT::Value, PTT::Operator) => false,
                    (PTT::Value, PTT::Parenth) => false,
                    (PTT::Value, PTT::Accessor) =>
                        if string[token_start..token_end].chars().all(|c| c.is_numeric()) { true } else { false },

                    (PTT::Operator, PTT::Value) => false,
                    (PTT::Operator, PTT::StringLiteral) => false,
                    (PTT::Operator, PTT::CharLiteral) => false,
                    (PTT::Operator, PTT::Collection) => false,
                    (PTT::Operator, PTT::Parenth) => false,
                    
                    (PTT::StringLiteral, PTT::StringLiteral) => false,
                    (PTT::CharLiteral, PTT::CharLiteral) => false,

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
            let is_char = last_token_type == ParseTokenType::CharLiteral;
            let used_accessor = current_char_type == ParseTokenType::Accessor;

            if (last_token_type == ParseTokenType::Value && current_char_type != ParseTokenType::Value) ||
               is_literal || is_char
            {
                // If current is parenth and this is not a string literal, it's the start of a call.
                if current_char_type == ParseTokenType::Parenth &&
                    last_token_type != ParseTokenType::StringLiteral &&
                    last_token_type != ParseTokenType::CharLiteral &&
                    last_token_type != ParseTokenType::Collection
                {
                    // Start call but don't reset token_start.
                    last_token_type = ParseTokenType::Call;
                }
                // Otherwise it's just a value.
                else
                {
                    let value = &string[token_start..token_end];
                    // Make sure if char, it is only one character.
                    if is_char
                    {
                        if value.len() < 1
                        {
                            return Err(format!("Tried to create a char type with no characters."));
                        }
                        else if value.len() > 1
                        {
                            return Err(format!("Too many characters to create a valid char type."));
                        }
                    }

                    let token = parse_token_value(&string[token_start..token_end], is_literal, is_char);
                    // Check for accessor on current char or after in the case of a string literal.
                    if (used_accessor || (is_literal && i+1 <string.len() && &string[i+1..i+2] == ".")) && !is_char 
                    {
                        current_accessor_token = token;
                        
                        last_token_type = ParseTokenType::Accessor;
                        token_start = i+1;
                        token_end = i+1;

                        if is_literal || is_char
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

                    if is_literal || is_char || used_accessor
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
                                        Ok(mut op) =>
                                        {
                                            match op.len()
                                            {
                                                0 => collection_operations.push(Token::Null),
                                                // Pop and Unwrap allowed because of our knowledge of a
                                                // single element present.
                                                1 => collection_operations.push(op.pop().unwrap()),
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
                                _ => { token = handle_accessor(current_accessor_token.clone(), Box::new(token)); }
                            }

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

                let token = handle_accessor(current_accessor_token.clone(), Box::new(parse_token_value(&string[token_start..token_end], is_literal, is_char)));

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
                    let mut token = parse_token_value(&string[token_start..i+1], false, false);

                    if let Token::Null = current_accessor_token {}
                    else
                    {
                        token = handle_accessor(current_accessor_token.clone(), Box::new(token));
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
            else if current_char_type == ParseTokenType::CharLiteral
            {
                last_token_type = ParseTokenType::CharLiteral;

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

fn handle_accessor(prev_token: Token, new_token: Box<Token>) -> Token
{
    if let Token::Accessor(prev, accessor) = prev_token
    {
        Token::Accessor(prev, Box::new(handle_accessor(*accessor, new_token)))
    }
    else
    {
        Token::Accessor(Box::new(prev_token), new_token)
    }
}

// Allows for the conversion from string to different types.
fn parse_token_value<'a>(value: &'a str, literal: bool, is_char: bool) -> Token
{
    if literal
    {
        return Token::String(value.to_string());
    }

    if is_char
    {
        return Token::Char(value.chars().next().unwrap());
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
fn handle_populating_operation(mut tokens: Vec<Token>) -> Vec<Token>
{

    let mut map: HashMap<usize, Token> = HashMap::with_capacity(tokens.len());

    for index in 0..tokens.len()
    {
        map.insert(index, tokens.pop().unwrap());
    }

    let mut stack: Vec<usize> = Vec::with_capacity(tokens.len());
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(tokens.len());

    for index in (0..map.len()).rev()
    {
        println!("{:?}", map[&index]);
        if let Token::Operator(op) = &map[&index]
        {
            if let Some(i) = stack.last().copied()
            {
                if let Token::Operator(mut stack_top) = map[&i].clone()
                {
                    while operator_a_gte_b(&stack_top, &op)
                    {
                        if let Some(result) = stack.pop()
                        {
                            queue.push_back(result);
                        }
                        if let Some(i) = stack.last().copied() {
                        if let Token::Operator(stack_top2) = &map[&i]
                        { stack_top = stack_top2.clone(); }
                        else { break; }}
                        else { break; }
                    }
                }
            }
            stack.push(index);
        }
        else
        {
            queue.push_back(index);
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
    // Without cloning, does allocate map.
    for token_index in queue.iter().rev()
    {
        tokens.push(map.remove(&token_index).unwrap());
    }
    
    tokens
}

// Check if operator worth is greater than or equal to another.
// Equal to allows for processing left most operators first that share the same value as another.
// (Eg. x*2/7: x*2 should go first, than divide that by 7.)
pub fn operator_a_gte_b(a: &str, b: &str) -> bool
{
    return utility::get_operator_worth(a) >= utility::get_operator_worth(b)
}
