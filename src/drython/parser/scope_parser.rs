use crate::drython::utility::CheckOption;
use crate::drython::utility;

pub fn parse_scope(expression: &str) -> Result<(Option<String>, Option<String>), String>
{
    let exp;

    if let Some(result) = expression.split_once(")")
    {
        exp = result.1;
    }
    else {return Ok((None, None));}

    // Loop
    if let CheckOption::Split(result) = 
        utility::ordered_strings_check(exp, &["loop", ":"], true)
    {
        return handle_scope_result(&result);
    }
    // If
    if let CheckOption::Split(result) = 
        utility::ordered_strings_check(exp, &["if", ":"], true)
    {
        return handle_scope_result(&result);
    }
    // Function
    if let CheckOption::Split(result) = 
        utility::ordered_chars_check(exp, &[b'(', b')', b':'], true)
    {
        println!("{:?}", result);
        return handle_scope_result(&result);
    }

    Ok((None, None))
}

fn handle_scope_result(result: &Vec<&str>) -> Result<(Option<String>, Option<String>), String>
{
    let mut scope: Option<String> = None;
    let mut arguments: Option<String> = None;

    if result.len() >= 1 && !result[0].is_empty()
    {
        scope = Some(result[0].to_string());

        if result.len() == 2 && !result[1].is_empty()
        {
            arguments = Some(result[1].to_string());
        }
    }
    else
    {
        return Ok((None, None));
    }

    Ok((scope, arguments))
}