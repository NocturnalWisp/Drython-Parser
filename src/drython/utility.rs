use std::vec;

// Inserts line numbers at the beginning of each line. Uses .lines() (seperated by \n)
pub fn insert_line_numbers(string: String) -> Vec<String>
{
    let mut new_strings: Vec<String> = vec![];

    for (index, line) in string.lines().enumerate()
    {
        new_strings.push(format!("{}){}", index+1, line));
    }

    new_strings
}

pub enum CheckOption<'a>
{
    Bool(bool),
    Split(Vec<&'a str>)
}

//NOTE: chars and sprites are seperate because char is slightly cheaper.
// Use string if u8's are not good enough.
// Checks for the appropriate characters (And makes sure they are in order.)
pub fn ordered_chars_check<'a>(string: &'a str, chars: &[u8], split: bool) -> CheckOption<'a>
{
    let mut char_index = 0;

    let string_chars = string.as_bytes();

    let mut i = 0;
    let mut after_last_found = 0;

    let mut split_result: Vec<&str> = vec![];

    while i < string_chars.len()
    {
        if string_chars[i] == chars[char_index]
        {
            // Add to split if enabled
            if split && (after_last_found <= i) && !&string[after_last_found..i].is_empty()
            {
                split_result.push(&string[after_last_found..i]);
            }

            char_index += 1;

            // Only used to track the last split.
            after_last_found = i+1;

            if char_index == chars.len()
            {
                break;
            }
        }

        i += 1;
    }

    if split { if char_index >= chars.len() {
        return CheckOption::Split(split_result);
    }}
    // If didn't capture all passed chars or failed to convert to utf8.
    // Or not split
    // Just return bool.
    CheckOption::Bool(char_index >= chars.len())
}

// Check for the appropriate strings within a string.
// (And make sure they are in the order given.)
pub fn ordered_strings_check<'a>(string: &'a str, strings: &[&str], split: bool) -> CheckOption<'a>
{
    let mut string_index: usize = 0;

    let string_chars = string.as_bytes();

    let mut i = 0;
    let mut after_last_found = 0;

    let mut split_result: Vec<&str> = vec![];

    while i < string_chars.len()
    {
        let find_string_chars = strings[string_index].as_bytes();

        // Break so as to not go out of bounds.
        if i + find_string_chars.len() > string_chars.len()
        {
            break;
        }

        // Check if all the current string in the string list matches
        // this and the following chars.
        let mut all_found = true;

        for (find_i, find_c) in find_string_chars.iter().enumerate()
        {
            // Check this element against the string list ascending.
            if *find_c != string_chars[i+find_i]
            {
                all_found = false;
            }
        }

        // Found substring in string!
        if all_found
        {
            // Add to split if enabled
            if split && (after_last_found < i) && !&string[after_last_found..i].is_empty()
            {
                split_result.push(&string[after_last_found..i]);
            }

            // Fast forward the loop till after the current string.
            i += find_string_chars.len();

            // Only used to track the last split.
            after_last_found = i;

            // Go to next string.
            string_index += 1;

            if string_index == strings.len()
            {
                break;
            }
        }
        else
        {
            i += 1
        }
    }

    if split { if string_index >= strings.len()
    {
        return CheckOption::Split(split_result);
    }}
    // If didn't capture all passed chars or failed to convert to utf8.
    // Or not split
    // Just return bool.
    CheckOption::Bool(string_index >= strings.len())
}

// The following are quick calls for finding certain items within strings
pub fn check_for_scope(string: &str) -> bool
{
    check_for_func(string) || check_for_if(string) || check_for_loop(string)
}

pub fn check_for_func(string: &str) -> bool
{
    if let CheckOption::Bool(result) = 
        ordered_chars_check(string, &[b'(', b')', b':'], false)
    {
        return result;
    }else {false}
}

pub fn check_for_if(string: &str) -> bool
{
    if let CheckOption::Bool(result) = 
        ordered_strings_check(string, &["if", ":"], false)
    {
        return result;
    }else {false}
}

pub fn check_for_loop(string: &str) -> bool
{
    if let CheckOption::Bool(result) = 
        ordered_strings_check(string, &["loop", ":"], false)
    {
        return result;
    }else {false}
}

// The following are quick calls for finding often used parses.
pub fn check_for_call(string: &str) -> bool
{
    if let CheckOption::Bool(result) = 
        ordered_chars_check(string, &[b'(', b')'], false)
    {
        return result;
    }else {false}
}

pub fn check_for_return(string: &str) -> bool
{
    string.to_lowercase().starts_with("return")
}