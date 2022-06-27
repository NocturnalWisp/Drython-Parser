// Inserts line numbers at the beginning of each line. Uses .lines() (seperated by \n)
pub fn insert_line_numbers(string: String) -> Vec<String>
{
    let mut new_strings: Vec<String> = Vec::new();

    for (index, line) in string.lines().enumerate()
    {
        new_strings.push(format!("{}){}", index+1, line));
    }

    new_strings
}

pub fn get_expression_type(string: &str) -> ExpressionType
{
    let mut buffer = String::new();
    let mut started_call_or_function = false;

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
            "return" => return ExpressionType::Return,
            "while" => return ExpressionType::While,
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
    Loop,
    While,
}

pub fn expression_is_scope(string: &str) -> bool
{
    match get_expression_type(string)
    {
        ExpressionType::Function => true,
        ExpressionType::If => true,
        ExpressionType::Loop => true,
        ExpressionType::While => true,
        _ => false
    }
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