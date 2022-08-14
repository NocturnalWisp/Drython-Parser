use crate::drython::utility::{get_expression_type, ExpressionType};

use crate::drython::types::error::*;

pub fn parse_scope(expression: &str, line_number: usize, error_manager: &mut ErrorManager) -> Result<(Option<String>, Option<String>), String>
{
    let exp;

    if let Some(result) = expression.split_once(")")
    {
        exp = result.1;
    }
    else {return Ok((None, None));}

    let expression_type_result = get_expression_type(exp);
    let expression_type = expression_type_result.clone().unwrap_or(ExpressionType::None);

    if let Err(message) = &expression_type_result
    {
        push_error!(error_manager, ParseError::new(line_number, message.as_str()));
        return Err(message.to_string());
    }

    // Loop
    if expression_type == ExpressionType::Loop
    {
        let result = exp.trim_start_matches("loop").trim_end_matches(":");

        return handle_scope_result(vec!["loop", result]);
    }
    // If
    if expression_type == ExpressionType::If
    {
        let result = exp.trim_start_matches("if").trim_end_matches(":");

        return handle_scope_result(vec!["if", result]);
    }
    // Elif
    if expression_type == ExpressionType::Elif
    {
        let result = exp.trim_start_matches("elif").trim_start_matches("elseif").trim_end_matches(":");

        return handle_scope_result(vec!["elif", result]);
    }
    // Else
    if expression_type == ExpressionType::Else
    {
        return handle_scope_result(vec!["else", ""]);
    }

    // Function
    if expression_type == ExpressionType::Function
    {
        let result_op = exp.split_once('(');

        if let Some(result) = result_op
        {
            return handle_scope_result(vec![result.0, result.1.trim_end_matches("):")]);
        }
    }

    Ok((None, None))
}

fn handle_scope_result(result: Vec<&str>) -> Result<(Option<String>, Option<String>), String>
{
    let mut _scope: Option<String> = None;
    let mut arguments: Option<String> = None;

    if result.len() >= 1 && !result[0].is_empty()
    {
        _scope = Some(result[0].to_string());

        if result.len() == 2 && !result[1].is_empty()
        {
            arguments = Some(result[1].to_string());
        }
    }
    else
    {
        return Ok((None, None));
    }

    Ok((_scope, arguments))
}
