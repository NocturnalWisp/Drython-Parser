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
                c if c.is_alphanumeric() || c == '.' || c == '_' => buffer.push(c),
                '=' => return Ok(ExpressionType::Assignment),
                '(' => started_call_or_function = true,
                c if operations_contains(c) => (),
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

    Err("Unkown expression.".to_string())
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionType
{
    None,
    Assignment,
    Function,
    Call,
    Return,
    If,
    Elif,
    Else,
    Loop,
    Break,
    Continue,
    Library,
    Comment,
    End,
}

pub fn expression_is_scope(expression_type: &ExpressionType) -> bool
{
    match expression_type
    {
        ExpressionType::Function => true,
        ExpressionType::If => true,
        ExpressionType::Loop => true,
        _ => false
    }
}

// Allows for splitting operations by comma.
// Avoids internal calls and scopes.
pub fn split_by(string: &str, split: char) -> Result<Vec<String>, String>
{
    let mut result: Vec<String> = Vec::new();

    let mut prev_start = 0;
    let mut in_count = 0;

    for (i, c) in string.chars().enumerate()
    {
        if (c == split && in_count == 0) || i == string.len()-1
        {
            let end = if i != string.len() - 1 {i} else {i+1};
            result.push(string[prev_start..end].to_string());
            prev_start = i+1;
        }

        if c == '(' || c == '['
        {
            in_count += 1;
        }
        else if c == ')' || c == ']'
        {
            in_count -= 1;
        }

        // If in count is ever less than 0, there is a bracket in excess.
        if in_count < 0
        {
            return Err(format!("Excess '{}' found. Make sure to properly enclose your statements.", c));
        }
    }

    // Too few brackets found.
    if in_count > 0
    {
        return Err(format!("Too few brackets/parenthises found. Make sure to enclose your expressions correctly."));
    }

    Ok(result)
}

pub const OPERATIONS: [&str; 14] = [
    "^", "/", "*", "%", "+", "-",
    "&&", "||",
    ">", "<", "<=", ">=", "==", "!="
];

// Determines whether the passed char can be found within the allowed operations array.
pub fn operations_contains(c: char) -> bool
{
    return OPERATIONS.iter().any(|x| x.contains(c));
}

pub fn get_operator_worth(string: &str) -> u8
{
    match string
    {
        "+"|"-" => 0,
        "*"|"/"|"%" => 1,
        "^" => 2,
        "||" => 3,
        "&&" => 4,
        ">"|"<"|"<="|">="|"=="|"!=" => 5,
        _ => 0
    }
}
