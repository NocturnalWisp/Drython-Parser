pub fn parse_var(line: &str) -> Result<(Vec<String>, String, String), String>
{
    let mut var: (Vec<String>, String, String) = (vec![], "".to_string(), "".to_string());

    let mut identifier_split: Vec<String> = line.split("!").map(|x| x.to_string()).collect();

    if identifier_split.len() == 0
    {
        return Err("Failed to parse variable assignement. Expected a single '='.".to_string());
    }

    let var_unsplit = identifier_split.pop().unwrap();
    let split: Vec<&str> = var_unsplit.split("=").collect();

    var.0 = identifier_split;

    if split.len() !=2
    {
        // Handle ++ and --
        if line.ends_with("++")
        {
            let temp = line.trim_end_matches("++");
            var.1 = temp.to_string();
            var.2 = format!("{}+{}", temp, 1);
        }
        else if line.ends_with("--")
        {
            let temp = line.trim_end_matches("--");
            var.1 = temp.to_string();
            var.2 = format!("{}-{}", temp, 1);
        }
        else
        {
            return Err("Failed to parse variable assignment. Expected a single '='.".to_string());
        }
    }
    else
    {
        var.1 = split[0].to_string();
        var.2 = split[1].to_string();

        // Check for assignment operatives.
        let mut operator: Option<char> = None;
        if var.1.ends_with('+') { operator = Some('+'); }
        else if var.1.ends_with('-') { operator = Some('-'); }
        else if var.1.ends_with('*') { operator = Some('*'); }
        else if var.1.ends_with('/') { operator = Some('/'); }

        if let Some(operator) = operator
        {
            var.1 = var.1.trim_end_matches(operator).to_string();
            var.2 = format!("{}{}{}", var.1, operator, var.2);
        }
    }

    Ok(var)
}
