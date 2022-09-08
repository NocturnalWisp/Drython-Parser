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

pub const OPERATIONS: [&str; 18] = [
    "^", "/", "*", "%", "+", "-", "+=", "-=", "*=", "/=",
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
        "+"|"-"|"+="|"-=" => 0,
        "*"|"/"|"%"|"*/"|"/=" => 1,
        "^" => 2,
        "||" => 3,
        "&&" => 4,
        ">"|"<"|"<="|">="|"=="|"!=" => 5,
        _ => 0
    }
}
