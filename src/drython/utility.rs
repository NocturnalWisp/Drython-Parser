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

    for c in string.chars()
    {
        if !started_call_or_function
        {
            match c
            {
                c if c.is_alphanumeric() => buffer.push(c),
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
            "elif" => return ExpressionType::Elif,
            "elseif" => return ExpressionType::Elif,
            "else" => return ExpressionType::Else,
            "return" => return ExpressionType::Return,
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

#[derive(PartialEq)]
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

pub const OPERATIONS: [&str; 18] = [
    "^", "/", "*", "%", "+", "-", "++", "--", "**", "//",
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