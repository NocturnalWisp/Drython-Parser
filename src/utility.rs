use std::fmt::Arguments;
use std::fmt::Write;

// Inserts line numbers at the beginning of each line. Uses .lines() (seperated by \n)
pub fn insert_line_numbers(string: String) -> String
{
    let mut new_string: Vec<String> = vec![];
    for (index, line) in string.lines().enumerate()
    {
        new_string.push(format!("{}){}", index+1, line));
    }

    new_string.join(";")
}

// Checks for the appropriate characters (And makes sure they are in order.)
pub fn ordered_chars_check(string: &str, chars: &[char]) -> bool
{
    let mut char_index = 0;

    for char in string.chars()
    {
        if char == chars[char_index]
        {
            char_index += 1;

            if char_index == chars.len()
            {
                break;
            }
        }
    }

    char_index >= chars.len()
}

// Check for the appropriate strings within a string.
// (And make sure they are in the order given.)
pub fn ordered_strings_check(string: &str, strings: &[&str]) -> bool
{
    let mut string_index: usize = 0;

    let string_chars = string.as_bytes();

    let mut i = 0;
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

        if all_found
        {
            // Fast forward the loop till after the current string.
            i += find_string_chars.len();

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

    string_index >= strings.len()
}

// The following are quick calls for finding certain items within strings
pub fn check_for_if(string: &str) -> bool
{
    ordered_strings_check(string, &["if", ":"])
}

// The following are quick calls for finding often used parses.
pub fn check_for_call(string: &str) -> bool
{
    ordered_chars_check(string, &['(', ')'])
}