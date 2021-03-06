pub fn get_expression_type(string: &str) -> ExpressionType
{
    let mut buffer = String::new();
    let mut started_call_or_function = false;

    match string.to_lowercase().as_str()
    {
        "break" => {return ExpressionType::Break;}
        "continue" => {return ExpressionType::Continue;}
        _ => ()
    };

    // Comments
    if string.starts_with("//") || string.starts_with("#")
    {
        return ExpressionType::None;
    }

    for c in string.chars()
    {
        if !started_call_or_function
        {
            match c
            {
                c if c.is_alphanumeric() || c == '.' => buffer.push(c),
                '=' => return ExpressionType::Assignment,
                '(' => started_call_or_function = true,
                _ => () //Invalid char.
            }
        }
        // Check for the end of a function creation.
        else
        {
            match c
            {
                ':' => return ExpressionType::Function,
                _ => ()
            }
        }

        // Buffer check for the start of an expression type.
        // Later parsing will check for errors in missing colons or arguments.
        match buffer.to_lowercase().as_str()
        {
            "loop" => return ExpressionType::Loop,
            "if" => return ExpressionType::If,
            "elif"|"elseif" => return ExpressionType::Elif,
            "else" => return ExpressionType::Else,
            "return" => return ExpressionType::Return,
            "use"|"import"|"include"|"using" => return ExpressionType::Library,
            _ => ()
        }
    }

    // Found a call but not a function.
    // (Char search closed out before finding a colon.)
    if started_call_or_function
    {
        return ExpressionType::Call;
    }

    ExpressionType::None
}

#[derive(PartialEq, Debug)]
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
}

pub fn expression_is_scope(string: &str) -> bool
{
    match get_expression_type(string)
    {
        ExpressionType::Function => true,
        ExpressionType::If => true,
        ExpressionType::Loop => true,
        _ => false
    }
}

// Allows for splitting operations by comma.
// Avoids internal calls and scopes.
pub fn split_by(string: &str, split: char) -> Vec<String>
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
        //TODO: Throw an error.
    }

    result
}

pub const OPERATIONS: [&str; 14] = [
    "^", "/", "*", "%", "+", "-",
    "&&", "||",
    ">", "<", "<=", ">=", "==", "!="
];

/// Determines whether the passed char can be found within the allowed operations array.
/// Short-circuiting function.
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