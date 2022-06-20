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

// pub enum CheckOption<'a>
// {
//     Bool(bool),
//     Split(Vec<&'a str>)
// }

// //NOTE: chars and sprites are seperate because char is slightly cheaper.
// // Use string if u8's are not good enough.
// // Checks for the appropriate characters (And makes sure they are in order.)
// pub fn ordered_chars_check<'a>(string: &'a str, chars: &[u8], split: bool) -> CheckOption<'a>
// {
//     let mut char_index = 0;

//     let string_chars = string.as_bytes();

//     let mut after_last_found = 0;

//     let mut split_result: Vec<&str> = Vec::new();

//     for i in 0..string_chars.len()
//     {
//         if string_chars[i] == chars[char_index]
//         {
//             // Add to split if enabled
//             if split && (after_last_found <= i) && !&string[after_last_found..i].is_empty()
//             {
//                 split_result.push(&string[after_last_found..i]);
//             }

//             char_index += 1;

//             // Only used to track the last split.
//             after_last_found = i+1;

//             if char_index == chars.len()
//             {
//                 break;
//             }
//         }
//     }

//     if split { if char_index >= chars.len() {
//         return CheckOption::Split(split_result);
//     }}
//     // If didn't capture all passed chars or failed to convert to utf8.
//     // Or not split
//     // Just return bool.
//     CheckOption::Bool(char_index >= chars.len())
// }

// // Check for the appropriate strings within a string.
// // (And make sure they are in the order given.)
// pub fn ordered_strings_check<'a>(string: &'a str, strings: &[&str], split: bool) -> CheckOption<'a>
// {
//     let mut string_index: usize = 0;

//     let string_chars = string.as_bytes();

//     let mut i = 0;
//     let mut after_last_found = 0;

//     let mut split_result: Vec<&str> = Vec::new();

//     while i < string_chars.len()
//     {
//         let find_string_chars = strings[string_index].as_bytes();

//         // Break so as to not go out of bounds.
//         if i + find_string_chars.len() > string_chars.len()
//         {
//             break;
//         }

//         // Check if all the current string in the string list matches
//         // this and the following chars.
//         let mut all_found = true;

//         for (find_i, find_c) in find_string_chars.iter().enumerate()
//         {
//             // Check this element against the string list ascending.
//             if *find_c != string_chars[i+find_i]
//             {
//                 all_found = false;
//             }
//         }

//         // Found substring in string!
//         if all_found
//         {
//             // Add to split if enabled
//             if split && (after_last_found < i) && !&string[after_last_found..i].is_empty()
//             {
//                 split_result.push(&string[after_last_found..i]);
//             }

//             // Fast forward the loop till after the current string.
//             i += find_string_chars.len();

//             // Only used to track the last split.
//             after_last_found = i;

//             // Go to next string.
//             string_index += 1;

//             if string_index == strings.len()
//             {
//                 break;
//             }
//         }
//         else
//         {
//             i += 1
//         }
//     }

//     if split { if string_index >= strings.len()
//     {
//         return CheckOption::Split(split_result);
//     }}
//     // If didn't capture all passed chars or failed to convert to utf8.
//     // Or not split
//     // Just return bool.
//     CheckOption::Bool(string_index >= strings.len())
// }

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

pub const OPERATIONS: [&str; 10] = [
    "^", "/", "*", "%", "+", "-",
    "&&", "||", "&", "|",
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
        "||"|"&" => 3,
        "&&"|"|" => 4,
        _ => 0
    }
}

// // The following are quick calls for finding certain items within strings
// pub fn check_for_scope(string: &str) -> bool
// {
//     check_for_func(string) || check_for_if(string) || check_for_loop(string)
// }

// pub fn check_for_func(string: &str) -> bool
// {
//     if let CheckOption::Bool(result) = 
//         ordered_chars_check(string, &[b'(', b')', b':'], false)
//     {
//         return result;
//     }else {false}
// }

// pub fn check_for_if(string: &str) -> bool
// {
//     if let CheckOption::Bool(result) = 
//         ordered_strings_check(string, &["if", ":"], false)
//     {
//         return result;
//     }else {false}
// }

// pub fn check_for_loop(string: &str) -> bool
// {
//     if let CheckOption::Bool(result) = 
//         ordered_strings_check(string, &["loop", ":"], false)
//     {
//         return result;
//     }else {false}
// }

// // The following are quick calls for finding often used parses.
// pub fn check_for_call(string: &str) -> bool
// {
//     if let CheckOption::Bool(result) = 
//         ordered_chars_check(string, &[b'(', b')'], false)
//     {
//         return result;
//     }else {false}
// }

// pub fn check_for_return(string: &str) -> bool
// {
//     string.to_lowercase().starts_with("return")
// }