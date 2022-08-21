use crate::utility;

// Parses a function call into the name of the function, and the arguments.
pub fn parse_call(call: &str) -> Result<(String, Vec<String>), String>
{
    let function: String;
    let arguments: Vec<String>;
    
    if let Some(first) = call.split_once("(")
    {
        if first.1.len() > 0 && first.1.as_bytes()[first.1.len()-1] == b')'
        {
            function = first.0.to_string();
            
            match utility::split_by(&first.1[0..first.1.len()-1], ',')
            {
                Ok(result) =>
                {
                    arguments = result;
                }
                Err(error) =>
                {
                    return Err(error);
                }
            }
        }
        else
        {
            return Err("No closing parenthesis found on function call.".to_string());
        }
    }
    else
    {
        return Err("Failed to parse call.\nInvalid function.".to_string());
    }

    Ok((function, arguments))
}
