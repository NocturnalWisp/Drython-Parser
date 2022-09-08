pub fn parse_var(line: &str) -> Result<(String, String), String>
{
    let mut var: (String, String) = ("".to_string(), "".to_string());
    
    let split: Vec<&str> = line.split("=").collect();

    if split.len() !=2
    {
        return Err("Failed to parse variable assignment. Expected a single '='.".to_string());
    }

    var.0 = split[0].to_string();
    var.1 = split[1].to_string();

    // Check for assignment operatives.
    let mut operator: Option<char> = None;
    if var.0.ends_with('+') { operator = Some('+'); }
    else if var.0.ends_with('-') { operator = Some('-'); }
    else if var.0.ends_with('*') { operator = Some('*'); }
    else if var.0.ends_with('/') { operator = Some('/'); }

    if let Some(operator) = operator
    {
        var.0 = var.0.trim_end_matches(operator).to_string();
        var.1 = format!("{}{}{}", var.0, operator, var.1);
    }

    Ok(var)
}
