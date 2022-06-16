
use std::fmt::Write;

use crate::drython::types::RegisteredList;
use crate::drython::utility::OPERATIONS;

pub fn check_var<'a>(var: (&'a str, &'a str), registered: &mut RegisteredList<'a>)
{
    let initial_msg = "Failed to parse variable:\n";

    let mut error_msg = String::new();

    if var.0.is_empty()
    {
        registered.error_list.push(format!("{}{}", initial_msg, "The variable name is empty."));
    }

    if var.1.is_empty()
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable value is empty.").ok();
        registered.error_list.push(error_msg.clone());
    }

    if !var.0.chars().all(char::is_alphanumeric)
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable name contains special characters.").ok();
        registered.error_list.push(error_msg.clone());
    }

    if !var.1.chars().all(|x| x.is_alphanumeric() 
        || OPERATIONS.contains(&x))
    {
        write!(&mut error_msg, "{}{}", initial_msg, "The variable value contains special characters.").ok();
        registered.error_list.push(error_msg.clone());
    }
    
    registered.var_list.push(var.0);
}

