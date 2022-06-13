#[path="./../utility.rs"]
pub mod utility;

use std::fmt::Write;

pub fn check_var(var: (& String, & String), _var_list: &Vec<String>, _func_list: &Vec<String>) -> Result<(), String>
{
    let initial_msg = "Failed to parse variable:\n";

    let mut error_msg = String::new();

    if var.0.is_empty()
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable name is empty.").ok();
        return Err(error_msg);
    }

    if var.1.is_empty()
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable value is empty.").ok();
        return Err(error_msg);
    }

    if !var.0.chars().all(char::is_alphanumeric)
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable name contains special characters.").ok();
        return Err(error_msg);
    }

    let special_chars = ['+', '-', '*', '/', '%', '|', '&', '^'];

    if !var.1.chars().all(|x| x.is_alphanumeric() 
        || special_chars.contains(&x))
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable value contains special characters.").ok();
        return Err(error_msg);
    }

    Ok(())
}

