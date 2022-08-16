use crate::drython::utility::ExpressionType;

pub fn parse_scope(exp: &str, expression_type: &ExpressionType) -> Result<(Option<String>, Option<String>), String>
{
    if exp.chars().last().unwrap() != ':'
    {
        return Err(format!("{:?} expressions need to end with a ':'", expression_type));
    }

    match expression_type
    {
        // Loop
        &ExpressionType::Loop =>
        {
            let result = exp.trim_start_matches("loop").trim_end_matches(":");

            Ok(handle_scope_result(vec!["loop", result]))
        },
        // If
        &ExpressionType::If =>
        {
            let result = exp.trim_start_matches("if").trim_end_matches(":");

            Ok(handle_scope_result(vec!["if", result]))
        },
        // Elif
        &ExpressionType::Elif =>
        {
            let result = exp.trim_start_matches("elif").trim_start_matches("elseif").trim_end_matches(":");

            Ok(handle_scope_result(vec!["elif", result]))
        },
        // Else
        &ExpressionType::Else =>
        {
            Ok(handle_scope_result(vec!["else", ""]))
        },
        // Function
        &ExpressionType::Function =>
        {
            let result_op = exp.split_once('(');

            if let Some(result) = result_op
            {
                Ok(handle_scope_result(vec![result.0, result.1.trim_end_matches("):")]))
            }
            else
            {
                Err("Failed to parse function.".to_string())
            }
        }
        _ => Ok((None, None))
    }
}

fn handle_scope_result(result: Vec<&str>) -> (Option<String>, Option<String>)
{
    if result.len() >= 1 && !result[0].is_empty()
    {
        let scope = Some(result[0].to_string());
        let mut arguments: Option<String> = None;

        if result.len() == 2 && !result[1].is_empty()
        {
            arguments = Some(result[1].to_string());
        }

        (scope, arguments)
    }
    else
    {
        return (None, None);
    }
}
